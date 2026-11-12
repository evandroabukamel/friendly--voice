//! Local delivery: saves the message to an "outbox" and, optionally,
//! plays the audio with a system player (e.g. `aplay`, `afplay`, `ffplay`).
use friendly_voice_application::ports::{Delivery, MessageDelivery, PortError, PortResult};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub struct OutboxDelivery {
    dir: PathBuf,
    player: Option<String>,
}

impl OutboxDelivery {
    pub fn new(dir: impl Into<PathBuf>, player: Option<String>) -> Self {
        Self {
            dir: dir.into(),
            player,
        }
    }
}

impl MessageDelivery for OutboxDelivery {
    fn deliver(&self, d: &Delivery<'_>) -> PortResult<String> {
        fs::create_dir_all(&self.dir).map_err(PortError::new)?;
        let stem = format!("{}-{}", d.at.format("%Y%m%d-%H%M%S"), d.speaker.id);
        let wav = self.dir.join(format!("{stem}.wav"));
        fs::write(&wav, &d.audio.wav).map_err(PortError::new)?;
        let note = format!(
            "From: {}\nWhen: {}\nTone: {}\n\n{}\n\nPronunciation (IPA): {}\n\nMarkup sent to the engine:\n{}\n",
            d.speaker.name,
            d.at.format("%Y-%m-%d %H:%M"),
            d.script.tone,
            d.message.text(),
            d.script.pronunciation.ipa(),
            d.audio.markup
        );
        fs::write(self.dir.join(format!("{stem}.txt")), note).map_err(PortError::new)?;
        if let Some(player) = &self.player {
            let mut parts = player.split_whitespace();
            let program = parts
                .next()
                .ok_or_else(|| PortError::new("empty player command"))?;
            Command::new(program)
                .args(parts)
                .arg(&wav)
                .status()
                .map_err(|e| PortError::new(format!("could not play with '{player}': {e}")))?;
        }
        Ok(wav.display().to_string())
    }
}
