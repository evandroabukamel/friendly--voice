//! Audio container detection by magic bytes (never by file extension).

/// Audio formats the app can read.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioFormat {
    Wav,
    Flac,
    /// MPEG-1/2 Layer III, with or without an ID3v2 tag.
    Mp3,
    /// ISO base media (MP4/M4A), usually AAC.
    M4a,
    /// Ogg container, usually Vorbis.
    Ogg,
}

impl AudioFormat {
    /// Detects the format of an in-memory file.
    pub fn detect(bytes: &[u8]) -> Option<Self> {
        if bytes.len() >= 12 && &bytes[..4] == b"RIFF" && &bytes[8..12] == b"WAVE" {
            return Some(Self::Wav);
        }
        if flac_stream(bytes).is_some() {
            return Some(Self::Flac);
        }
        // An ID3v2 tag not followed by FLAC is an MP3 tag.
        if id3v2_len(bytes).is_some() || is_mpeg_audio_frame(bytes) {
            return Some(Self::Mp3);
        }
        if bytes.len() >= 8 && &bytes[4..8] == b"ftyp" {
            return Some(Self::M4a);
        }
        if bytes.starts_with(b"OggS") {
            return Some(Self::Ogg);
        }
        None
    }

    /// File extension used when storing a file of this format.
    pub fn extension(self) -> &'static str {
        match self {
            Self::Wav => "wav",
            Self::Flac => "flac",
            Self::Mp3 => "mp3",
            Self::M4a => "m4a",
            Self::Ogg => "ogg",
        }
    }
}

/// A valid MPEG audio frame header: 11-bit sync, a defined version and
/// layer (layer `00` is AAC ADTS), and no reserved bitrate or sample rate.
fn is_mpeg_audio_frame(bytes: &[u8]) -> bool {
    let [a, b, c, ..] = *bytes else {
        return false;
    };
    let sync = a == 0xFF && b & 0xE0 == 0xE0;
    let version = (b >> 3) & 0b11;
    let layer = (b >> 1) & 0b11;
    let bitrate = c >> 4;
    let sample_rate = (c >> 2) & 0b11;
    sync && version != 0b01 && layer != 0 && bitrate != 0b1111 && sample_rate != 0b11
}

/// Returns the FLAC stream (starting at `fLaC`), skipping an optional ID3v2
/// tag that some taggers prepend to FLAC files.
pub(crate) fn flac_stream(bytes: &[u8]) -> Option<&[u8]> {
    let start = id3v2_len(bytes).unwrap_or(0);
    let rest = bytes.get(start..)?;
    rest.starts_with(b"fLaC").then_some(rest)
}

/// Length of an ID3v2 tag at the start of the buffer (header + body + footer).
fn id3v2_len(bytes: &[u8]) -> Option<usize> {
    if bytes.len() < 10 || &bytes[..3] != b"ID3" {
        return None;
    }
    let size = bytes[6..10].iter().try_fold(0usize, |acc, &b| {
        (b < 0x80).then_some((acc << 7) | b as usize)
    })?;
    let footer = if bytes[5] & 0x10 != 0 { 10 } else { 0 };
    Some(10 + size + footer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_by_magic_bytes() {
        assert_eq!(
            AudioFormat::detect(b"RIFF\0\0\0\0WAVEfmt "),
            Some(AudioFormat::Wav)
        );
        assert_eq!(
            AudioFormat::detect(b"fLaC\0\0\0\x22"),
            Some(AudioFormat::Flac)
        );
        assert_eq!(
            AudioFormat::detect(b"OggS\0\x02\0\0\0\0\0\0"),
            Some(AudioFormat::Ogg)
        );
        assert_eq!(
            AudioFormat::detect(b"\0\0\0\x1cftypM4A \0\0\x02\0"),
            Some(AudioFormat::M4a)
        );
        // MPEG-1 Layer III frame header without an ID3 tag.
        assert_eq!(
            AudioFormat::detect(b"\xff\xfb\x90\x64\0\0\0\0"),
            Some(AudioFormat::Mp3)
        );
        assert_eq!(AudioFormat::detect(b""), None);
        assert_eq!(AudioFormat::detect(b"not audio at all"), None);
    }

    #[test]
    fn mpeg_sync_alone_is_not_enough() {
        // AAC ADTS (layer bits 00) shares the sync word with MPEG audio.
        assert_eq!(AudioFormat::detect(b"\xff\xf1\x50\x80\0\0\0\0"), None);
        // Reserved bitrate index 1111.
        assert_eq!(AudioFormat::detect(b"\xff\xfb\xf0\x64\0\0\0\0"), None);
        // Reserved sample-rate index 11.
        assert_eq!(AudioFormat::detect(b"\xff\xfb\x9c\x64\0\0\0\0"), None);
    }

    #[test]
    fn id3_tag_without_flac_is_mp3() {
        let mut bytes = b"ID3\x04\x00\x00\x00\x00\x00\x05".to_vec();
        bytes.extend_from_slice(b"12345\xff\xfb\x90\x64");
        assert_eq!(AudioFormat::detect(&bytes), Some(AudioFormat::Mp3));
    }

    #[test]
    fn extensions() {
        let all = [
            AudioFormat::Wav,
            AudioFormat::Flac,
            AudioFormat::Mp3,
            AudioFormat::M4a,
            AudioFormat::Ogg,
        ];
        let ext: Vec<_> = all.iter().map(|f| f.extension()).collect();
        assert_eq!(ext, ["wav", "flac", "mp3", "m4a", "ogg"]);
    }

    #[test]
    fn skips_id3_tag_before_flac() {
        // ID3v2.4 header with a 5-byte body (syncsafe size 0x00000005).
        let mut bytes = b"ID3\x04\x00\x00\x00\x00\x00\x05".to_vec();
        bytes.extend_from_slice(b"12345fLaC");
        assert_eq!(AudioFormat::detect(&bytes), Some(AudioFormat::Flac));
        assert_eq!(flac_stream(&bytes), Some(&b"fLaC"[..]));
    }
}
