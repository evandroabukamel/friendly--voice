//! Acceptance tests (criteria of stories US-01..US-08 and US-18), exercising the
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
    fn wav(&self, name: &str, secs: f32, f0: f32, transcript: &str, ipa: &str) -> PathBuf {
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
        std::fs::write(path.with_extension("txt"), transcript).unwrap();
        std::fs::write(path.with_extension("ipa"), ipa).unwrap();
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
        .split(')')
        .next()
        .unwrap()
        .to_string()
}

fn p(path: &Path) -> &str {
    path.to_str().unwrap()
}

#[test]
fn full_flow_learns_accent_and_speaks() {
    let env = Env::new("flow");

    // US-01: consent is mandatory
    let (ok, out) = env.run(&["pessoa", "adicionar", "--nome", "Vó"]);
    assert!(!ok && out.contains("consentimento"), "{out}");
    let id = speaker_id(&env.ok(&[
        "pessoa",
        "adicionar",
        "--nome",
        "Vó",
        "--consentimento-de",
        "Vó",
    ]));

    // US-02: samples with transcript and phonemes (files next to the .wav)
    let samples = [
        env.wav(
            "a1",
            6.0,
            210.0,
            "Leite quente de noite, tudo certo.",
            "lejtʃi kẽtʃi dʒi nojtʃi tudu sɛhtu",
        ),
        env.wav(
            "a2",
            6.0,
            205.0,
            "Os amigos de verdade vão falar com você.",
            "uz amiguʃ dʒi vehdadʒi vãw fala kũ vose",
        ),
        env.wav(
            "a3",
            5.0,
            215.0,
            "A festa é mesmo na porta do mar.",
            "a fɛʃta ɛ meʒmu na pɔhta du ma",
        ),
    ];
    for s in &samples {
        env.ok(&["amostra", "adicionar", "--pessoa", &id, "--audio", p(s)]);
    }

    // US-03/US-04: voice with the learned accent
    let out = env.ok(&["voz", "gerar", "--pessoa", &id]);
    assert!(out.contains("Carioca"), "{out}");
    assert!(out.contains("✔    término"), "{out}");
    assert!(out.contains("✔    pronúncia"), "{out}");

    // US-06: pronunciation of new text with the accent
    let out = env.ok(&["pronuncia", "--pessoa", &id, "Bom dia, você é forte!"]);
    assert!(out.contains("/bõ dʒia vose ɛ fohtʃi/"), "{out}");

    // US-05: an on-demand message produces audio
    let out = env.ok(&[
        "--meu-nome",
        "Evandro",
        "falar",
        "--pessoa",
        &id,
        "--tema",
        "celebracao",
    ]);
    assert!(out.contains("Evandro"), "{out}");
    let saida = env.dir.join("data/saida");
    assert!(std::fs::read_dir(&saida).unwrap().any(|e| e
        .unwrap()
        .path()
        .extension()
        .is_some_and(|x| x == "wav")));

    // US-18: the user picks the tone of voice of each message
    let out = env.ok(&["falar", "--pessoa", &id, "--tema", "calma"]);
    assert!(
        out.contains("tom: calmo"),
        "initial tone suggested by the theme:\n{out}"
    );
    let out = env.ok(&[
        "falar",
        "--pessoa",
        &id,
        "--tema",
        "calma",
        "--tom",
        "animado",
        "--ajuste-tom",
        "-2",
    ]);
    assert!(out.contains("tom: animado (-2 st)"), "{out}");
    let (ok, out) = env.run(&["falar", "--pessoa", &id, "--tom", "bravo"]);
    assert!(!ok && out.contains("tom 'bravo' desconhecido"), "{out}");
    let (ok, out) = env.run(&["falar", "--pessoa", &id, "--ajuste-tom", "9"]);
    assert!(!ok && out.contains("semitons"), "{out}");
    let out = env.ok(&[
        "agenda",
        "criar",
        "--pessoa",
        &id,
        "--diario",
        "21:00",
        "--tom",
        "sussurrado",
    ]);
    assert!(out.contains("tom: sussurrado"), "{out}");

    // US-07/US-08: scheduling
    let out = env.ok(&[
        "agenda",
        "criar",
        "--pessoa",
        &id,
        "--diario",
        "08:00",
        "--silencio",
        "22:00-07:00",
    ]);
    assert!(out.contains("todo dia às 08:00"), "{out}");
    let (ok, out) = env.run(&["agenda", "criar", "--pessoa", &id, "--semanal", "xyz@08:00"]);
    assert!(!ok && out.contains("dia inválido"), "{out}");
    env.ok(&["agenda", "executar"]);
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
    let id = speaker_id(&env.ok(&[
        "pessoa",
        "adicionar",
        "--nome",
        "Mãe",
        "--consentimento-de",
        "Mãe",
    ]));
    for (name, f0) in [("f1", 200.0), ("f2", 190.0)] {
        let path = env.dir.join(format!("{name}.flac"));
        flac(&path, 6.0, f0);
        // Sidecar files work for FLAC too.
        std::fs::write(path.with_extension("txt"), "Leite quente de noite.").unwrap();
        std::fs::write(path.with_extension("ipa"), "lejtʃi kẽtʃi dʒi nojtʃi").unwrap();
        let out = env.ok(&["amostra", "adicionar", "--pessoa", &id, "--audio", p(&path)]);
        assert!(out.contains("adicionada (6.0s)"), "{out}");
        assert!(out.contains("Ouvido:   /lejtʃikẽtʃidʒinojtʃi/"), "{out}");
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
    env.ok(&["voz", "gerar", "--pessoa", &id]);
    env.ok(&["falar", "--pessoa", &id, "--texto", "Boa noite."]);

    let ogg = env.dir.join("x.ogg");
    std::fs::write(&ogg, b"OggS\0\0\0\0\0\0\0\0 not supported").unwrap();
    let (ok, out) = env.run(&[
        "amostra",
        "adicionar",
        "--pessoa",
        &id,
        "--audio",
        p(&ogg),
        "--texto",
        "oi",
    ]);
    assert!(!ok && out.contains("WAV (PCM/float) ou FLAC"), "{out}");
}

#[test]
fn rejects_invalid_audio() {
    let env = Env::new("bad");
    let id = speaker_id(&env.ok(&[
        "pessoa",
        "adicionar",
        "--nome",
        "Mãe",
        "--consentimento-de",
        "Mãe",
    ]));
    let short = env.wav("curto", 0.5, 200.0, "oi", "oj");
    let (ok, out) = env.run(&[
        "amostra",
        "adicionar",
        "--pessoa",
        &id,
        "--audio",
        p(&short),
    ]);
    assert!(!ok && out.contains("muito curto"), "{out}");
    let (ok, out) = env.run(&["voz", "gerar", "--pessoa", &id]);
    assert!(!ok && out.contains("insuficiente"), "{out}");
}
