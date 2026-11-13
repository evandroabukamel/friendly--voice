//! Test doubles (in-memory) for the ports. They show that the use cases
//! are testable without I/O — a direct benefit of dependency inversion.
use crate::ports::*;
use crate::use_cases::SpeakMessage;
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
}
impl SpeechSynthesizer for FakeEngine {
    fn synthesize(&self, _: &VoiceProfile, script: &SpeechScript) -> PortResult<SynthesizedAudio> {
        self.spoken.lock().unwrap().push(script.clone());
        Ok(SynthesizedAudio { wav: vec![1, 2, 3] })
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
}
