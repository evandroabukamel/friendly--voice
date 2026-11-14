package app.friendlyvoice

/**
 * Functions of the Rust library (crates/mobile), resolved by name
 * (Java_app_friendlyvoice_NativeBridge_*). Every response is JSON:
 * `{"ok": true, "data": ...}` or `{"ok": false, "error": "..."}`.
 *
 * Only [app.friendlyvoice.data.NativeRepository] calls these, off the main
 * thread and one at a time. Keep every declaration in this file so a renamed
 * native function is a one-line fix.
 */
object NativeBridge {
    init {
        System.loadLibrary("friendly_voice_mobile")
    }

    @JvmStatic external fun configure(dataDir: String, engineUrl: String?): String

    @JvmStatic external fun catalog(): String

    @JvmStatic external fun listSpeakers(): String

    @JvmStatic external fun registerSpeaker(
        name: String,
        relationship: String?,
        consentBy: String?,
        language: String?,
        listenerName: String?,
    ): String

    @JvmStatic external fun setListenerName(speakerId: String, listenerName: String?): String

    @JvmStatic external fun listSamples(speakerId: String): String

    @JvmStatic external fun addSample(speakerId: String, wav: ByteArray, transcript: String?): String

    @JvmStatic external fun buildVoice(speakerId: String): String

    @JvmStatic external fun speak(
        speakerId: String,
        category: String?,
        text: String?,
        tone: String?,
        pitchShift: Float,
        listener: String?,
    ): String

    @JvmStatic external fun createSchedule(requestJson: String): String

    @JvmStatic external fun updateSchedule(id: String, requestJson: String): String

    @JvmStatic external fun setScheduleEnabled(id: String, enabled: Boolean): String

    @JvmStatic external fun listSchedules(): String

    @JvmStatic external fun removeSchedule(id: String): String

    @JvmStatic external fun runDueSchedules(listener: String?): String
}
