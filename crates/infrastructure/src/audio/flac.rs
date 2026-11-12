//! FLAC decoding with `claxon` (pure Rust).
use super::format::flac_stream;
use friendly_voice_application::ports::{AudioDecoder, PortError, PortResult};
use friendly_voice_domain::audio::AudioClip;
use std::io::Cursor;

/// Decodes FLAC (any bit depth and channel count), mixing down to mono.
#[derive(Debug, Clone, Copy, Default)]
pub struct FlacDecoder;

impl AudioDecoder for FlacDecoder {
    fn decode(&self, bytes: &[u8]) -> PortResult<AudioClip> {
        let stream = flac_stream(bytes).ok_or_else(|| PortError::new("invalid FLAC"))?;
        let mut reader = claxon::FlacReader::new(Cursor::new(stream))
            .map_err(|e| PortError::new(format!("invalid FLAC: {e}")))?;
        let info = reader.streaminfo();
        let channels = info.channels.max(1) as usize;
        let scale = (1i64 << (info.bits_per_sample - 1)) as f32;
        let mut mono = Vec::with_capacity(info.samples.unwrap_or(0) as usize / channels);
        let (mut acc, mut n) = (0.0f32, 0usize);
        for sample in reader.samples() {
            let sample = sample.map_err(|e| PortError::new(format!("corrupted FLAC: {e}")))?;
            acc += sample as f32 / scale;
            n += 1;
            if n == channels {
                mono.push(acc / channels as f32);
                acc = 0.0;
                n = 0;
            }
        }
        AudioClip::new(info.sample_rate, mono).map_err(PortError::new)
    }
}

/// Test helper: encodes interleaved integer samples as FLAC.
#[cfg(test)]
pub(crate) fn encode_flac_for_tests(
    samples: &[i32],
    channels: usize,
    bits_per_sample: usize,
    sample_rate: usize,
) -> Vec<u8> {
    use flacenc::component::BitRepr;
    use flacenc::error::Verify;
    let config = flacenc::config::Encoder::default().into_verified().unwrap();
    let source =
        flacenc::source::MemSource::from_samples(samples, channels, bits_per_sample, sample_rate);
    let stream = flacenc::encode_with_fixed_block_size(&config, source, config.block_size).unwrap();
    let mut sink = flacenc::bitsink::ByteSink::new();
    stream.write(&mut sink).unwrap();
    sink.as_slice().to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sine(n: usize, amplitude: f32) -> impl Iterator<Item = f32> {
        (0..n).map(move |i| amplitude * (i as f32 * 0.05).sin())
    }

    #[test]
    fn decodes_16_bit_mono() {
        let samples: Vec<i32> = sine(16_000, 0.5).map(|s| (s * 32767.0) as i32).collect();
        let clip = FlacDecoder
            .decode(&encode_flac_for_tests(&samples, 1, 16, 16_000))
            .unwrap();
        assert_eq!(clip.sample_rate(), 16_000);
        assert_eq!(clip.samples().len(), 16_000);
        assert!((clip.samples()[100] - (100.0f32 * 0.05).sin() * 0.5).abs() < 1e-3);
    }

    #[test]
    fn decodes_24_bit_stereo_mixing_to_mono() {
        // Left = signal, right = silence → mono is half the signal.
        let scale = (1 << 23) as f32 - 1.0;
        let samples: Vec<i32> = sine(4_410, 0.8)
            .flat_map(|s| [(s * scale) as i32, 0])
            .collect();
        let clip = FlacDecoder
            .decode(&encode_flac_for_tests(&samples, 2, 24, 44_100))
            .unwrap();
        assert_eq!(clip.sample_rate(), 44_100);
        assert_eq!(clip.samples().len(), 4_410);
        assert!((clip.samples()[200] - (200.0f32 * 0.05).sin() * 0.4).abs() < 1e-3);
    }

    #[test]
    fn rejects_garbage() {
        assert!(FlacDecoder.decode(b"fLaC not really").is_err());
        assert!(FlacDecoder.decode(b"RIFF").is_err());
    }
}
