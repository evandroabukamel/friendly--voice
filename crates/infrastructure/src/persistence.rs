//! Persistence in JSON files (suitable for a personal/local app).
use friendly_voice_application::ports::*;
use friendly_voice_domain::{
    ids::{SampleId, ScheduleId, SpeakerId},
    schedule::Schedule,
    speaker::Speaker,
    voice::{VoiceProfile, VoiceSample},
};
use serde::{de::DeserializeOwned, Serialize};
use std::fs;
use std::marker::PhantomData;
use std::path::{Component, Path, PathBuf};
use std::sync::Mutex;

/// Collection of `T` stored in a JSON file.
pub struct JsonRepository<T> {
    file: PathBuf,
    lock: Mutex<()>,
    _t: PhantomData<T>,
}

impl<T: Serialize + DeserializeOwned + Clone> JsonRepository<T> {
    pub fn new(file: impl Into<PathBuf>) -> Self {
        Self {
            file: file.into(),
            lock: Mutex::new(()),
            _t: PhantomData,
        }
    }

    fn read(&self) -> PortResult<Vec<T>> {
        match fs::read(&self.file) {
            Ok(bytes) => serde_json::from_slice(&bytes)
                .map_err(|e| PortError::new(format!("{}: {e}", self.file.display()))),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Vec::new()),
            Err(e) => Err(PortError::new(e)),
        }
    }

    fn write(&self, items: &[T]) -> PortResult<()> {
        if let Some(dir) = self.file.parent() {
            fs::create_dir_all(dir).map_err(PortError::new)?;
        }
        let tmp = self.file.with_extension("json.tmp");
        let json = serde_json::to_vec_pretty(items).map_err(PortError::new)?;
        fs::write(&tmp, json).map_err(PortError::new)?;
        fs::rename(&tmp, &self.file).map_err(PortError::new)
    }

    fn upsert(&self, item: &T, same: impl Fn(&T) -> bool) -> PortResult<()> {
        let _g = self
            .lock
            .lock()
            .map_err(|_| PortError::new("lock envenenado"))?;
        let mut all = self.read()?;
        match all.iter().position(&same) {
            Some(i) => all[i] = item.clone(),
            None => all.push(item.clone()),
        }
        self.write(&all)
    }

    fn all(&self) -> PortResult<Vec<T>> {
        let _g = self
            .lock
            .lock()
            .map_err(|_| PortError::new("lock envenenado"))?;
        self.read()
    }
}

impl SpeakerRepository for JsonRepository<Speaker> {
    fn save(&self, s: &Speaker) -> PortResult<()> {
        self.upsert(s, |x| x.id == s.id)
    }
    fn find(&self, id: &SpeakerId) -> PortResult<Option<Speaker>> {
        Ok(self.all()?.into_iter().find(|s| &s.id == id))
    }
    fn list(&self) -> PortResult<Vec<Speaker>> {
        self.all()
    }
}

impl SampleRepository for JsonRepository<VoiceSample> {
    fn save(&self, s: &VoiceSample) -> PortResult<()> {
        self.upsert(s, |x| x.id == s.id)
    }
    fn list_by_speaker(&self, id: &SpeakerId) -> PortResult<Vec<VoiceSample>> {
        Ok(self
            .all()?
            .into_iter()
            .filter(|s| &s.speaker_id == id)
            .collect())
    }
}

impl VoiceProfileRepository for JsonRepository<VoiceProfile> {
    fn save(&self, p: &VoiceProfile) -> PortResult<()> {
        self.upsert(p, |x| x.speaker_id == p.speaker_id)
    }
    fn find(&self, id: &SpeakerId) -> PortResult<Option<VoiceProfile>> {
        Ok(self.all()?.into_iter().find(|p| &p.speaker_id == id))
    }
}

impl ScheduleRepository for JsonRepository<Schedule> {
    fn save(&self, s: &Schedule) -> PortResult<()> {
        self.upsert(s, |x| x.id == s.id)
    }
    fn list(&self) -> PortResult<Vec<Schedule>> {
        self.all()
    }
    fn delete(&self, id: &ScheduleId) -> PortResult<bool> {
        let _g = self
            .lock
            .lock()
            .map_err(|_| PortError::new("lock envenenado"))?;
        let mut all = self.read()?;
        let before = all.len();
        all.retain(|s| &s.id != id);
        self.write(&all)?;
        Ok(all.len() != before)
    }
}

/// Stores the audio files at `<root>/<speaker>/<sample>.wav`.
pub struct FileAudioStore {
    root: PathBuf,
}

impl FileAudioStore {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }
    fn safe(part: &str) -> PortResult<&str> {
        let ok = !part.is_empty()
            && Path::new(part)
                .components()
                .all(|c| matches!(c, Component::Normal(_)))
            && !part.contains(['/', '\\']);
        ok.then_some(part)
            .ok_or_else(|| PortError::new(format!("identificador inválido: {part}")))
    }
}

impl AudioStore for FileAudioStore {
    fn store(&self, speaker: &SpeakerId, sample: &SampleId, bytes: &[u8]) -> PortResult<String> {
        let rel = format!(
            "{}/{}.wav",
            Self::safe(speaker.as_str())?,
            Self::safe(sample.as_str())?
        );
        let path = self.root.join(&rel);
        fs::create_dir_all(path.parent().expect("has parent")).map_err(PortError::new)?;
        fs::write(&path, bytes).map_err(PortError::new)?;
        Ok(rel)
    }
    fn load(&self, locator: &str) -> PortResult<Vec<u8>> {
        if Path::new(locator)
            .components()
            .any(|c| !matches!(c, Component::Normal(_)))
        {
            return Err(PortError::new("localizador inválido"));
        }
        fs::read(self.root.join(locator)).map_err(PortError::new)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDateTime;
    use friendly_voice_domain::speaker::Consent;

    fn tmp(name: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("fv-test-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&d);
        d
    }

    #[test]
    fn speakers_roundtrip_and_upsert() {
        let dir = tmp("spk");
        let repo = JsonRepository::<Speaker>::new(dir.join("speakers.json"));
        let consent = Consent {
            granted_by: "A".into(),
            granted_at: NaiveDateTime::default(),
            statement: "".into(),
        };
        let mut s = Speaker::new(SpeakerId::new("1"), "Ana", None, Some(consent)).unwrap();
        repo.save(&s).unwrap();
        s.name = "Ana Maria".into();
        repo.save(&s).unwrap();
        assert_eq!(repo.list().unwrap(), vec![s.clone()]);
        assert_eq!(repo.find(&SpeakerId::new("1")).unwrap(), Some(s));
        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn audio_store_blocks_path_traversal() {
        let dir = tmp("audio");
        let store = FileAudioStore::new(&dir);
        let loc = store
            .store(&SpeakerId::new("s"), &SampleId::new("a"), b"x")
            .unwrap();
        assert_eq!(store.load(&loc).unwrap(), b"x");
        assert!(store.load("../etc/passwd").is_err());
        assert!(store
            .store(&SpeakerId::new(".."), &SampleId::new("a"), b"x")
            .is_err());
        fs::remove_dir_all(dir).unwrap();
    }
}
