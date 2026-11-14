package app.friendlyvoice

import android.Manifest
import android.app.NotificationChannel
import android.app.NotificationManager
import android.app.PendingIntent
import android.content.Context
import android.content.Intent
import android.content.pm.PackageManager
import android.media.MediaPlayer
import android.os.Build
import androidx.core.app.NotificationCompat
import androidx.core.app.NotificationManagerCompat
import androidx.core.content.ContextCompat
import androidx.work.CoroutineWorker
import androidx.work.ExistingPeriodicWorkPolicy
import androidx.work.PeriodicWorkRequestBuilder
import androidx.work.WorkManager
import androidx.work.WorkerParameters
import app.friendlyvoice.ui.MainActivity
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import java.util.concurrent.CountDownLatch
import java.util.concurrent.TimeUnit

/**
 * Periodically fires the schedules that are due (user-defined rules and tones
 * live in the Rust core). Android limits periodic work to at least 15 minutes,
 * so that is the precision of the firings. Goes through the app's single
 * NativeRepository, so it never writes the core's files at the same time as
 * the screens.
 */
class ScheduleWorker(context: Context, params: WorkerParameters) : CoroutineWorker(context, params) {

    override suspend fun doWork(): Result {
        val ctx = applicationContext
        val repository = (ctx as FriendlyVoiceApp).repository
        val fired = repository.runDueSchedules().getOrElse { return Result.retry() }
        fired.forEachIndexed { i, message ->
            if (message.ok) {
                notify(ctx, i, message.text, message.tone)
                message.audioPath?.let { play(it) }
            }
        }
        return Result.success()
    }

    private fun notify(ctx: Context, id: Int, text: String, tone: String) {
        val granted = Build.VERSION.SDK_INT < 33 ||
            ContextCompat.checkSelfPermission(ctx, Manifest.permission.POST_NOTIFICATIONS) ==
            PackageManager.PERMISSION_GRANTED
        if (!granted) return
        val manager = ctx.getSystemService(NotificationManager::class.java)
        manager.createNotificationChannel(
            NotificationChannel(CHANNEL, ctx.getString(R.string.channel_name), NotificationManager.IMPORTANCE_DEFAULT),
        )
        val open = PendingIntent.getActivity(
            ctx,
            0,
            Intent(ctx, MainActivity::class.java),
            PendingIntent.FLAG_IMMUTABLE,
        )
        val notification = NotificationCompat.Builder(ctx, CHANNEL)
            .setSmallIcon(R.drawable.ic_notification)
            .setContentTitle(ctx.getString(R.string.notification_title))
            .setContentText(text)
            .setStyle(NotificationCompat.BigTextStyle().bigText(ctx.getString(R.string.notification_body, text, tone)))
            .setContentIntent(open)
            .setAutoCancel(true)
            .build()
        NotificationManagerCompat.from(ctx).notify(1000 + id, notification)
    }

    /** Plays the audio and waits for it to finish (the worker must not end before). */
    private suspend fun play(path: String) {
        withContext(Dispatchers.IO) {
            val done = CountDownLatch(1)
            val player = MediaPlayer()
            try {
                player.setDataSource(path)
                player.setOnCompletionListener { done.countDown() }
                player.setOnErrorListener { _, _, _ ->
                    done.countDown()
                    true
                }
                player.prepare()
                player.start()
                done.await(3, TimeUnit.MINUTES)
            } catch (e: Exception) {
                // No audio: the notification already delivered the text.
            } finally {
                player.release()
            }
        }
    }

    companion object {
        private const val CHANNEL = "scheduled_messages"
        private const val WORK_NAME = "friendly-voice-schedules"

        fun ensureScheduled(context: Context) {
            val request = PeriodicWorkRequestBuilder<ScheduleWorker>(15, TimeUnit.MINUTES).build()
            WorkManager.getInstance(context)
                .enqueueUniquePeriodicWork(WORK_NAME, ExistingPeriodicWorkPolicy.KEEP, request)
        }
    }
}
