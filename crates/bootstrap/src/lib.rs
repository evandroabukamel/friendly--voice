//! # Friendly Voice — Bootstrap
//!
//! Composition root: the only place that knows the concrete implementations.
//! It is shared by every interface (CLI, Android app via JNI).
//! Swapping an adapter (e.g. JSON → SQLite, sketch → neural engine) means
//! changing one line here — no use case changes (OCP + DIP).
pub mod parse;

use friendly_voice_application::{ports::*, use_cases::*};
use friendly_voice_domain::{
    schedule::Schedule,
    speaker::Speaker,
    voice::{VoiceProfile, VoiceSample},
};
use friendly_voice_infrastructure::{
    audio::{DspAcousticAnalyzer, WavDecoder},
    delivery::OutboxDelivery,
    messages::CuratedMessageCatalog,
    persistence::{FileAudioStore, JsonRepository},
    phonetics::PortugueseG2p,
    system::{SystemClock, TimeIdGenerator},
    voice::{LocalVoiceBank, ProsodySketchSynthesizer},
};
use std::path::Path;
use std::sync::Arc;

pub struct Container {
    pub speakers: Arc<dyn SpeakerRepository>,
    pub samples: Arc<dyn SampleRepository>,
    pub profiles: Arc<dyn VoiceProfileRepository>,
    pub schedules: Arc<dyn ScheduleRepository>,
    store: Arc<dyn AudioStore>,
    phonemizer: Arc<dyn Phonemizer>,
    clock: Arc<dyn Clock>,
    ids: Arc<dyn IdGenerator>,
    cloning: Arc<dyn VoiceCloningEngine>,
    synthesizer: Arc<dyn SpeechSynthesizer>,
    transcriber: Option<Arc<dyn Transcriber>>,
    recognizer: Option<Arc<dyn PhoneRecognizer>>,
    delivery: Arc<dyn MessageDelivery>,
    pub engine_name: &'static str,
}

impl Container {
    pub fn new(
        data: &Path,
        engine_url: Option<&str>,
        player: Option<String>,
    ) -> Result<Self, String> {
        let local_sketch = Arc::new(ProsodySketchSynthesizer::default());
        #[allow(clippy::type_complexity)]
        let (cloning, synthesizer, transcriber, recognizer, engine_name): (
            Arc<dyn VoiceCloningEngine>,
            Arc<dyn SpeechSynthesizer>,
            Option<Arc<dyn Transcriber>>,
            Option<Arc<dyn PhoneRecognizer>>,
            &'static str,
        ) = match engine_url {
            None => (
                Arc::new(LocalVoiceBank),
                local_sketch,
                None,
                None,
                "local (esboço de prosódia)",
            ),
            Some(url) => Self::http(url)?,
        };
        Ok(Self {
            speakers: Arc::new(JsonRepository::<Speaker>::new(data.join("speakers.json"))),
            samples: Arc::new(JsonRepository::<VoiceSample>::new(
                data.join("samples.json"),
            )),
            profiles: Arc::new(JsonRepository::<VoiceProfile>::new(
                data.join("voices.json"),
            )),
            schedules: Arc::new(JsonRepository::<Schedule>::new(data.join("schedules.json"))),
            store: Arc::new(FileAudioStore::new(data.join("audio"))),
            phonemizer: Arc::new(PortugueseG2p),
            clock: Arc::new(SystemClock),
            ids: Arc::new(TimeIdGenerator::default()),
            cloning,
            synthesizer,
            transcriber,
            recognizer,
            delivery: Arc::new(OutboxDelivery::new(data.join("saida"), player)),
            engine_name,
        })
    }

    #[cfg(feature = "http-engine")]
    #[allow(clippy::type_complexity)]
    fn http(
        url: &str,
    ) -> Result<
        (
            Arc<dyn VoiceCloningEngine>,
            Arc<dyn SpeechSynthesizer>,
            Option<Arc<dyn Transcriber>>,
            Option<Arc<dyn PhoneRecognizer>>,
            &'static str,
        ),
        String,
    > {
        use friendly_voice_infrastructure::voice::{
            HttpPhoneRecognizer, HttpTranscriber, HttpVoiceEngine,
        };
        let engine = Arc::new(HttpVoiceEngine::new(url));
        Ok((
            engine.clone(),
            engine,
            Some(Arc::new(HttpTranscriber::new(url))),
            Some(Arc::new(HttpPhoneRecognizer::new(url))),
            "servidor neural (HTTP)",
        ))
    }

    #[cfg(not(feature = "http-engine"))]
    #[allow(clippy::type_complexity)]
    fn http(
        _url: &str,
    ) -> Result<
        (
            Arc<dyn VoiceCloningEngine>,
            Arc<dyn SpeechSynthesizer>,
            Option<Arc<dyn Transcriber>>,
            Option<Arc<dyn PhoneRecognizer>>,
            &'static str,
        ),
        String,
    > {
        Err("--motor-url requer compilar com `cargo build --features http-engine`".into())
    }

    pub fn register_speaker(&self) -> RegisterSpeaker {
        RegisterSpeaker::new(self.speakers.clone(), self.ids.clone(), self.clock.clone())
    }

    pub fn add_voice_sample(&self) -> AddVoiceSample {
        let mut uc = AddVoiceSample::new(
            self.speakers.clone(),
            self.samples.clone(),
            self.store.clone(),
            Arc::new(WavDecoder),
            Arc::new(DspAcousticAnalyzer::default()),
            self.phonemizer.clone(),
            self.ids.clone(),
        );
        if let Some(t) = &self.transcriber {
            uc = uc.with_transcriber(t.clone());
        }
        if let Some(r) = &self.recognizer {
            uc = uc.with_phone_recognizer(r.clone());
        }
        uc
    }

    pub fn build_voice_profile(&self) -> BuildVoiceProfile {
        BuildVoiceProfile::new(
            self.speakers.clone(),
            self.samples.clone(),
            self.profiles.clone(),
            self.store.clone(),
            self.cloning.clone(),
        )
    }

    pub fn preview_pronunciation(&self) -> PreviewPronunciation {
        PreviewPronunciation::new(self.profiles.clone(), self.phonemizer.clone())
    }

    pub fn speak_message(&self) -> SpeakMessage {
        SpeakMessage::new(
            self.speakers.clone(),
            self.profiles.clone(),
            Arc::new(CuratedMessageCatalog::new()),
            self.phonemizer.clone(),
            self.synthesizer.clone(),
            self.delivery.clone(),
            self.clock.clone(),
        )
    }

    pub fn create_schedule(&self) -> CreateSchedule {
        CreateSchedule::new(
            self.speakers.clone(),
            self.schedules.clone(),
            self.ids.clone(),
            self.clock.clone(),
        )
    }

    pub fn list_schedules(&self) -> ListSchedules {
        ListSchedules::new(self.schedules.clone())
    }

    pub fn remove_schedule(&self) -> RemoveSchedule {
        RemoveSchedule::new(self.schedules.clone())
    }

    pub fn run_due_schedules(&self) -> RunDueSchedules {
        RunDueSchedules::new(
            self.schedules.clone(),
            Arc::new(self.speak_message()),
            self.clock.clone(),
        )
    }
}
