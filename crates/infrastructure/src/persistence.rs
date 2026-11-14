//! Persistence in JSON files (suitable for a personal/local app).
use crate::audio::AudioFormat;
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
            .map_err(|_| PortError::new("poisoned lock"))?;
        let mut all = self.read()?;
        match all.iter().position(&same) {
            Some(i) => all[i] = item.clone(),
            None => all.push(item.clone()),
        }
        self.write(&all)
    }

    /// Removes the matching items; returns how many. Writes only on change.
    fn remove_where(&self, matches: impl Fn(&T) -> bool) -> PortResult<usize> {
        let _g = self
            .lock
            .lock()
            .map_err(|_| PortError::new("poisoned lock"))?;
        let mut all = self.read()?;
        let before = all.len();
        all.retain(|x| !matches(x));
        let removed = before - all.len();
        if removed > 0 {
            self.write(&all)?;
        }
        Ok(removed)
    }

    fn all(&self) -> PortResult<Vec<T>> {
        let _g = self
            .lock
            .lock()
            .map_err(|_| PortError::new("poisoned lock"))?;
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
    fn delete_by_speaker(&self, id: &SpeakerId) -> PortResult<usize> {
        self.remove_where(|s| &s.speaker_id == id)
    }
}

impl VoiceProfileRepository for JsonRepository<VoiceProfile> {
    fn save(&self, p: &VoiceProfile) -> PortResult<()> {
        self.upsert(p, |x| x.speaker_id == p.speaker_id)
    }
    fn find(&self, id: &SpeakerId) -> PortResult<Option<VoiceProfile>> {
        Ok(self.all()?.into_iter().find(|p| &p.speaker_id == id))
    }
    fn delete(&self, id: &SpeakerId) -> PortResult<bool> {
        Ok(self.remove_where(|p| &p.speaker_id == id)? > 0)
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
        Ok(self.remove_where(|s| &s.id == id)? > 0)
    }
}

/// Stores the audio files at `<root>/<speaker>/<sample>.<wav|flac>`, keeping
/// the original bytes; the extension comes from the detected format.
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
            .ok_or_else(|| PortError::new(format!("invalid identifier: {part}")))
    }
}

impl AudioStore for FileAudioStore {
    fn store(&self, speaker: &SpeakerId, sample: &SampleId, bytes: &[u8]) -> PortResult<String> {
        let extension = AudioFormat::detect(bytes)
            .map(AudioFormat::extension)
            .unwrap_or("bin");
        let rel = format!(
            "{}/{}.{extension}",
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
            return Err(PortError::new("invalid locator"));
        }
        fs::read(self.root.join(locator)).map_err(PortError::new)
    }
    fn delete_all(&self, speaker: &SpeakerId) -> PortResult<()> {
        match fs::remove_dir_all(self.root.join(Self::safe(speaker.as_str())?)) {
            Err(e) if e.kind() != std::io::ErrorKind::NotFound => Err(PortError::new(e)),
            _ => Ok(()),
        }
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
        let flac = store
            .store(&SpeakerId::new("s"), &SampleId::new("b"), b"fLaC....")
            .unwrap();
        assert_eq!(flac, "s/b.flac");
        let wav = store
            .store(
                &SpeakerId::new("s"),
                &SampleId::new("c"),
                b"RIFF\0\0\0\0WAVEfmt ",
            )
            .unwrap();
        assert_eq!(wav, "s/c.wav");
        assert!(store.load("../etc/passwd").is_err());
        assert!(store
            .store(&SpeakerId::new(".."), &SampleId::new("a"), b"x")
            .is_err());
        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn deleting_a_persons_data_is_idempotent() {
        use friendly_voice_domain::audio::AcousticFeatures;
        use friendly_voice_domain::voice::ProsodyProfile;
        let dir = tmp("delete");
        let samples = JsonRepository::<VoiceSample>::new(dir.join("samples.json"));
        let sample = |id: &str, spk: &str| VoiceSample {
            id: SampleId::new(id),
            speaker_id: SpeakerId::new(spk),
            audio_locator: String::new(),
            duration_secs: 3.0,
            transcript: None,
            features: AcousticFeatures {
                mean_pitch_hz: 200.0,
                pitch_spread_hz: 1.0,
                rms_energy: 0.1,
                pause_ratio: 0.1,
            },
        };
        for (id, spk) in [("a", "s"), ("b", "s"), ("c", "t")] {
            samples.save(&sample(id, spk)).unwrap();
        }
        let s = SpeakerId::new("s");
        assert_eq!(samples.delete_by_speaker(&s).unwrap(), 2);
        assert_eq!(samples.delete_by_speaker(&s).unwrap(), 0);
        assert_eq!(
            samples.list_by_speaker(&SpeakerId::new("t")).unwrap().len(),
            1
        );

        let profiles = JsonRepository::<VoiceProfile>::new(dir.join("voices.json"));
        profiles
            .save(&VoiceProfile {
                speaker_id: s.clone(),
                engine_voice_id: "v".into(),
                language: Default::default(),
                gender: None,
                prosody: ProsodyProfile::default(),
                sample_count: 1,
                total_audio_secs: 12.0,
                built_at: None,
            })
            .unwrap();
        assert!(profiles.delete(&s).unwrap());
        assert!(!profiles.delete(&s).unwrap());
        assert_eq!(profiles.find(&s).unwrap(), None);

        let store = FileAudioStore::new(dir.join("audio"));
        store.store(&s, &SampleId::new("a"), b"x").unwrap();
        store
            .store(&SpeakerId::new("t"), &SampleId::new("c"), b"y")
            .unwrap();
        store.delete_all(&s).unwrap();
        store.delete_all(&s).unwrap();
        assert!(!dir.join("audio").join("s").exists());
        assert!(dir.join("audio").join("t").exists());
        assert!(store.delete_all(&SpeakerId::new("..")).is_err());
        fs::remove_dir_all(dir).unwrap();
    }
}
