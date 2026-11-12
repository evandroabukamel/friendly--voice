//! Motor local, sem rede e sem rede neural.
//!
//! * [`LocalVoiceBank`] registra as referências da pessoa (o timbre real
//!   exige um motor neural — ver `HttpVoiceEngine`).
//! * [`ProsodySketchSynthesizer`] gera um "esboço sonoro" da fala: cada
//!   sílaba vira um pulso vozeado no pitch médio da pessoa, no ritmo dela,
//!   com tônicas mais longas/agudas, ruído nas fricativas e pausas na
//!   pontuação. Serve para validar ritmo/entonação e o pipeline ponta a ponta.
use super::ssml::to_ssml;
use crate::audio::encode_wav;
use friendly_voice_application::ports::{
    PortError, PortResult, ReferenceAudio, SpeechSynthesizer, SynthesizedAudio, VoiceCloningEngine,
};
use friendly_voice_domain::{
    audio::AudioClip,
    phonetics::Phoneme,
    speaker::Speaker,
    voice::{SpeechScript, VoiceProfile},
};
use std::f32::consts::TAU;

#[derive(Debug, Clone, Copy, Default)]
pub struct LocalVoiceBank;

impl VoiceCloningEngine for LocalVoiceBank {
    fn enroll(&self, speaker: &Speaker, references: &[ReferenceAudio]) -> PortResult<String> {
        if references.is_empty() {
            return Err(PortError::new("nenhuma referência de áudio"));
        }
        Ok(format!("local:{}:{}", speaker.id, references.len()))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ProsodySketchSynthesizer {
    pub sample_rate: u32,
}

impl Default for ProsodySketchSynthesizer {
    fn default() -> Self {
        Self {
            sample_rate: 22_050,
        }
    }
}

fn is_fricative(p: &Phoneme) -> bool {
    matches!(
        p.symbol(),
        "s" | "ʃ" | "f" | "h" | "x" | "z" | "ʒ" | "v" | "tʃ" | "dʒ"
    )
}

impl ProsodySketchSynthesizer {
    fn tone(&self, out: &mut Vec<f32>, secs: f32, f0_start: f32, f0_end: f32, phase: &mut f32) {
        let n = (secs * self.sample_rate as f32) as usize;
        let attack = (n / 8).max(1);
        for i in 0..n {
            let t = i as f32 / n.max(1) as f32;
            let f0 = f0_start + (f0_end - f0_start) * t;
            *phase += TAU * f0 / self.sample_rate as f32;
            // Fonte glotal aproximada: harmônicos decrescentes.
            let v: f32 = (1..=6).map(|k| (*phase * k as f32).sin() / k as f32).sum();
            let env = if i < attack {
                i as f32 / attack as f32
            } else {
                1.0 - (t - 0.125).max(0.0) * 0.9
            };
            out.push(0.25 * v * env);
        }
    }

    fn noise(&self, out: &mut Vec<f32>, secs: f32, seed: &mut u32) {
        let n = (secs * self.sample_rate as f32) as usize;
        for _ in 0..n {
            *seed ^= *seed << 13;
            *seed ^= *seed >> 17;
            *seed ^= *seed << 5;
            out.push((*seed as f32 / u32::MAX as f32 - 0.5) * 0.08);
        }
    }

    fn silence(&self, out: &mut Vec<f32>, secs: f32) {
        out.extend(std::iter::repeat_n(
            0.0,
            (secs * self.sample_rate as f32) as usize,
        ));
    }

    pub fn render(&self, script: &SpeechScript) -> AudioClip {
        let p = script.prosody;
        let base = if p.mean_pitch_hz > 0.0 {
            p.mean_pitch_hz
        } else {
            180.0
        };
        let spread = if p.pitch_spread_hz > 0.0 {
            p.pitch_spread_hz
        } else {
            base * 0.2
        };
        let syll = if p.syllables_per_second > 0.0 {
            1.0 / p.syllables_per_second
        } else {
            0.18
        };
        let total = script.pronunciation.syllable_count().max(1) as f32;
        let (mut out, mut phase, mut seed, mut k) = (Vec::new(), 0.0f32, 0x9E37_79B9u32, 0.0f32);
        for word in &script.pronunciation.words {
            for (i, ph) in word.phonemes.iter().enumerate() {
                if ph.is_vowel() {
                    // Declinação: a frase "desce" do início ao fim.
                    let decl = base + spread * 0.5 * (1.0 - 2.0 * k / total);
                    let stressed = word.stress == Some(i);
                    let (dur, f0) = if stressed {
                        (syll * 1.35, decl + spread * 0.4)
                    } else {
                        (syll * 0.85, decl)
                    };
                    self.tone(&mut out, dur * 0.8, f0, f0 * 0.97, &mut phase);
                    k += 1.0;
                } else if is_fricative(ph) {
                    self.noise(&mut out, syll * 0.3, &mut seed);
                } else if ph.is_consonant() && !ph.is_voiced() {
                    self.silence(&mut out, syll * 0.12);
                }
            }
            self.silence(&mut out, if word.pause_after { 0.35 } else { 0.03 });
        }
        AudioClip::new(self.sample_rate, out).expect("sample rate > 0")
    }
}

impl SpeechSynthesizer for ProsodySketchSynthesizer {
    fn synthesize(
        &self,
        _voice: &VoiceProfile,
        script: &SpeechScript,
    ) -> PortResult<SynthesizedAudio> {
        Ok(SynthesizedAudio {
            wav: encode_wav(&self.render(script))?,
            markup: to_ssml(script),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use friendly_voice_domain::phonetics::{parse_ipa, PhoneticUtterance, PhoneticWord};
    use friendly_voice_domain::voice::ProsodyProfile;

    #[test]
    fn faster_speaker_produces_shorter_audio() {
        let utt = PhoneticUtterance::new(vec![PhoneticWord::new(
            "x",
            parse_ipa("vosekõsegi"),
            Some(1),
        )]);
        let script = |rate| SpeechScript {
            text: "x".into(),
            pronunciation: utt.clone(),
            prosody: ProsodyProfile {
                mean_pitch_hz: 200.0,
                syllables_per_second: rate,
                ..Default::default()
            },
        };
        let s = ProsodySketchSynthesizer::default();
        assert!(s.render(&script(7.0)).duration_secs() < s.render(&script(4.0)).duration_secs());
    }
}
