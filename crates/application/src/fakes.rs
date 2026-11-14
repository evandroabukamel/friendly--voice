//! Test doubles (in-memory) for the ports. They show that the use cases
//! are testable without I/O — a direct benefit of dependency inversion.
use crate::ports::*;
use crate::use_cases::{
    AddVoiceSample, BuildVoiceProfile, CreateSchedule, DeleteSpeaker, RevokeConsent,
    SetScheduleEnabled, SpeakMessage, VoiceDataEraser,
};
use chrono::{Duration, NaiveDate, NaiveDateTime};
use friendly_voice_domain::{
    audio::{AcousticFeatures, AudioClip},
    ids::{SampleId, ScheduleId, SpeakerId},
    language::Language,
    message::{MessageCategory, MotivationalMessage},
    schedule::Schedule,
    speaker::{Consent, Speaker},
    voice::{ProsodyProfile, SpeechScript, VoiceGender, VoiceProfile, VoiceSample},
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Default)]
pub struct MemSpeakers(Mutex<Vec<Speaker>>);
impl SpeakerRepository for MemSpeakers {
    fn save(&self, s: &Speaker) -> PortResult<()> {
        let mut v = self.0.lock().unwrap();
        v.retain(|x| x.id != s.id);
        v.push(s.clone());
        Ok(())
    }
    fn find(&self, id: &SpeakerId) -> PortResult<Option<Speaker>> {
        Ok(self.0.lock().unwrap().iter().find(|s| &s.id == id).cloned())
    }
    fn list(&self) -> PortResult<Vec<Speaker>> {
        Ok(self.0.lock().unwrap().clone())
    }
    fn delete(&self, id: &SpeakerId) -> PortResult<bool> {
        let mut v = self.0.lock().unwrap();
        let before = v.len();
        v.retain(|s| &s.id != id);
        Ok(v.len() != before)
    }
}

#[derive(Default)]
pub struct MemSamples(Mutex<Vec<VoiceSample>>);
impl SampleRepository for MemSamples {
    fn save(&self, s: &VoiceSample) -> PortResult<()> {
        self.0.lock().unwrap().push(s.clone());
        Ok(())
    }
    fn list_by_speaker(&self, id: &SpeakerId) -> PortResult<Vec<VoiceSample>> {
        Ok(self
            .0
            .lock()
            .unwrap()
            .iter()
            .filter(|s| &s.speaker_id == id)
            .cloned()
            .collect())
    }
    fn delete_by_speaker(&self, id: &SpeakerId) -> PortResult<usize> {
        let mut v = self.0.lock().unwrap();
        let before = v.len();
        v.retain(|s| &s.speaker_id != id);
        Ok(before - v.len())
    }
}

#[derive(Default)]
pub struct MemProfiles(Mutex<HashMap<SpeakerId, VoiceProfile>>);
impl VoiceProfileRepository for MemProfiles {
    fn save(&self, p: &VoiceProfile) -> PortResult<()> {
        self.0
            .lock()
            .unwrap()
            .insert(p.speaker_id.clone(), p.clone());
        Ok(())
    }
    fn find(&self, id: &SpeakerId) -> PortResult<Option<VoiceProfile>> {
        Ok(self.0.lock().unwrap().get(id).cloned())
    }
    fn delete(&self, id: &SpeakerId) -> PortResult<bool> {
        Ok(self.0.lock().unwrap().remove(id).is_some())
    }
}

#[derive(Default)]
pub struct MemSchedules(Mutex<Vec<Schedule>>);
impl ScheduleRepository for MemSchedules {
    fn save(&self, s: &Schedule) -> PortResult<()> {
        let mut v = self.0.lock().unwrap();
        v.retain(|x| x.id != s.id);
        v.push(s.clone());
        Ok(())
    }
    fn list(&self) -> PortResult<Vec<Schedule>> {
        Ok(self.0.lock().unwrap().clone())
    }
    fn delete(&self, id: &ScheduleId) -> PortResult<bool> {
        let mut v = self.0.lock().unwrap();
        let before = v.len();
        v.retain(|x| &x.id != id);
        Ok(v.len() != before)
    }
}

#[derive(Default)]
pub struct MemAudio(Mutex<HashMap<String, Vec<u8>>>);
impl AudioStore for MemAudio {
    fn store(&self, spk: &SpeakerId, id: &SampleId, bytes: &[u8]) -> PortResult<String> {
        let loc = format!("{spk}/{id}");
        self.0.lock().unwrap().insert(loc.clone(), bytes.to_vec());
        Ok(loc)
    }
    fn load(&self, loc: &str) -> PortResult<Vec<u8>> {
        self.0
            .lock()
            .unwrap()
            .get(loc)
            .cloned()
            .ok_or_else(|| PortError::new("missing"))
    }
    fn delete_all(&self, spk: &SpeakerId) -> PortResult<()> {
        let prefix = format!("{spk}/");
        self.0
            .lock()
            .unwrap()
            .retain(|k, _| !k.starts_with(&prefix));
        Ok(())
    }
}
impl MemAudio {
    pub fn len(&self) -> usize {
        self.0.lock().unwrap().len()
    }
}

/// "Decodes" bytes holding only the duration (f32 LE) into a 16 kHz tone.
pub struct FakeDecoder;
impl AudioDecoder for FakeDecoder {
    fn decode(&self, bytes: &[u8]) -> PortResult<AudioClip> {
        let secs = f32::from_le_bytes(bytes[..4].try_into().map_err(PortError::new)?);
        let n = (secs * 16_000.0) as usize;
        let s = (0..n).map(|i| 0.3 * (i as f32 * 0.08).sin()).collect();
        AudioClip::new(16_000, s).map_err(PortError::new)
    }
}

pub struct FakeAnalyzer;
impl AcousticAnalyzer for FakeAnalyzer {
    fn analyze(&self, _: &AudioClip) -> AcousticFeatures {
        AcousticFeatures {
            mean_pitch_hz: 200.0,
            pitch_spread_hz: 40.0,
            rms_energy: 0.2,
            pause_ratio: 0.2,
        }
    }
}

/// Transcribes every clip as the same sentence; remembers the language asked.
#[derive(Default)]
pub struct FakeTranscriber(Mutex<Option<Language>>);
impl FakeTranscriber {
    pub fn language(&self) -> Option<Language> {
        self.0.lock().unwrap().clone()
    }
}
impl Transcriber for FakeTranscriber {
    fn transcribe(&self, _: &AudioClip, language: &Language) -> PortResult<String> {
        *self.0.lock().unwrap() = Some(language.clone());
        Ok("bom dia".into())
    }
}

#[derive(Default)]
pub struct FakeEngine {
    enrolled: Mutex<(usize, Option<VoiceGender>)>,
    spoken: Mutex<Vec<SpeechScript>>,
    forgotten: Mutex<Vec<String>>,
    /// When set, `forget` fails (voice server unreachable).
    pub forget_fails: Mutex<bool>,
}
impl FakeEngine {
    pub fn enrolled_references(&self) -> usize {
        self.enrolled.lock().unwrap().0
    }
    pub fn enrolled_gender(&self) -> Option<VoiceGender> {
        self.enrolled.lock().unwrap().1
    }
    pub fn spoken(&self) -> Vec<SpeechScript> {
        self.spoken.lock().unwrap().clone()
    }
    pub fn forgotten(&self) -> Vec<String> {
        self.forgotten.lock().unwrap().clone()
    }
}
impl VoiceCloningEngine for FakeEngine {
    fn enroll(
        &self,
        s: &Speaker,
        gender: Option<VoiceGender>,
        refs: &[ReferenceAudio],
    ) -> PortResult<String> {
        *self.enrolled.lock().unwrap() = (refs.len(), gender);
        Ok(format!("fake-voice:{}", s.id))
    }
    fn forget(&self, voice_id: &str) -> PortResult<()> {
        if *self.forget_fails.lock().unwrap() {
            return Err(PortError::new("voice server unreachable"));
        }
        self.forgotten.lock().unwrap().push(voice_id.to_string());
        Ok(())
    }
}
impl SpeechSynthesizer for FakeEngine {
    fn synthesize(&self, _: &VoiceProfile, script: &SpeechScript) -> PortResult<SynthesizedAudio> {
        self.spoken.lock().unwrap().push(script.clone());
        Ok(SynthesizedAudio { wav: vec![1, 2, 3] })
    }
}

/// Remembers which people had their cached speech cleared.
#[derive(Default)]
pub struct MemSpeechCache(Mutex<Vec<SpeakerId>>);
impl MemSpeechCache {
    pub fn cleared(&self) -> Vec<SpeakerId> {
        self.0.lock().unwrap().clone()
    }
}
impl SpeechCache for MemSpeechCache {
    fn clear(&self, speaker: &SpeakerId) -> PortResult<()> {
        self.0.lock().unwrap().push(speaker.clone());
        Ok(())
    }
}

pub struct FakeMessages;
impl MessageSource for FakeMessages {
    fn pick(
        &self,
        c: MessageCategory,
        _: &Speaker,
        listener: Option<&str>,
    ) -> PortResult<MotivationalMessage> {
        let text = format!("Você consegue, {}!", listener.unwrap_or("querido"));
        MotivationalMessage::new(&text, Some(c)).map_err(PortError::new)
    }
}

#[derive(Default)]
pub struct FakeDelivery(Mutex<Vec<String>>);
impl FakeDelivery {
    pub fn delivered(&self) -> Vec<String> {
        self.0.lock().unwrap().clone()
    }
}
impl MessageDelivery for FakeDelivery {
    fn deliver(&self, d: &Delivery<'_>) -> PortResult<String> {
        self.0.lock().unwrap().push(d.message.text().to_string());
        Ok("ok".into())
    }
}

pub struct FakeClock(Mutex<NaiveDateTime>);
impl FakeClock {
    pub fn advance(&self, d: Duration) {
        *self.0.lock().unwrap() += d;
    }
}
impl Clock for FakeClock {
    fn now(&self) -> NaiveDateTime {
        *self.0.lock().unwrap()
    }
}

#[derive(Default)]
pub struct SeqIds(Mutex<u32>);
impl IdGenerator for SeqIds {
    fn next_id(&self, prefix: &str) -> String {
        let mut n = self.0.lock().unwrap();
        *n += 1;
        format!("{prefix}-{n}")
    }
}

pub struct Fakes {
    pub speakers: Arc<MemSpeakers>,
    pub samples: Arc<MemSamples>,
    pub profiles: Arc<MemProfiles>,
    pub schedules: Arc<MemSchedules>,
    pub store: Arc<MemAudio>,
    pub decoder: Arc<FakeDecoder>,
    pub analyzer: Arc<FakeAnalyzer>,
    pub transcriber: Arc<FakeTranscriber>,
    pub engine: Arc<FakeEngine>,
    pub speech_cache: Arc<MemSpeechCache>,
    pub messages: Arc<FakeMessages>,
    pub delivery: Arc<FakeDelivery>,
    pub clock: Arc<FakeClock>,
    pub ids: Arc<SeqIds>,
}

impl Fakes {
    pub fn new() -> Self {
        let start = NaiveDate::from_ymd_opt(2026, 11, 2)
            .unwrap()
            .and_hms_opt(10, 0, 0)
            .unwrap();
        Self {
            speakers: Default::default(),
            samples: Default::default(),
            profiles: Default::default(),
            schedules: Default::default(),
            store: Default::default(),
            decoder: Arc::new(FakeDecoder),
            analyzer: Arc::new(FakeAnalyzer),
            transcriber: Default::default(),
            engine: Default::default(),
            speech_cache: Default::default(),
            messages: Arc::new(FakeMessages),
            delivery: Default::default(),
            clock: Arc::new(FakeClock(Mutex::new(start))),
            ids: Default::default(),
        }
    }

    pub fn audio_bytes(secs: f32) -> Vec<u8> {
        secs.to_le_bytes().to_vec()
    }

    pub fn speaker(&self) -> Speaker {
        self.speaker_speaking(Language::default())
    }

    pub fn speaker_speaking(&self, language: Language) -> Speaker {
        let s = Speaker::new(
            SpeakerId::new(self.ids.next_id("spk")),
            "Mãe",
            None,
            Some(Consent {
                granted_by: "Mãe".into(),
                granted_at: self.clock.now(),
                statement: "ok".into(),
            }),
        )
        .unwrap()
        .with_language(language);
        self.speakers.save(&s).unwrap();
        s
    }

    pub fn add_sample(&self, spk: &Speaker, secs: f32, transcript: Option<&str>) {
        let id = SampleId::new(self.ids.next_id("smp"));
        let loc = self
            .store
            .store(&spk.id, &id, &Self::audio_bytes(secs))
            .unwrap();
        self.samples
            .save(&VoiceSample {
                id,
                speaker_id: spk.id.clone(),
                audio_locator: loc,
                duration_secs: secs,
                transcript: transcript.map(str::to_string),
                features: FakeAnalyzer.analyze(&AudioClip::new(16_000, vec![]).unwrap()),
            })
            .unwrap();
    }

    pub fn voice_profile(&self, spk: &Speaker) {
        self.profiles
            .save(&VoiceProfile {
                speaker_id: spk.id.clone(),
                engine_voice_id: "v".into(),
                language: spk.language.clone(),
                gender: Some(VoiceGender::Female),
                prosody: ProsodyProfile {
                    mean_pitch_hz: 210.0,
                    pitch_spread_hz: 40.0,
                    syllables_per_second: 5.0,
                    pause_ratio: 0.2,
                },
                sample_count: 1,
                total_audio_secs: 10.0,
                built_at: None,
            })
            .unwrap();
    }

    pub fn speak_message(&self) -> SpeakMessage {
        SpeakMessage::new(
            self.speakers.clone(),
            self.profiles.clone(),
            self.messages.clone(),
            self.engine.clone(),
            self.delivery.clone(),
            self.clock.clone(),
        )
    }

    pub fn eraser(&self) -> VoiceDataEraser {
        VoiceDataEraser {
            schedules: self.schedules.clone(),
            samples: self.samples.clone(),
            store: self.store.clone(),
            profiles: self.profiles.clone(),
            engine: self.engine.clone(),
            cache: self.speech_cache.clone(),
            clock: self.clock.clone(),
        }
    }

    pub fn revoke_consent(&self) -> RevokeConsent {
        RevokeConsent::new(self.speakers.clone(), self.eraser(), self.clock.clone())
    }

    pub fn delete_speaker(&self) -> DeleteSpeaker {
        DeleteSpeaker::new(self.speakers.clone(), self.eraser(), self.clock.clone())
    }

    pub fn create_schedule(&self) -> CreateSchedule {
        CreateSchedule::new(
            self.speakers.clone(),
            self.schedules.clone(),
            self.ids.clone(),
            self.clock.clone(),
        )
    }

    pub fn set_schedule_enabled(&self) -> SetScheduleEnabled {
        SetScheduleEnabled::new(
            self.speakers.clone(),
            self.schedules.clone(),
            self.clock.clone(),
        )
    }

    pub fn add_voice_sample(&self) -> AddVoiceSample {
        AddVoiceSample::new(
            self.speakers.clone(),
            self.samples.clone(),
            self.store.clone(),
            self.decoder.clone(),
            self.analyzer.clone(),
            self.ids.clone(),
        )
    }

    pub fn build_voice_profile(&self) -> BuildVoiceProfile {
        BuildVoiceProfile::new(
            self.speakers.clone(),
            self.samples.clone(),
            self.profiles.clone(),
            self.store.clone(),
            self.engine.clone(),
            self.speech_cache.clone(),
            self.clock.clone(),
        )
    }
}
