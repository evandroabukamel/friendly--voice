//! Local engine, with no network and no neural network.
//!
//! * [`LocalVoiceBank`] registers the person's references (the real timbre
//!   needs a neural engine — see `HttpVoiceEngine`).
//! * [`ProsodySketchSynthesizer`] renders a "sound sketch" of the text: each
//!   syllable (group of vowel letters) becomes a voiced pulse at the person's
//!   mean pitch and rhythm, the next-to-last syllable of a word is longer and
//!   higher, sibilant letters become noise and punctuation becomes a pause.
//!   It validates rhythm/intonation and the end-to-end pipeline.
use crate::audio::encode_wav;
use friendly_voice_application::ports::{
    PortError, PortResult, ReferenceAudio, SpeechSynthesizer, SynthesizedAudio, VoiceCloningEngine,
};
use friendly_voice_domain::{
    audio::AudioClip,
    speaker::Speaker,
    voice::{estimate_syllables, SpeechScript, VoiceGender, VoiceProfile},
};
use std::f32::consts::TAU;

#[derive(Debug, Clone, Copy, Default)]
pub struct LocalVoiceBank;

impl VoiceCloningEngine for LocalVoiceBank {
    fn enroll(
        &self,
        speaker: &Speaker,
        _gender: Option<VoiceGender>,
        references: &[ReferenceAudio],
    ) -> PortResult<String> {
        if references.is_empty() {
            return Err(PortError::new("no reference audio"));
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

/// Sound source derived from the chosen tone.
#[derive(Debug, Clone, Copy)]
struct Voicing {
    gain: f32,
    breathy: bool,
}

/// White noise in [-0.5, 0.5].
fn xorshift(seed: &mut u32) -> f32 {
    *seed ^= *seed << 13;
    *seed ^= *seed >> 17;
    *seed ^= *seed << 5;
    *seed as f32 / u32::MAX as f32 - 0.5
}

fn sibilants(word: &str) -> usize {
    word.chars()
        .filter(|c| {
            matches!(
                c.to_ascii_lowercase(),
                's' | 'z' | 'f' | 'x' | 'j' | 'v' | 'ç'
            )
        })
        .count()
}

/// Words with the punctuation that follows them.
fn words(text: &str) -> impl Iterator<Item = (&str, bool)> {
    text.split_whitespace().map(|w| {
        let pause = w.ends_with(|c: char| ".,;:!?…—()".contains(c));
        (w, pause)
    })
}

impl ProsodySketchSynthesizer {
    #[allow(clippy::too_many_arguments)]
    fn tone(
        &self,
        out: &mut Vec<f32>,
        secs: f32,
        f0_start: f32,
        f0_end: f32,
        phase: &mut f32,
        voice: Voicing,
        seed: &mut u32,
    ) {
        let n = (secs * self.sample_rate as f32) as usize;
        let attack = (n / 8).max(1);
        for i in 0..n {
            let t = i as f32 / n.max(1) as f32;
            let f0 = f0_start + (f0_end - f0_start) * t;
            *phase += TAU * f0 / self.sample_rate as f32;
            // Approximate glottal source: decaying harmonics; for a breathy
            // voice (whisper) the source is noise modulated by the pitch.
            let harmonics: f32 = (1..=6).map(|k| (*phase * k as f32).sin() / k as f32).sum();
            let v = if voice.breathy {
                xorshift(seed) * 3.0 * (0.6 + 0.4 * harmonics.abs().min(1.0))
            } else {
                harmonics
            };
            let env = if i < attack {
                i as f32 / attack as f32
            } else {
                1.0 - (t - 0.125).max(0.0) * 0.9
            };
            out.push(0.25 * voice.gain * v * env);
        }
    }

    fn noise(&self, out: &mut Vec<f32>, secs: f32, seed: &mut u32, gain: f32) {
        let n = (secs * self.sample_rate as f32) as usize;
        for _ in 0..n {
            out.push(xorshift(seed) * 0.16 * gain);
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
        let total = estimate_syllables(&script.text).max(1) as f32;
        let adj = script.tone.adjustment();
        let voice = Voicing {
            gain: 10f32.powf(adj.volume_db / 20.0),
            breathy: adj.breathy,
        };
        let (mut out, mut phase, mut seed, mut k) = (Vec::new(), 0.0f32, 0x9E37_79B9u32, 0.0f32);
        for (word, pause_after) in words(&script.text) {
            let syllables = estimate_syllables(word);
            let stressed = syllables.saturating_sub(2).min(syllables.saturating_sub(1));
            for i in 0..syllables {
                // Declination: the sentence pitch "falls" from start to end.
                let decl = base + spread * 0.5 * (1.0 - 2.0 * k / total);
                let (dur, f0) = if i == stressed {
                    (syll * 1.35, decl + spread * 0.4)
                } else {
                    (syll * 0.85, decl)
                };
                self.tone(
                    &mut out,
                    dur * 0.8,
                    f0,
                    f0 * 0.97,
                    &mut phase,
                    voice,
                    &mut seed,
                );
                k += 1.0;
            }
            for _ in 0..sibilants(word) {
                self.noise(&mut out, syll * 0.3, &mut seed, voice.gain);
            }
            self.silence(&mut out, if pause_after { 0.35 } else { 0.03 });
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
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use friendly_voice_domain::tone::{ToneStyle, VoiceTone};
    use friendly_voice_domain::voice::ProsodyProfile;

    fn script(rate: f32, style: ToneStyle) -> SpeechScript {
        SpeechScript {
            text: "Você consegue, meu bem.".into(),
            language: Default::default(),
            prosody: ProsodyProfile {
                mean_pitch_hz: 200.0,
                syllables_per_second: rate,
                ..Default::default()
            },
            tone: VoiceTone::of(style),
        }
    }

    #[test]
    fn faster_speaker_produces_shorter_audio() {
        let s = ProsodySketchSynthesizer::default();
        assert!(
            s.render(&script(7.0, ToneStyle::Natural)).duration_secs()
                < s.render(&script(4.0, ToneStyle::Natural)).duration_secs()
        );
    }

    #[test]
    fn whisper_is_quieter_than_firm() {
        let s = ProsodySketchSynthesizer::default();
        assert!(
            s.render(&script(5.0, ToneStyle::Whisper)).rms()
                < s.render(&script(5.0, ToneStyle::Firm)).rms() / 3.0
        );
    }
}
