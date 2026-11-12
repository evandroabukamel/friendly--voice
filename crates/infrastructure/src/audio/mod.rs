mod dsp;
mod flac;
mod format;
mod wav;

pub use dsp::DspAcousticAnalyzer;
pub use flac::FlacDecoder;
pub use format::AudioFormat;
pub use wav::{encode_wav, WavDecoder};

use friendly_voice_application::ports::{AudioDecoder, PortError, PortResult};
use friendly_voice_domain::audio::AudioClip;

/// Decodes any supported format, picking the decoder by magic bytes.
/// New formats are added here without touching the use cases (OCP).
#[derive(Debug, Clone, Copy, Default)]
pub struct AudioFileDecoder;

impl AudioDecoder for AudioFileDecoder {
    fn decode(&self, bytes: &[u8]) -> PortResult<AudioClip> {
        match AudioFormat::detect(bytes) {
            Some(AudioFormat::Wav) => WavDecoder.decode(bytes),
            Some(AudioFormat::Flac) => FlacDecoder.decode(bytes),
            None => Err(PortError::new(format!(
                "unsupported audio format; use {}",
                AudioFormat::SUPPORTED
            ))),
        }
    }
}

/// Converts any supported audio file to WAV (WAV input is returned as is).
pub fn to_wav(bytes: &[u8]) -> PortResult<Vec<u8>> {
    match AudioFormat::detect(bytes) {
        Some(AudioFormat::Wav) => Ok(bytes.to_vec()),
        _ => encode_wav(&AudioFileDecoder.decode(bytes)?),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tone() -> Vec<i32> {
        (0..8_000)
            .map(|i| ((i as f32 * 0.07).sin() * 12_000.0) as i32)
            .collect()
    }

    #[test]
    fn picks_the_decoder_by_content() {
        let flac = flac::encode_flac_for_tests(&tone(), 1, 16, 16_000);
        let from_flac = AudioFileDecoder.decode(&flac).unwrap();
        let wav = encode_wav(&from_flac).unwrap();
        let from_wav = AudioFileDecoder.decode(&wav).unwrap();
        assert_eq!(from_flac.samples().len(), from_wav.samples().len());
        let err = AudioFileDecoder.decode(b"ID3 mp3 data...").unwrap_err();
        assert!(err.0.contains("FLAC"), "{err}");
    }

    #[test]
    fn converts_flac_to_wav() {
        let flac = flac::encode_flac_for_tests(&tone(), 1, 16, 16_000);
        let wav = to_wav(&flac).unwrap();
        assert_eq!(AudioFormat::detect(&wav), Some(AudioFormat::Wav));
        assert_eq!(WavDecoder.decode(&wav).unwrap().samples().len(), 8_000);
    }
}
