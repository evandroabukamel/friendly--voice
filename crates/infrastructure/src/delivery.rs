//! Local delivery: saves the message to an "outbox" and, optionally,
//! plays the audio with a system player (e.g. `aplay`, `afplay`, `ffplay`,
//! or `windows` for the player built into Windows).
use friendly_voice_application::ports::{Delivery, MessageDelivery, PortError, PortResult};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Player name that selects the WAV player built into Windows
/// (`System.Media.SoundPlayer` through PowerShell), so nothing has to be installed.
const WINDOWS_PLAYER: &str = "windows";

/// Builds the command that plays `wav`: the configured command line with the
/// file appended, or PowerShell's `SoundPlayer` for [`WINDOWS_PLAYER`].
fn player_command(player: &str, wav: &Path) -> PortResult<Command> {
    if player.trim().eq_ignore_ascii_case(WINDOWS_PLAYER) {
        // Single quotes keep PowerShell from expanding the path; a quote
        // inside it is escaped by doubling.
        let path = wav.display().to_string().replace('\'', "''");
        let mut cmd = Command::new("powershell");
        cmd.args([
            "-NoProfile",
            "-NonInteractive",
            "-Command",
            &format!("(New-Object System.Media.SoundPlayer '{path}').PlaySync()"),
        ]);
        return Ok(cmd);
    }
    let mut parts = player.split_whitespace();
    let program = parts
        .next()
        .ok_or_else(|| PortError::new("empty player command"))?;
    let mut cmd = Command::new(program);
    cmd.args(parts).arg(wav);
    Ok(cmd)
}

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
            "From: {}\nWhen: {}\nLanguage: {}\nTone: {}\n\n{}\n",
            d.speaker.name,
            d.at.format("%Y-%m-%d %H:%M"),
            d.script.language,
            d.script.tone,
            d.message.text(),
        );
        fs::write(self.dir.join(format!("{stem}.txt")), note).map_err(PortError::new)?;
        if let Some(player) = &self.player {
            player_command(player, &wav)?
                .status()
                .map_err(|e| PortError::new(format!("could not play with '{player}': {e}")))?;
        }
        Ok(wav.display().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(cmd: &Command) -> Vec<String> {
        cmd.get_args()
            .map(|a| a.to_string_lossy().into_owned())
            .collect()
    }

    #[test]
    fn command_line_player_gets_the_file_appended() {
        let cmd = player_command("ffplay -nodisp -autoexit", Path::new("out/a.wav")).unwrap();
        assert_eq!(cmd.get_program(), "ffplay");
        assert_eq!(args(&cmd), ["-nodisp", "-autoexit", "out/a.wav"]);
    }

    #[test]
    fn windows_player_uses_powershell_sound_player() {
        let cmd = player_command("Windows", Path::new(r"C:\saida\it's.wav")).unwrap();
        assert_eq!(cmd.get_program(), "powershell");
        assert_eq!(
            args(&cmd).last().unwrap(),
            r"(New-Object System.Media.SoundPlayer 'C:\saida\it''s.wav').PlaySync()"
        );
    }

    #[test]
    fn empty_player_is_an_error() {
        assert!(player_command("  ", Path::new("a.wav")).is_err());
    }
}
