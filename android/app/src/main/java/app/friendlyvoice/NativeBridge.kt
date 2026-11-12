package app.friendlyvoice

import android.content.Context
import org.json.JSONArray
import org.json.JSONObject

/**
 * Functions of the Rust library (crates/mobile). Every response is JSON:
 * `{"ok": true, "data": ...}` or `{"ok": false, "error": "..."}`.
 */
object NativeBridge {
    init {
        System.loadLibrary("friendly_voice_mobile")
    }

    @JvmStatic external fun configure(dataDir: String, engineUrl: String?): String
    @JvmStatic external fun catalog(): String
    @JvmStatic external fun listSpeakers(): String
    @JvmStatic external fun registerSpeaker(name: String, relationship: String?, consentBy: String?): String
    @JvmStatic external fun addSample(speakerId: String, wav: ByteArray, transcript: String?, observedIpa: String?): String
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
    @JvmStatic external fun listSchedules(): String
    @JvmStatic external fun removeSchedule(id: String): String
    @JvmStatic external fun runDueSchedules(listener: String?): String

    /** Points the library at the app's private folder and the configured server. */
    fun configure(context: Context): NativeResult {
        val prefs = Prefs(context)
        return NativeResult(configure(context.filesDir.absolutePath, prefs.engineUrl))
    }
}

class NativeResult(json: String) {
    private val root = JSONObject(json)
    val ok: Boolean = root.optBoolean("ok", false)
    val error: String = root.optString("error", "")

    fun obj(): JSONObject = root.getJSONObject("data")
    fun arr(): JSONArray = root.getJSONArray("data")
}

/** Simple user preferences. */
class Prefs(context: Context) {
    private val sp = context.getSharedPreferences("friendly_voice", Context.MODE_PRIVATE)

    var engineUrl: String?
        get() = sp.getString("engine_url", null)?.takeIf { it.isNotBlank() }
        set(v) = sp.edit().putString("engine_url", v).apply()

    var listener: String?
        get() = sp.getString("listener", null)?.takeIf { it.isNotBlank() }
        set(v) = sp.edit().putString("listener", v).apply()
}
