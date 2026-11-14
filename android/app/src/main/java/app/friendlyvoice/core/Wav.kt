package app.friendlyvoice.core

import java.nio.ByteBuffer
import java.nio.ByteOrder

/** 16-bit PCM WAV encoding. Pure Kotlin so it is tested on the JVM. */
object Wav {
    const val HEADER_BYTES = 44

    /** The 44-byte RIFF header for [dataBytes] bytes of 16-bit PCM. */
    fun header(dataBytes: Int, sampleRate: Int, channels: Int): ByteArray {
        val blockAlign = channels * 2
        return ByteBuffer.allocate(HEADER_BYTES).order(ByteOrder.LITTLE_ENDIAN).apply {
            put("RIFF".toByteArray(Charsets.US_ASCII))
            putInt(36 + dataBytes)
            put("WAVE".toByteArray(Charsets.US_ASCII))
            put("fmt ".toByteArray(Charsets.US_ASCII))
            putInt(16)
            putShort(1) // PCM
            putShort(channels.toShort())
            putInt(sampleRate)
            putInt(sampleRate * blockAlign)
            putShort(blockAlign.toShort())
            putShort(16)
            put("data".toByteArray(Charsets.US_ASCII))
            putInt(dataBytes)
        }.array()
    }

    /** A complete mono WAV file for [samples]. */
    fun encode(samples: ShortArray, sampleRate: Int): ByteArray {
        val data = samples.size * 2
        val buffer = ByteBuffer.allocate(HEADER_BYTES + data).order(ByteOrder.LITTLE_ENDIAN)
        buffer.put(header(data, sampleRate, channels = 1))
        for (s in samples) buffer.putShort(s)
        return buffer.array()
    }
}

/** PCM helpers. */
object Pcm {
    /** Interleaved frames → mono, averaging the channels of each frame. */
    fun toMono(interleaved: ShortArray, channels: Int): ShortArray {
        if (channels <= 1) return interleaved
        val frames = interleaved.size / channels
        return ShortArray(frames) { f ->
            var sum = 0
            for (c in 0 until channels) sum += interleaved[f * channels + c]
            (sum / channels).toShort()
        }
    }
}

/** A growable buffer of 16-bit samples (avoids boxing). */
class ShortBuffer(initialCapacity: Int = 1 shl 16) {
    private var data = ShortArray(initialCapacity)
    var size = 0
        private set

    fun append(samples: ShortArray) {
        if (size + samples.size > data.size) {
            data = data.copyOf(maxOf(data.size * 2, size + samples.size))
        }
        samples.copyInto(data, size)
        size += samples.size
    }

    fun toArray(): ShortArray = data.copyOf(size)
}

/** Audio container detected by magic bytes (the core does the same). */
enum class AudioKind {
    WAV,
    FLAC,
    OTHER,
    ;

    companion object {
        fun sniff(head: ByteArray): AudioKind {
            fun ascii(from: Int, len: Int) =
                if (head.size >= from + len) String(head, from, len, Charsets.US_ASCII) else ""
            return when {
                ascii(0, 4) == "RIFF" && ascii(8, 4) == "WAVE" -> WAV
                ascii(0, 4) == "fLaC" -> FLAC
                else -> OTHER
            }
        }
    }
}
