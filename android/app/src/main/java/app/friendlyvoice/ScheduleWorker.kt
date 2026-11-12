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
import androidx.work.ExistingPeriodicWorkPolicy
import androidx.work.PeriodicWorkRequestBuilder
import androidx.work.WorkManager
import androidx.work.Worker
import androidx.work.WorkerParameters
import java.util.concurrent.CountDownLatch
import java.util.concurrent.TimeUnit

/**
 * Periodically fires the schedules that are due (user-defined rules and tones
 * live in the Rust core). Android limits periodic work to at least 15 minutes,
 * so that is the precision of the firings.
 */
class ScheduleWorker(context: Context, params: WorkerParameters) : Worker(context, params) {

    override fun doWork(): Result {
        val ctx = applicationContext
        if (!NativeBridge.configure(ctx).ok) return Result.retry()
        val res = NativeResult(NativeBridge.runDueSchedules(Prefs(ctx).listener))
        if (!res.ok) return Result.retry()
        val fired = res.arr()
        for (i in 0 until fired.length()) {
            val item = fired.getJSONObject(i)
            if (item.optBoolean("ok")) {
                notify(ctx, i, item.getString("text"), item.optString("tone"))
                play(item.getString("audio_path"))
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
            .setSmallIcon(android.R.drawable.ic_lock_silent_mode_off)
            .setContentTitle("A message for you 💛")
            .setContentText(text)
            .setStyle(NotificationCompat.BigTextStyle().bigText("$text\n(tone: $tone)"))
            .setContentIntent(open)
            .setAutoCancel(true)
            .build()
        NotificationManagerCompat.from(ctx).notify(1000 + id, notification)
    }

    /** Plays the audio and waits for it to finish (the worker must not end before). */
    private fun play(path: String) {
        val done = CountDownLatch(1)
        val player = MediaPlayer()
        try {
            player.setDataSource(path)
            player.setOnCompletionListener { done.countDown() }
            player.prepare()
            player.start()
            done.await(3, TimeUnit.MINUTES)
        } catch (e: Exception) {
            // No audio: the notification already delivered the text.
        } finally {
            player.release()
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
