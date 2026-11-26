//! Acceptance tests (criteria of stories US-01..US-08, US-11, US-18 and US-27), exercising the
//! real binary with generated audio and data in a temporary directory.
use std::path::{Path, PathBuf};
use std::process::Command;

struct Env {
    dir: PathBuf,
}

impl Env {
    fn new(name: &str) -> Self {
        let dir = std::env::temp_dir().join(format!("fv-accept-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        Self { dir }
    }

    fn run(&self, args: &[&str]) -> (bool, String) {
        let out = Command::new(env!("CARGO_BIN_EXE_friendly-voice"))
            .args(args)
            .env("FRIENDLY_VOICE_DATA", self.dir.join("data"))
            .env_remove("FRIENDLY_VOICE_ENGINE_URL")
            .env_remove("FRIENDLY_VOICE_PLAYER")
            .output()
            .unwrap();
        let text = format!(
            "{}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
        (out.status.success(), text)
    }

    fn ok(&self, args: &[&str]) -> String {
        let (ok, text) = self.run(args);
        assert!(ok, "command {args:?} failed:\n{text}");
        text
    }

    /// Voiced audio (harmonics) with regular pauses.
    fn wav(&self, name: &str, secs: f32, f0: f32, transcript: Option<&str>) -> PathBuf {
        let path = self.dir.join(format!("{name}.wav"));
        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: 16_000,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };
        let mut w = hound::WavWriter::create(&path, spec).unwrap();
        for i in 0..(secs * 16_000.0) as usize {
            let t = i as f32 / 16_000.0;
            let on = (t % 0.5) < 0.35;
            let v: f32 = if on {
                (1..6)
                    .map(|k| (std::f32::consts::TAU * f0 * k as f32 * t).sin() / k as f32)
                    .sum::<f32>()
                    * 0.15
            } else {
                0.0
            };
            w.write_sample((v * i16::MAX as f32) as i16).unwrap();
        }
        w.finalize().unwrap();
        if let Some(t) = transcript {
            std::fs::write(path.with_extension("txt"), t).unwrap();
        }
        path
    }
}

impl Drop for Env {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.dir);
    }
}

fn speaker_id(out: &str) -> String {
    out.split("id: ")
        .nth(1)
        .unwrap()
        .split([')', ','])
        .next()
        .unwrap()
        .to_string()
}

fn p(path: &Path) -> &str {
    path.to_str().unwrap()
}

#[test]
fn full_flow_clones_the_voice_and_speaks() {
    let env = Env::new("flow");

    // US-01: consent is mandatory
    let (ok, out) = env.run(&["person", "add", "--name", "Vó"]);
    assert!(!ok && out.contains("consent"), "{out}");
    let out = env.ok(&["person", "add", "--name", "Vó", "--consent-by", "Vó"]);
    assert!(out.contains("speaks Portuguese"), "{out}");
    let id = speaker_id(&out);

    // US-02: samples, with or without a transcript (file next to the .wav)
    let samples = [
        env.wav("a1", 6.0, 210.0, Some("Leite quente de noite, tudo certo.")),
        env.wav("a2", 6.0, 205.0, None),
        env.wav("a3", 5.0, 215.0, Some("A festa é mesmo na porta do mar.")),
    ];
    for s in &samples {
        env.ok(&["sample", "add", "--person", &id, "--audio", p(s)]);
    }

    // US-11: the voice is cloned from the samples; a high voice is female
    let out = env.ok(&["voice", "generate", "--person", &id]);
    assert!(out.contains("female voice"), "{out}");
    assert!(out.contains("Portuguese (pt-BR)"), "{out}");
    let out = env.ok(&["voice", "show", "--person", &id]);
    assert!(out.contains("3 sample(s)"), "{out}");

    // US-05: an on-demand message produces audio
    let out = env.ok(&[
        "--my-name",
        "Evandro",
        "speak",
        "--person",
        &id,
        "--theme",
        "celebration",
    ]);
    assert!(out.contains("Evandro"), "{out}");
    let outbox = env.dir.join("data/outbox");
    assert!(std::fs::read_dir(&outbox).unwrap().any(|e| e
        .unwrap()
        .path()
        .extension()
        .is_some_and(|x| x == "wav")));

    // US-18: the user picks the tone of voice of each message
    let out = env.ok(&["speak", "--person", &id, "--theme", "calm"]);
    assert!(
        out.contains("tone: calm"),
        "initial tone suggested by the theme:\n{out}"
    );
    let out = env.ok(&[
        "speak",
        "--person",
        &id,
        "--theme",
        "calm",
        "--tone",
        "cheerful",
        "--pitch-shift",
        "-2",
    ]);
    assert!(out.contains("tone: cheerful (-2 st)"), "{out}");
    let (ok, out) = env.run(&["speak", "--person", &id, "--tone", "bravo"]);
    assert!(!ok && out.contains("unknown tone 'bravo'"), "{out}");
    let (ok, out) = env.run(&["speak", "--person", &id, "--pitch-shift", "9"]);
    assert!(!ok && out.contains("semitones"), "{out}");
    let out = env.ok(&[
        "schedule", "create", "--person", &id, "--daily", "21:00", "--tone", "whisper",
    ]);
    assert!(out.contains("tone: whisper"), "{out}");

    // US-07/US-08: scheduling
    let out = env.ok(&[
        "schedule",
        "create",
        "--person",
        &id,
        "--daily",
        "08:00",
        "--quiet",
        "22:00-07:00",
    ]);
    assert!(out.contains("every day at 08:00"), "{out}");
    let (ok, out) = env.run(&[
        "schedule",
        "create",
        "--person",
        &id,
        "--weekly",
        "xyz@08:00",
    ]);
    assert!(!ok && out.contains("invalid weekday"), "{out}");
    env.ok(&["schedule", "run"]);
}

/// Voiced audio encoded as 24-bit stereo FLAC.
fn flac(path: &Path, secs: f32, f0: f32) {
    use flacenc::component::BitRepr;
    use flacenc::error::Verify;
    let sr = 44_100usize;
    let scale = (1 << 23) as f32 - 1.0;
    let samples: Vec<i32> = (0..(secs * sr as f32) as usize)
        .flat_map(|i| {
            let t = i as f32 / sr as f32;
            let v: f32 = if (t % 0.5) < 0.35 {
                (1..6)
                    .map(|k| (std::f32::consts::TAU * f0 * k as f32 * t).sin() / k as f32)
                    .sum::<f32>()
                    * 0.15
            } else {
                0.0
            };
            let s = (v * scale) as i32;
            [s, s]
        })
        .collect();
    let config = flacenc::config::Encoder::default().into_verified().unwrap();
    let source = flacenc::source::MemSource::from_samples(&samples, 2, 24, sr);
    let stream = flacenc::encode_with_fixed_block_size(&config, source, config.block_size).unwrap();
    let mut sink = flacenc::bitsink::ByteSink::new();
    stream.write(&mut sink).unwrap();
    std::fs::write(path, sink.as_slice()).unwrap();
}

#[test]
fn accepts_flac_samples() {
    let env = Env::new("flac");
    let id = speaker_id(&env.ok(&["person", "add", "--name", "Mãe", "--consent-by", "Mãe"]));
    for (name, f0) in [("f1", 200.0), ("f2", 190.0)] {
        let path = env.dir.join(format!("{name}.flac"));
        flac(&path, 6.0, f0);
        // The transcript sidecar works for FLAC too.
        std::fs::write(path.with_extension("txt"), "Leite quente de noite.").unwrap();
        let out = env.ok(&["sample", "add", "--person", &id, "--audio", p(&path)]);
        assert!(out.contains("added (6.0s)"), "{out}");
        assert!(out.contains("Text: Leite quente de noite."), "{out}");
    }
    // The original FLAC is kept, with the right extension.
    let audio_dir = env.dir.join("data/audio").join(&id);
    let stored: Vec<_> = std::fs::read_dir(audio_dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect();
    assert!(
        stored
            .iter()
            .all(|p| p.extension().is_some_and(|x| x == "flac")),
        "{stored:?}"
    );
    env.ok(&["voice", "generate", "--person", &id]);
    env.ok(&["speak", "--person", &id, "--text", "Boa noite."]);

    // AIFF: a real audio format, but not one we read.
    let aiff = env.dir.join("x.aiff");
    std::fs::write(&aiff, b"FORM\0\0\0\x2eAIFFCOMM not supported").unwrap();
    let (ok, out) = env.run(&[
        "sample",
        "add",
        "--person",
        &id,
        "--audio",
        p(&aiff),
        "--transcript",
        "oi",
    ]);
    assert!(
        !ok && out
            .contains("This audio format is not supported. Use a WAV, FLAC, MP3, M4A or OGG file."),
        "{out}"
    );
}

/// The former Portuguese command and option names still work (hidden aliases),
/// so existing scripts and cron jobs are not broken.
#[test]
fn portuguese_aliases_still_work() {
    let env = Env::new("aliases");
    let id = speaker_id(&env.ok(&[
        "pessoa",
        "adicionar",
        "--nome",
        "Vó",
        "--consentimento-de",
        "Vó",
    ]));
    let wav = env.wav("a1", 12.0, 210.0, None);
    env.ok(&[
        "amostra",
        "adicionar",
        "--pessoa",
        &id,
        "--audio",
        p(&wav),
        "--texto",
        "Leite quente de noite.",
    ]);
    env.ok(&["amostra", "listar", "--pessoa", &id]);
    env.ok(&["--meu-nome", "Ana", "voz", "gerar", "--pessoa", &id]);
    let out = env.ok(&[
        "falar",
        "--pessoa",
        &id,
        "--tema",
        "calma",
        "--tom",
        "carinhoso",
        "--ajuste-tom",
        "-1",
    ]);
    assert!(out.contains("tone: warm (-1 st)"), "{out}");
    env.ok(&[
        "agenda",
        "criar",
        "--pessoa",
        &id,
        "--semanal",
        "seg,sex@07:30",
        "--silencio",
        "22:00-07:00",
    ]);
    env.ok(&["agenda", "executar"]);
    env.ok(&["temas"]);
    env.ok(&["tons"]);
}

/// English values are accepted for themes, tones and weekdays.
#[test]
fn english_values_are_accepted() {
    let env = Env::new("english-values");
    let id = speaker_id(&env.ok(&[
        "person",
        "add",
        "--name",
        "Grandma",
        "--consent-by",
        "Grandma",
    ]));
    let wav = env.wav("a1", 12.0, 210.0, None);
    env.ok(&["sample", "add", "--person", &id, "--audio", p(&wav)]);
    env.ok(&["voice", "generate", "--person", &id]);
    let out = env.ok(&[
        "speak",
        "--person",
        &id,
        "--theme",
        "good-morning",
        "--tone",
        "whisper",
    ]);
    assert!(out.contains("tone: whisper"), "{out}");
    let out = env.ok(&[
        "schedule",
        "create",
        "--person",
        &id,
        "--weekly",
        "mon,fri@07:30",
        "--theme",
        "calm",
    ]);
    assert!(out.contains("mon,fri at 07:30"), "{out}");
}

/// Real ≈3 s phone-style recordings shared with the infrastructure tests.
fn fixture(name: &str) -> Vec<u8> {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../infrastructure/tests/fixtures");
    std::fs::read(dir.join(name)).unwrap()
}

/// US-16: MP3, M4A and OGG recordings are accepted like WAV and FLAC.
#[test]
fn accepts_mp3_m4a_and_ogg_samples() {
    let env = Env::new("compressed");
    let help = env.ok(&["sample", "add", "--help"]);
    assert!(help.contains(".mp3, .m4a or .ogg"), "{help}");

    let id = speaker_id(&env.ok(&["person", "add", "--name", "Mãe", "--consent-by", "Mãe"]));
    // Four samples: the voice needs at least 10 s in total.
    for (i, name) in ["voice.mp3", "voice.m4a", "voice.ogg", "voice.mp3"]
        .iter()
        .enumerate()
    {
        // The format comes from the content: the stored name is not trusted.
        let path = env.dir.join(format!("upload-{i}"));
        std::fs::write(&path, fixture(name)).unwrap();
        let out = env.ok(&["sample", "add", "--person", &id, "--audio", p(&path)]);
        assert!(out.contains("added (3.0s)"), "{name}: {out}");
    }
    // The original files are kept, with the right extensions.
    let mut stored: Vec<_> = std::fs::read_dir(env.dir.join("data/audio").join(&id))
        .unwrap()
        .map(|e| e.unwrap().path().extension().unwrap().to_owned())
        .collect();
    stored.sort();
    assert_eq!(stored, ["m4a", "mp3", "mp3", "ogg"]);
    env.ok(&["voice", "generate", "--person", &id]);

    // A recording over 3 minutes goes through the same limit as a WAV.
    // MP3 frames can be concatenated: repeat the tag-less frames 61 times.
    let mp3 = fixture("voice.mp3");
    let tag = 10
        + mp3[6..10]
            .iter()
            .fold(0usize, |acc, &b| (acc << 7) | b as usize);
    let long = env.dir.join("long.mp3");
    std::fs::write(&long, mp3[tag..].repeat(61)).unwrap();
    let (ok, out) = env.run(&["sample", "add", "--person", &id, "--audio", p(&long)]);
    assert!(
        !ok && out.contains("too long") && out.contains("split it"),
        "{out}"
    );

    let opus = env.dir.join("opus.ogg");
    std::fs::write(&opus, fixture("opus.ogg")).unwrap();
    let (ok, out) = env.run(&["sample", "add", "--person", &id, "--audio", p(&opus)]);
    assert!(
        !ok && out.contains("This audio format is not supported. Convert the file to WAV."),
        "{out}"
    );
    let silent = env.dir.join("no-audio.m4a");
    std::fs::write(&silent, fixture("no-audio.m4a")).unwrap();
    let (ok, out) = env.run(&["sample", "add", "--person", &id, "--audio", p(&silent)]);
    assert!(
        !ok && out.contains("This file has no audio. Use a WAV, FLAC, MP3, M4A or OGG file."),
        "{out}"
    );
}

#[test]
fn rejects_invalid_audio() {
    let env = Env::new("bad");
    let id = speaker_id(&env.ok(&["person", "add", "--name", "Mãe", "--consent-by", "Mãe"]));
    let short = env.wav("curto", 0.5, 200.0, Some("oi"));
    let (ok, out) = env.run(&["sample", "add", "--person", &id, "--audio", p(&short)]);
    assert!(!ok && out.contains("too short"), "{out}");
    let (ok, out) = env.run(&["voice", "generate", "--person", &id]);
    assert!(!ok && out.contains("not enough"), "{out}");
}

/// US-27: the language of the recordings is chosen per person; a low voice
/// is detected as male.
#[test]
fn speaks_in_the_chosen_language() {
    let env = Env::new("language");
    let (ok, out) = env.run(&[
        "person",
        "add",
        "--name",
        "Grandpa",
        "--consent-by",
        "Grandpa",
        "--language",
        "xx",
    ]);
    assert!(!ok && out.contains("unsupported language"), "{out}");
    let out = env.ok(&[
        "person",
        "add",
        "--name",
        "Grandpa",
        "--consent-by",
        "Grandpa",
        "--language",
        "en-us",
    ]);
    assert!(out.contains("speaks English"), "{out}");
    let id = speaker_id(&out);
    let wav = env.wav("g1", 12.0, 115.0, Some("Good morning, my dear."));
    env.ok(&["sample", "add", "--person", &id, "--audio", p(&wav)]);
    let out = env.ok(&["voice", "generate", "--person", &id]);
    assert!(
        out.contains("male voice") && out.contains("English (en-US)"),
        "{out}"
    );

    // the themes are written in Portuguese; other languages speak free text
    let (ok, out) = env.run(&["speak", "--person", &id, "--theme", "calm"]);
    assert!(!ok && out.contains("only available in Portuguese"), "{out}");
    let out = env.ok(&["speak", "--person", &id, "--text", "You can do it!"]);
    assert!(out.contains("language: en-US"), "{out}");
    let out = env.ok(&["person", "list"]);
    assert!(
        out.contains("[en-US]") && out.contains("male voice"),
        "{out}"
    );
    let out = env.ok(&["languages"]);
    assert!(
        out.contains("pt  Portuguese") && out.contains("ja  Japanese"),
        "{out}"
    );
}

#[test]
fn the_name_the_person_uses_wins_over_my_name() {
    let env = Env::new("listener");
    let out = env.ok(&[
        "person",
        "add",
        "--name",
        "Mãe",
        "--consent-by",
        "Mãe",
        "--listener-name",
        "Filhote",
    ]);
    assert!(out.contains("calls you Filhote"), "{out}");
    let id = speaker_id(&out);
    let wav = env.wav("m1", 12.0, 210.0, None);
    env.ok(&["sample", "add", "--person", &id, "--audio", p(&wav)]);
    env.ok(&["voice", "generate", "--person", &id]);
    let out = env.ok(&[
        "--my-name",
        "Evandro",
        "speak",
        "--person",
        &id,
        "--theme",
        "celebration",
    ]);
    assert!(out.contains("Filhote") && !out.contains("Evandro"), "{out}");
}

#[test]
fn listener_names_revoke_and_delete_from_the_cli() {
    let env = Env::new("lifecycle");
    let out = env.ok(&["person", "add", "--name", "Mãe", "--consent-by", "Mãe"]);
    let id = speaker_id(&out);
    let wav = env.wav("m1", 12.0, 210.0, None);
    env.ok(&["sample", "add", "--person", &id, "--audio", p(&wav)]);
    env.ok(&["voice", "generate", "--person", &id]);
    let speak = |extra: &[&str]| {
        let mut args = vec!["--my-name", "Evandro", "speak", "--person", &id];
        args.extend(["--theme", "celebration"]);
        args.extend(extra);
        env.ok(&args)
    };

    // A name for one message wins over the stored one and --my-name.
    let out = speak(&["--listener-name", "Campeão"]);
    assert!(out.contains("Campeão"), "{out}");
    let out = env.ok(&[
        "person",
        "set-listener-name",
        "--person",
        &id,
        "--listener-name",
        "Filhote",
    ]);
    assert!(out.contains("Mãe calls you Filhote"), "{out}");
    assert!(speak(&[]).contains("Filhote"));
    let out = env.ok(&["person", "set-listener-name", "--person", &id]);
    assert!(out.contains("cleared"), "{out}");
    assert!(speak(&[]).contains("Evandro"));

    // US-15: revoking consent deletes the voice and refuses every use.
    env.ok(&["schedule", "create", "--person", &id, "--daily", "08:00"]);
    let out = env.ok(&["person", "revoke", "--person", &id, "--by", "Mãe"]);
    assert!(out.contains("revoked") && out.contains("deleted"), "{out}");
    env.ok(&["person", "revoke", "--person", &id]);
    let (ok, out) = env.run(&["speak", "--person", &id, "--text", "oi"]);
    assert!(!ok && out.contains("revoked"), "{out}");
    let (ok, out) = env.run(&["sample", "add", "--person", &id, "--audio", p(&wav)]);
    assert!(!ok && out.contains("revoked"), "{out}");
    assert!(!env.ok(&["person", "list"]).contains(&id));
    let out = env.ok(&["schedule", "list"]);
    assert!(
        out.contains("next: —"),
        "the schedule is kept, switched off: {out}"
    );
    assert!(!env.dir.join("data/audio").join(&id).exists());

    // Deleting a person is idempotent.
    let out = env.ok(&["person", "add", "--name", "Tio", "--consent-by", "Tio"]);
    let uncle = speaker_id(&out);
    let out = env.ok(&["person", "delete", "--person", &uncle]);
    assert!(out.contains("deleted"), "{out}");
    env.ok(&["person", "delete", "--person", &uncle]);
    assert!(!env.ok(&["person", "list"]).contains(&uncle));
}
