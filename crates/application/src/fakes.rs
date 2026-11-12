//! Dublês de teste (in-memory) para as portas. Mostram que os casos de uso
//! são testáveis sem I/O — benefício direto da inversão de dependência.
use crate::ports::*;
use crate::use_cases::SpeakMessage;
use chrono::{Duration, NaiveDate, NaiveDateTime};
use friendly_voice_domain::{
    accent::{AccentProfile, RuleCatalog},
    audio::{AcousticFeatures, AudioClip},
    ids::{SampleId, ScheduleId, SpeakerId},
    message::{MessageCategory, MotivationalMessage},
    phonetics::{parse_ipa, Phoneme, PhoneticUtterance, PhoneticWord},
    schedule::Schedule,
    speaker::{Consent, Speaker},
    voice::{ProsodyProfile, SpeechScript, VoiceProfile, VoiceSample},
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

/// "Decodifica" bytes que contêm apenas a duração (f32 LE) em um tom de 16 kHz.
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

/// Mini-dicionário fonético.
pub struct FakePhonemizer;
impl Phonemizer for FakePhonemizer {
    fn phonemize(&self, text: &str) -> PhoneticUtterance {
        let dict: HashMap<&str, (&str, Option<usize>)> = HashMap::from([
            ("leite", ("lejte", Some(1))),
            ("quente", ("kẽte", Some(1))),
            ("noite", ("nojte", Some(1))),
            ("de", ("de", None)),
            ("festa", ("fɛsta", Some(1))),
            ("bom", ("bõ", Some(1))),
            ("dia", ("dia", Some(1))),
        ]);
        let words = text
            .split_whitespace()
            .map(|w| {
                let w = w
                    .trim_matches(|c: char| !c.is_alphanumeric())
                    .to_lowercase();
                match dict.get(w.as_str()) {
                    Some((ipa, stress)) => PhoneticWord::new(w.clone(), parse_ipa(ipa), *stress),
                    None => PhoneticWord::new(
                        w.clone(),
                        w.chars().map(|c| Phoneme::new(c.to_string())).collect(),
                        None,
                    ),
                }
            })
            .filter(|w| !w.phonemes.is_empty())
            .collect();
        PhoneticUtterance::new(words)
    }
}

#[derive(Default)]
pub struct FakeEngine(Mutex<usize>);
impl FakeEngine {
    pub fn enrolled_references(&self) -> usize {
        *self.0.lock().unwrap()
    }
}
impl VoiceCloningEngine for FakeEngine {
    fn enroll(&self, s: &Speaker, refs: &[ReferenceAudio]) -> PortResult<String> {
        *self.0.lock().unwrap() = refs.len();
        Ok(format!("fake-voice:{}", s.id))
    }
}
impl SpeechSynthesizer for FakeEngine {
    fn synthesize(&self, _: &VoiceProfile, script: &SpeechScript) -> PortResult<SynthesizedAudio> {
        Ok(SynthesizedAudio {
            wav: vec![1, 2, 3],
            markup: script.pronunciation.ipa(),
        })
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
    pub phonemizer: Arc<FakePhonemizer>,
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
            phonemizer: Arc::new(FakePhonemizer),
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
        .unwrap();
        self.speakers.save(&s).unwrap();
        s
    }

    pub fn add_sample(&self, spk: &Speaker, secs: f32, transcript: &str, observed: Option<&str>) {
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
                transcript: transcript.into(),
                canonical: self.phonemizer.phonemize(transcript),
                observed: observed.map(parse_ipa),
                features: FakeAnalyzer.analyze(&AudioClip::new(16_000, vec![]).unwrap()),
            })
            .unwrap();
    }

    pub fn carioca_profile(&self, spk: &Speaker) {
        let catalog = RuleCatalog::portuguese();
        let accent = AccentProfile {
            label: "Carioca (RJ)".into(),
            segment_rules: catalog
                .segment_rules
                .into_iter()
                .filter(|r| {
                    [
                        "final-e-raising",
                        "final-o-raising",
                        "t-palatalization",
                        "d-palatalization",
                        "coda-s-palatalization",
                    ]
                    .contains(&r.code.as_str())
                })
                .collect(),
            juncture_rules: vec![],
        };
        self.profiles
            .save(&VoiceProfile {
                speaker_id: spk.id.clone(),
                engine_voice_id: "v".into(),
                accent,
                prosody: ProsodyProfile::default(),
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
            self.phonemizer.clone(),
            self.engine.clone(),
            self.delivery.clone(),
            self.clock.clone(),
        )
    }
}
