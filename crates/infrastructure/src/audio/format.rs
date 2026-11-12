//! Audio container detection by magic bytes (never by file extension).

/// Audio formats the app can read.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioFormat {
    Wav,
    Flac,
}

impl AudioFormat {
    pub const SUPPORTED: &'static str = "WAV (PCM/float) ou FLAC";

    /// Detects the format of an in-memory file.
    pub fn detect(bytes: &[u8]) -> Option<Self> {
        if bytes.len() >= 12 && &bytes[..4] == b"RIFF" && &bytes[8..12] == b"WAVE" {
            return Some(Self::Wav);
        }
        if flac_stream(bytes).is_some() {
            return Some(Self::Flac);
        }
        None
    }

    /// File extension used when storing a file of this format.
    pub fn extension(self) -> &'static str {
        match self {
            Self::Wav => "wav",
            Self::Flac => "flac",
        }
    }
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
        assert_eq!(AudioFormat::detect(b"OggS\0\0\0\0\0\0\0\0"), None);
        assert_eq!(AudioFormat::detect(b""), None);
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
