package app.friendlyvoice.audio

import android.annotation.SuppressLint
import android.media.AudioFormat
import android.media.AudioRecord
import android.media.MediaRecorder
import app.friendlyvoice.R
import app.friendlyvoice.core.Wav
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.isActive
import kotlinx.coroutines.withContext
import java.io.File
import java.io.RandomAccessFile
import java.util.concurrent.atomic.AtomicBoolean

/**
 * Records the microphone (44.1 kHz, mono, 16-bit) straight into a WAV file on
 * Dispatchers.IO. Stops on [stop], on cancellation, or after [maxSeconds].
 * The AudioRecord is always released, and the WAV header always finalized.
 */
class VoiceRecorder(private val maxSeconds: Int = MAX_SECONDS) {

    private val stopRequested = AtomicBoolean(false)

    fun stop() = stopRequested.set(true)

    /**
     * Records into [target] and returns the recorded duration in seconds.
     * The caller must hold the RECORD_AUDIO permission.
     */
    @SuppressLint("MissingPermission")
    suspend fun record(target: File, onProgress: (seconds: Int) -> Unit): Double = withContext(Dispatchers.IO) {
        stopRequested.set(false)
        target.parentFile?.mkdirs()
        val minBuffer = AudioRecord.getMinBufferSize(SAMPLE_RATE, CHANNEL, ENCODING)
        if (minBuffer <= 0) throw AudioError(R.string.error_mic_unavailable)
        val bufferBytes = maxOf(minBuffer, SAMPLE_RATE / 5 * 2)
        val recorder = AudioRecord(MediaRecorder.AudioSource.MIC, SAMPLE_RATE, CHANNEL, ENCODING, bufferBytes)
        var dataBytes = 0L
        try {
            if (recorder.state != AudioRecord.STATE_INITIALIZED) {
                throw AudioError(R.string.error_mic_unavailable)
            }
            RandomAccessFile(target, "rw").use { file ->
                file.setLength(0)
                file.write(ByteArray(Wav.HEADER_BYTES)) // patched at the end
                val buffer = ByteArray(bufferBytes)
                val maxBytes = SAMPLE_RATE.toLong() * 2 * maxSeconds
                var lastSecond = -1
                recorder.startRecording()
                try {
                    while (coroutineContext.isActive && !stopRequested.get() && dataBytes < maxBytes) {
                        val wanted = minOf(buffer.size.toLong(), maxBytes - dataBytes).toInt()
                        val read = recorder.read(buffer, 0, wanted)
                        if (read < 0) throw AudioError(R.string.error_recording_failed)
                        file.write(buffer, 0, read)
                        dataBytes += read
                        val second = (dataBytes / (SAMPLE_RATE * 2)).toInt()
                        if (second != lastSecond) {
                            lastSecond = second
                            onProgress(second)
                        }
                    }
                } finally {
                    recorder.stop()
                    file.seek(0)
                    file.write(Wav.header(dataBytes.toInt(), SAMPLE_RATE, channels = 1))
                }
            }
        } finally {
            recorder.release()
        }
        dataBytes.toDouble() / (SAMPLE_RATE * 2)
    }

    companion object {
        const val SAMPLE_RATE = 44_100
        const val MAX_SECONDS = 180
        private const val CHANNEL = AudioFormat.CHANNEL_IN_MONO
        private const val ENCODING = AudioFormat.ENCODING_PCM_16BIT
    }
}
