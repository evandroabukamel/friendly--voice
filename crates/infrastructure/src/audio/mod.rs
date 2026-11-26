mod compressed;
mod dsp;
mod flac;
mod format;
mod wav;

pub use compressed::CompressedAudioDecoder;
pub use dsp::DspAcousticAnalyzer;
pub use flac::FlacDecoder;
pub use format::AudioFormat;
pub use wav::{encode_wav, WavDecoder};

use friendly_voice_application::ports::{AudioDecoder, PortError, PortResult};
use friendly_voice_domain::audio::AudioClip;

/// The file's bytes match none of the supported formats.
pub const UNSUPPORTED_FORMAT: &str =
    "This audio format is not supported. Use a WAV, FLAC, MP3, M4A or OGG file.";
/// The container holds no audio track (e.g. only video or subtitles).
pub const NO_AUDIO_TRACK: &str = "This file has no audio. Use a WAV, FLAC, MP3, M4A or OGG file.";
/// The container is supported but its codec is not (e.g. Opus, ALAC).
pub const UNSUPPORTED_CODEC: &str = "This audio format is not supported. Convert the file to WAV.";
/// The container is supported but its content is damaged or cut short.
pub const UNREADABLE: &str =
    "This audio file could not be read; it may be damaged. Convert the file to WAV.";

/// Decodes any supported format, picking the decoder by magic bytes.
/// New formats are added here without touching the use cases (OCP).
#[derive(Debug, Clone, Copy, Default)]
pub struct AudioFileDecoder;

impl AudioDecoder for AudioFileDecoder {
    fn decode(&self, bytes: &[u8]) -> PortResult<AudioClip> {
        match AudioFormat::detect(bytes) {
            Some(AudioFormat::Wav) => WavDecoder.decode(bytes),
            Some(AudioFormat::Flac) => FlacDecoder.decode(bytes),
            Some(AudioFormat::Mp3 | AudioFormat::M4a | AudioFormat::Ogg) => {
                CompressedAudioDecoder.decode(bytes)
            }
            None => Err(PortError::new(UNSUPPORTED_FORMAT)),
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
        let err = AudioFileDecoder.decode(b"not audio at all...").unwrap_err();
        assert_eq!(err.0, UNSUPPORTED_FORMAT);
    }

    // Real files (≈3 s of voiced tone with pauses) made with ffmpeg:
    // MP3 = LAME 64 kb/s stereo 44.1 kHz with an ID3v2 tag; M4A = AAC-LC
    // mono 44.1 kHz; OGG = Vorbis mono 22.05 kHz.
    const MP3: &[u8] = include_bytes!("../../tests/fixtures/voice.mp3");
    const M4A: &[u8] = include_bytes!("../../tests/fixtures/voice.m4a");
    const OGG: &[u8] = include_bytes!("../../tests/fixtures/voice.ogg");
    /// Opus in an Ogg container: a container we read with a codec we do not.
    const OPUS: &[u8] = include_bytes!("../../tests/fixtures/opus.ogg");
    /// An M4A holding only a subtitle track.
    const NO_AUDIO: &[u8] = include_bytes!("../../tests/fixtures/no-audio.m4a");

    /// Converts to WAV and checks the result is the 3 s voiced recording.
    fn assert_converts_to_wav(bytes: &[u8], sample_rate: u32) {
        let wav = to_wav(bytes).unwrap();
        assert_eq!(AudioFormat::detect(&wav), Some(AudioFormat::Wav));
        let clip = WavDecoder.decode(&wav).unwrap();
        assert_eq!(clip.sample_rate(), sample_rate);
        let d = clip.duration_secs();
        assert!((d - 3.0).abs() < 0.1, "duration {d}");
        assert!(clip.rms() > 0.02, "rms {}", clip.rms());
    }

    #[test]
    fn converts_mp3_to_wav() {
        assert_eq!(AudioFormat::detect(MP3), Some(AudioFormat::Mp3));
        assert_converts_to_wav(MP3, 44_100);
    }

    #[test]
    fn converts_m4a_to_wav() {
        assert_eq!(AudioFormat::detect(M4A), Some(AudioFormat::M4a));
        assert_converts_to_wav(M4A, 44_100);
    }

    #[test]
    fn converts_ogg_vorbis_to_wav() {
        assert_eq!(AudioFormat::detect(OGG), Some(AudioFormat::Ogg));
        assert_converts_to_wav(OGG, 22_050);
    }

    #[test]
    fn explains_files_it_cannot_use() {
        assert_eq!(
            AudioFileDecoder.decode(NO_AUDIO).unwrap_err().0,
            NO_AUDIO_TRACK
        );
        assert_eq!(
            AudioFileDecoder.decode(OPUS).unwrap_err().0,
            UNSUPPORTED_CODEC
        );
        // A recognised container whose content is cut short.
        assert_eq!(
            AudioFileDecoder.decode(&M4A[..200]).unwrap_err().0,
            UNREADABLE
        );
    }

    #[test]
    fn converts_flac_to_wav() {
        let flac = flac::encode_flac_for_tests(&tone(), 1, 16, 16_000);
        let wav = to_wav(&flac).unwrap();
        assert_eq!(AudioFormat::detect(&wav), Some(AudioFormat::Wav));
        assert_eq!(WavDecoder.decode(&wav).unwrap().samples().len(), 8_000);
    }
}
