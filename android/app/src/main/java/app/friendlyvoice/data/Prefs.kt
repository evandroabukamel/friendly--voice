package app.friendlyvoice.data

import android.content.Context

/** User settings: the default listener name and the voice-server URL. */
class Prefs(context: Context) {
    private val sp = context.getSharedPreferences("friendly_voice", Context.MODE_PRIVATE)

    var engineUrl: String?
        get() = sp.getString(KEY_ENGINE_URL, null)?.takeIf { it.isNotBlank() }
        set(v) = sp.edit().putString(KEY_ENGINE_URL, v?.trim()).apply()

    var listenerName: String?
        get() = sp.getString(KEY_LISTENER, null)?.takeIf { it.isNotBlank() }
        set(v) = sp.edit().putString(KEY_LISTENER, v?.trim()).apply()

    private companion object {
        // Same keys as the first app version, so existing settings are kept.
        const val KEY_ENGINE_URL = "engine_url"
        const val KEY_LISTENER = "listener"
    }
}
