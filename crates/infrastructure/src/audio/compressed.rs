//! MP3, M4A (AAC) and Ogg Vorbis decoding with `symphonia` (pure Rust).
use friendly_voice_application::ports::{AudioDecoder, DecodeError};
use friendly_voice_domain::audio::AudioClip;
use friendly_voice_domain::audio::SampleProblem::{NoAudioTrack, Unreadable, UnsupportedCodec};
use std::io::Cursor;
use symphonia::core::codecs::audio::AudioDecoderOptions;
use symphonia::core::codecs::CodecParameters;
use symphonia::core::errors::Error;
use symphonia::core::formats::probe::Hint;
use symphonia::core::formats::{FormatOptions, TrackType};
use symphonia::core::io::{MediaSourceStream, MediaSourceStreamOptions};
use symphonia::core::meta::MetadataOptions;

/// Decodes the compressed formats (container found by probing), mixing the
/// first audio track down to mono.
#[derive(Debug, Clone, Copy, Default)]
pub struct CompressedAudioDecoder;

impl AudioDecoder for CompressedAudioDecoder {
    fn decode(&self, bytes: &[u8]) -> Result<AudioClip, DecodeError> {
        let source = MediaSourceStream::new(
            Box::new(Cursor::new(bytes.to_vec())),
            MediaSourceStreamOptions::default(),
        );
        let mut reader = symphonia::default::get_probe()
            .probe(
                &Hint::new(),
                source,
                FormatOptions::default(),
                MetadataOptions::default(),
            )
            .map_err(|_| Unreadable)?;
        let track = reader.first_track(TrackType::Audio).ok_or(NoAudioTrack)?;
        let Some(CodecParameters::Audio(params)) = &track.codec_params else {
            return Err(UnsupportedCodec.into());
        };
        let track_id = track.id;
        let mut decoder = symphonia::default::get_codecs()
            .make_audio_decoder(params, &AudioDecoderOptions::default())
            .map_err(|_| UnsupportedCodec)?;

        let mut sample_rate = params.sample_rate;
        let mut mono = Vec::new();
        let mut interleaved: Vec<f32> = Vec::new();
        loop {
            let packet = match reader.next_packet() {
                Ok(Some(packet)) => packet,
                Ok(None) => break,
                // A file cut short at the end still plays up to the cut.
                Err(Error::IoError(e)) if e.kind() == std::io::ErrorKind::UnexpectedEof => break,
                Err(_) => return Err(Unreadable.into()),
            };
            if packet.track_id != track_id {
                continue;
            }
            let buffer = match decoder.decode(&packet) {
                Ok(buffer) => buffer,
                // A corrupted frame is skipped, as players do.
                Err(Error::DecodeError(_)) => continue,
                Err(_) => return Err(Unreadable.into()),
            };
            sample_rate = Some(buffer.spec().rate());
            let channels = buffer.spec().channels().count().max(1);
            buffer.copy_to_vec_interleaved(&mut interleaved);
            mono.extend(
                interleaved
                    .chunks(channels)
                    .map(|frame| frame.iter().sum::<f32>() / channels as f32),
            );
        }
        match sample_rate {
            Some(rate) if !mono.is_empty() => {
                AudioClip::new(rate, mono).map_err(|_| Unreadable.into())
            }
            _ => Err(Unreadable.into()),
        }
    }
}
