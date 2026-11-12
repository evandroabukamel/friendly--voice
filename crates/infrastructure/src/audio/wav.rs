use friendly_voice_application::ports::{AudioDecoder, PortError, PortResult};
use friendly_voice_domain::audio::AudioClip;
use std::io::Cursor;

/// Decodes PCM WAV (8/16/24/32-bit or float), mixing down to mono.
#[derive(Debug, Clone, Copy, Default)]
pub struct WavDecoder;

impl AudioDecoder for WavDecoder {
    fn decode(&self, bytes: &[u8]) -> PortResult<AudioClip> {
        let mut reader = hound::WavReader::new(Cursor::new(bytes))
            .map_err(|e| PortError::new(format!("WAV inválido (use .wav PCM): {e}")))?;
        let spec = reader.spec();
        let channels = spec.channels.max(1) as usize;
        let interleaved: Vec<f32> = match spec.sample_format {
            hound::SampleFormat::Float => reader
                .samples::<f32>()
                .collect::<Result<_, _>>()
                .map_err(PortError::new)?,
            hound::SampleFormat::Int => {
                let scale = (1i64 << (spec.bits_per_sample - 1)) as f32;
                reader
                    .samples::<i32>()
                    .map(|s| s.map(|v| v as f32 / scale))
                    .collect::<Result<_, _>>()
                    .map_err(PortError::new)?
            }
        };
        let mono = interleaved
            .chunks(channels)
            .map(|frame| frame.iter().sum::<f32>() / channels as f32)
            .collect();
        AudioClip::new(spec.sample_rate, mono).map_err(PortError::new)
    }
}

/// Encodes a mono clip as 16-bit PCM WAV.
pub fn encode_wav(clip: &AudioClip) -> PortResult<Vec<u8>> {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: clip.sample_rate(),
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut cursor = Cursor::new(Vec::new());
    {
        let mut w = hound::WavWriter::new(&mut cursor, spec).map_err(PortError::new)?;
        for s in clip.samples() {
            w.write_sample((s.clamp(-1.0, 1.0) * i16::MAX as f32) as i16)
                .map_err(PortError::new)?;
        }
        w.finalize().map_err(PortError::new)?;
    }
    Ok(cursor.into_inner())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let clip = AudioClip::new(16_000, vec![0.0, 0.5, -0.5, 0.25]).unwrap();
        let back = WavDecoder.decode(&encode_wav(&clip).unwrap()).unwrap();
        assert_eq!(back.sample_rate(), 16_000);
        assert!((back.samples()[1] - 0.5).abs() < 1e-3);
    }

    #[test]
    fn rejects_garbage() {
        assert!(WavDecoder.decode(b"not a wav").is_err());
    }
}
