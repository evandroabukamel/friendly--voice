package app.friendlyvoice.audio

import android.media.AudioAttributes
import android.media.MediaPlayer
import androidx.lifecycle.DefaultLifecycleObserver
import androidx.lifecycle.LifecycleOwner

/**
 * Plays one generated message at a time. Prepares asynchronously (never blocks
 * the main thread) and releases the MediaPlayer when its lifecycle stops.
 */
class MessagePlayer(private val onError: () -> Unit) : DefaultLifecycleObserver {
    private var player: MediaPlayer? = null

    fun play(path: String) {
        release()
        val p = MediaPlayer()
        player = p
        try {
            p.setAudioAttributes(
                AudioAttributes.Builder()
                    .setUsage(AudioAttributes.USAGE_MEDIA)
                    .setContentType(AudioAttributes.CONTENT_TYPE_SPEECH)
                    .build(),
            )
            p.setDataSource(path)
            p.setOnPreparedListener { it.start() }
            p.setOnCompletionListener { mp -> if (player === mp) release() }
            p.setOnErrorListener { mp, _, _ ->
                if (player === mp) release()
                onError()
                true
            }
            p.prepareAsync()
        } catch (e: Exception) {
            release()
            onError()
        }
    }

    fun release() {
        player?.release()
        player = null
    }

    override fun onStop(owner: LifecycleOwner) = release()
}
