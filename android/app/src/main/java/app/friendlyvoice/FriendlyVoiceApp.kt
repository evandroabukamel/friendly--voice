package app.friendlyvoice

import android.app.Application
import app.friendlyvoice.data.NativeRepository
import app.friendlyvoice.data.Prefs

/** Holds the single [NativeRepository] shared by the screens and ScheduleWorker. */
class FriendlyVoiceApp : Application() {
    val prefs: Prefs by lazy { Prefs(this) }
    val repository: NativeRepository by lazy { NativeRepository(filesDir.absolutePath, prefs) }
}
