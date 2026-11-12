//! Renders SSML with the (IPA) pronunciation of each word, accent already applied.
use friendly_voice_domain::voice::SpeechScript;

fn escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

/// Rate relative to average pt-BR speech (~5.5 syllables/s).
fn rate_percent(syllables_per_second: f32) -> u32 {
    if syllables_per_second <= 0.0 {
        return 100;
    }
    ((syllables_per_second / 5.5) * 100.0)
        .clamp(60.0, 160.0)
        .round() as u32
}

pub fn to_ssml(script: &SpeechScript) -> String {
    let p = &script.prosody;
    let mut body = String::new();
    for (i, w) in script.pronunciation.words.iter().enumerate() {
        if i > 0 {
            body.push(' ');
        }
        body.push_str(&format!(
            "<phoneme alphabet=\"ipa\" ph=\"{}\">{}</phoneme>",
            escape(&w.ipa()),
            escape(&w.orthography)
        ));
        if w.pause_after {
            body.push_str("<break time=\"300ms\"/>");
        }
    }
    let pitch = if p.mean_pitch_hz > 0.0 {
        format!(" pitch=\"{:.0}Hz\"", p.mean_pitch_hz)
    } else {
        String::new()
    };
    let tone = script.tone.adjustment();
    let volume = if tone.volume_db != 0.0 {
        format!(" volume=\"{:+.0}dB\"", tone.volume_db)
    } else {
        String::new()
    };
    format!(
        "<speak version=\"1.1\" xml:lang=\"pt-BR\"><!-- tom: {} --><prosody rate=\"{}%\"{pitch}{volume}>{body}</prosody></speak>",
        script.tone,
        rate_percent(p.syllables_per_second)
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use friendly_voice_domain::phonetics::{parse_ipa, PhoneticUtterance, PhoneticWord};
    use friendly_voice_domain::tone::{ToneStyle, VoiceTone};
    use friendly_voice_domain::voice::ProsodyProfile;

    #[test]
    fn renders_phonemes() {
        let script = SpeechScript {
            text: "noite".into(),
            pronunciation: PhoneticUtterance::new(vec![PhoneticWord::new(
                "noite",
                parse_ipa("nojtʃi"),
                Some(1),
            )]),
            prosody: ProsodyProfile {
                mean_pitch_hz: 210.0,
                syllables_per_second: 5.5,
                ..Default::default()
            },
            tone: VoiceTone::of(ToneStyle::Calm),
        };
        let s = to_ssml(&script);
        assert!(s.contains("<phoneme alphabet=\"ipa\" ph=\"nojtʃi\">noite</phoneme>"));
        assert!(s.contains("rate=\"100%\"") && s.contains("pitch=\"210Hz\""));
        assert!(
            s.contains("volume=\"-3dB\"") && s.contains("tom: calmo"),
            "{s}"
        );
    }
}
