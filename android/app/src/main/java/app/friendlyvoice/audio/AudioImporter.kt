package app.friendlyvoice.audio

import android.content.Context
import android.media.AudioFormat
import android.media.MediaCodec
import android.media.MediaExtractor
import android.media.MediaFormat
import android.net.Uri
import app.friendlyvoice.core.AudioKind
import app.friendlyvoice.core.Pcm
import app.friendlyvoice.core.ShortBuffer
import app.friendlyvoice.core.Wav
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.ensureActive
import kotlinx.coroutines.withContext
import java.io.IOException
import java.nio.ByteOrder
import kotlin.coroutines.coroutineContext

/**
 * Turns a picked audio file into bytes the core accepts. WAV and FLAC go as
 * they are (the core detects them by magic bytes); anything else (MP3…) is
 * decoded here with MediaCodec into a 16-bit mono PCM WAV. Runs on
 * Dispatchers.IO: never on the main thread.
 */
class AudioImporter(private val context: Context) {

    suspend fun load(uri: Uri): ByteArray = withContext(Dispatchers.IO) {
        val resolver = context.contentResolver
        val head = resolver.openInputStream(uri)?.use { input ->
            val buffer = ByteArray(12)
            val read = input.read(buffer)
            if (read <= 0) ByteArray(0) else buffer.copyOf(read)
        } ?: throw IOException("Could not open the file.")
        when (AudioKind.sniff(head)) {
            AudioKind.WAV, AudioKind.FLAC ->
                resolver.openInputStream(uri)?.use { it.readBytes() } ?: throw IOException("Could not open the file.")
            AudioKind.OTHER -> decodeToWav(uri)
        }
    }

    private suspend fun decodeToWav(uri: Uri): ByteArray {
        val extractor = MediaExtractor()
        try {
            extractor.setDataSource(context, uri, null)
            val track = (0 until extractor.trackCount).firstOrNull {
                extractor.getTrackFormat(it).getString(MediaFormat.KEY_MIME)?.startsWith("audio/") == true
            } ?: throw IOException("This file has no audio track. Use a WAV, MP3 or FLAC file.")
            extractor.selectTrack(track)
            val format = extractor.getTrackFormat(track)
            val mime = format.getString(MediaFormat.KEY_MIME) ?: throw IOException("Unknown audio format.")
            var sampleRate = format.getInteger(MediaFormat.KEY_SAMPLE_RATE)
            var channels = format.getInteger(MediaFormat.KEY_CHANNEL_COUNT)

            val codec = MediaCodec.createDecoderByType(mime)
            val mono = ShortBuffer()
            try {
                codec.configure(format, null, null, 0)
                codec.start()
                val info = MediaCodec.BufferInfo()
                var inputDone = false
                var outputDone = false
                while (!outputDone) {
                    coroutineContext.ensureActive()
                    if (!inputDone) {
                        val inIndex = codec.dequeueInputBuffer(TIMEOUT_US)
                        if (inIndex >= 0) {
                            val input = codec.getInputBuffer(inIndex) ?: throw IOException("Decoder error.")
                            val size = extractor.readSampleData(input, 0)
                            if (size < 0) {
                                codec.queueInputBuffer(inIndex, 0, 0, 0, MediaCodec.BUFFER_FLAG_END_OF_STREAM)
                                inputDone = true
                            } else {
                                codec.queueInputBuffer(inIndex, 0, size, extractor.sampleTime, 0)
                                extractor.advance()
                            }
                        }
                    }
                    val outIndex = codec.dequeueOutputBuffer(info, TIMEOUT_US)
                    if (outIndex == MediaCodec.INFO_OUTPUT_FORMAT_CHANGED) {
                        val out = codec.outputFormat
                        sampleRate = out.getInteger(MediaFormat.KEY_SAMPLE_RATE)
                        channels = out.getInteger(MediaFormat.KEY_CHANNEL_COUNT)
                        if (out.containsKey(MediaFormat.KEY_PCM_ENCODING) &&
                            out.getInteger(MediaFormat.KEY_PCM_ENCODING) != AudioFormat.ENCODING_PCM_16BIT
                        ) {
                            throw IOException("Unsupported audio encoding. Convert the file to WAV.")
                        }
                    } else if (outIndex >= 0) {
                        val output = codec.getOutputBuffer(outIndex)
                        if (output != null && info.size > 0) {
                            output.position(info.offset)
                            output.limit(info.offset + info.size)
                            val shorts = ShortArray(info.size / 2)
                            output.order(ByteOrder.LITTLE_ENDIAN).asShortBuffer().get(shorts)
                            mono.append(Pcm.toMono(shorts, channels))
                        }
                        codec.releaseOutputBuffer(outIndex, false)
                        if ((info.flags and MediaCodec.BUFFER_FLAG_END_OF_STREAM) != 0) outputDone = true
                        // Guard memory: the core refuses samples over 180 s anyway.
                        if (mono.size > sampleRate.toLong() * MAX_DECODE_SECS) {
                            throw IOException("This recording is longer than 180 seconds: split it into pieces.")
                        }
                    }
                }
                codec.stop()
            } finally {
                codec.release()
            }
            return Wav.encode(mono.toArray(), sampleRate)
        } finally {
            extractor.release()
        }
    }

    private companion object {
        const val TIMEOUT_US = 10_000L
        const val MAX_DECODE_SECS = 185
    }
}
