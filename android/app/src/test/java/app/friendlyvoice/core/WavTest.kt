package app.friendlyvoice.core

import org.junit.Assert.assertArrayEquals
import org.junit.Assert.assertEquals
import org.junit.Test

class WavTest {

    private fun le32(bytes: ByteArray, at: Int): Int =
        (bytes[at].toInt() and 0xFF) or
            ((bytes[at + 1].toInt() and 0xFF) shl 8) or
            ((bytes[at + 2].toInt() and 0xFF) shl 16) or
            ((bytes[at + 3].toInt() and 0xFF) shl 24)

    private fun le16(bytes: ByteArray, at: Int): Int =
        (bytes[at].toInt() and 0xFF) or ((bytes[at + 1].toInt() and 0xFF) shl 8)

    private fun ascii(bytes: ByteArray, at: Int, len: Int) = String(bytes, at, len, Charsets.US_ASCII)

    @Test
    fun `header describes 16-bit PCM with the right sizes`() {
        val header = Wav.header(dataBytes = 8, sampleRate = 44_100, channels = 1)

        assertEquals(44, header.size)
        assertEquals("RIFF", ascii(header, 0, 4))
        assertEquals(36 + 8, le32(header, 4))
        assertEquals("WAVE", ascii(header, 8, 4))
        assertEquals("fmt ", ascii(header, 12, 4))
        assertEquals(16, le32(header, 16))
        assertEquals(1, le16(header, 20)) // PCM
        assertEquals(1, le16(header, 22)) // channels
        assertEquals(44_100, le32(header, 24))
        assertEquals(88_200, le32(header, 28)) // byte rate
        assertEquals(2, le16(header, 32)) // block align
        assertEquals(16, le16(header, 34)) // bits per sample
        assertEquals("data", ascii(header, 36, 4))
        assertEquals(8, le32(header, 40))
    }

    @Test
    fun `encode writes the samples little-endian after the header`() {
        val wav = Wav.encode(shortArrayOf(1, -1, 0x1234, Short.MIN_VALUE), sampleRate = 16_000)

        assertEquals(44 + 8, wav.size)
        assertEquals(16_000, le32(wav, 24))
        assertArrayEquals(
            byteArrayOf(0x01, 0x00, 0xFF.toByte(), 0xFF.toByte(), 0x34, 0x12, 0x00, 0x80.toByte()),
            wav.copyOfRange(44, 52),
        )
    }

    @Test
    fun `stereo is mixed to mono by averaging each frame`() {
        val mono = Pcm.toMono(shortArrayOf(100, 300, -200, 200, 32_767, 32_767), channels = 2)

        assertArrayEquals(shortArrayOf(200, 0, 32_767), mono)
    }

    @Test
    fun `mono input is returned unchanged`() {
        val input = shortArrayOf(1, 2, 3)
        assertArrayEquals(input, Pcm.toMono(input, channels = 1))
    }

    @Test
    fun `incomplete trailing frames are dropped`() {
        assertArrayEquals(shortArrayOf(15), Pcm.toMono(shortArrayOf(10, 20, 30), channels = 2))
    }

    @Test
    fun `the format is sniffed from magic bytes, not the file name`() {
        val wav = Wav.encode(shortArrayOf(0), 16_000)
        assertEquals(AudioKind.WAV, AudioKind.sniff(wav))
        assertEquals(AudioKind.FLAC, AudioKind.sniff("fLaC\u0000\u0000".toByteArray(Charsets.US_ASCII)))
        assertEquals(AudioKind.OTHER, AudioKind.sniff("ID3\u0004\u0000\u0000".toByteArray(Charsets.US_ASCII)))
        assertEquals(AudioKind.OTHER, AudioKind.sniff(byteArrayOf(1, 2)))
    }
}
