//! Entrega local: salva a mensagem em uma "caixa de saída" e, opcionalmente,
//! toca o áudio com um player do sistema (ex.: `aplay`, `afplay`, `ffplay`).
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
            "De: {}\nQuando: {}\n\n{}\n\nPronúncia (IPA): {}\n\nMarcação enviada ao motor:\n{}\n",
            d.speaker.name,
            d.at.format("%d/%m/%Y %H:%M"),
            d.message.text(),
            d.script.pronunciation.ipa(),
            d.audio.markup
        );
        fs::write(self.dir.join(format!("{stem}.txt")), note).map_err(PortError::new)?;
        if let Some(player) = &self.player {
            let mut parts = player.split_whitespace();
            let program = parts.next().ok_or_else(|| PortError::new("player vazio"))?;
            Command::new(program)
                .args(parts)
                .arg(&wav)
                .status()
                .map_err(|e| {
                    PortError::new(format!("não foi possível tocar com '{player}': {e}"))
                })?;
        }
        Ok(wav.display().to_string())
    }
}
