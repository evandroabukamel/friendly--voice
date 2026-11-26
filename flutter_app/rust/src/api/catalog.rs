//! Options shown by the UI: themes (with their initial tone), tones and the
//! languages a person can speak. All come from the domain, so the app never
//! hard-codes a list.
use friendly_voice_domain::{
    language::SUPPORTED,
    message::MessageCategory,
    tone::{ToneStyle, MAX_PITCH_SHIFT_SEMITONES},
};

pub struct Catalog {
    pub themes: Vec<ThemeOption>,
    pub tones: Vec<ToneOption>,
    pub languages: Vec<LanguageOption>,
    pub max_pitch_shift: f32,
}

pub struct ThemeOption {
    pub slug: String,
    /// Tone used when the user does not pick one.
    pub default_tone: String,
}

pub struct ToneOption {
    pub slug: String,
    pub description: String,
}

pub struct LanguageOption {
    pub code: String,
    pub name: String,
}

#[flutter_rust_bridge::frb(sync)]
pub fn catalog() -> Catalog {
    Catalog {
        themes: MessageCategory::ALL
            .iter()
            .map(|c| ThemeOption {
                slug: c.slug().into(),
                default_tone: ToneStyle::default_for(Some(*c)).slug().into(),
            })
            .collect(),
        tones: ToneStyle::ALL
            .iter()
            .map(|t| ToneOption {
                slug: t.slug().into(),
                description: t.description().into(),
            })
            .collect(),
        languages: SUPPORTED
            .iter()
            .map(|(code, name)| LanguageOption {
                code: (*code).into(),
                name: (*name).into(),
            })
            .collect(),
        max_pitch_shift: MAX_PITCH_SHIFT_SEMITONES,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lists_themes_with_their_initial_tone() {
        let c = catalog();
        let calm = c.themes.iter().find(|t| t.slug == "calm").unwrap();
        assert_eq!(calm.default_tone, "calm");
        assert_eq!(c.themes.len(), 6);
    }

    #[test]
    fn lists_tones_languages_and_the_pitch_limit() {
        let c = catalog();
        let slugs: Vec<_> = c.tones.iter().map(|t| t.slug.as_str()).collect();
        assert_eq!(
            slugs,
            ["natural", "warm", "cheerful", "calm", "firm", "whisper"]
        );
        assert!(c.tones.iter().all(|t| !t.description.is_empty()));
        assert!(c.languages.iter().any(|l| l.code == "pt"));
        assert_eq!(c.max_pitch_shift, 6.0);
    }
}
