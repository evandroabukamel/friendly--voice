package app.friendlyvoice.core

import java.io.Serializable

// Small Kotlin views of the JSON the Rust core returns. Parsed once, at the
// boundary (NativeJson); the UI never touches JSON. No Android types here.

data class Category(val slug: String, val defaultTone: String)

data class Tone(val slug: String, val description: String)

data class Language(val code: String, val name: String)

data class Catalog(
    val categories: List<Category>,
    val tones: List<Tone>,
    val maxPitchShift: Int,
    val languages: List<Language>,
)

data class Speaker(
    val id: String,
    val name: String,
    val language: String,
    val consentGrantedBy: String?,
    val listenerName: String?,
    val voiceReady: Boolean,
    val gender: String?,
    val sampleCount: Int,
    val totalSampleSecs: Double,
)

data class Sample(
    val id: String,
    val durationSecs: Double,
    val transcript: String?,
    val meanPitchHz: Double?,
)

data class VoiceSummary(
    val language: String,
    val gender: String?,
    val sampleCount: Int,
    val totalAudioSecs: Double,
)

data class SpokenMessage(val text: String, val tone: String, val language: String, val audioPath: String)

data class FiredMessage(val ok: Boolean, val text: String, val tone: String, val audioPath: String?)

data class Schedule(
    val id: String,
    val speakerId: String,
    /** Null when the voice was deleted. */
    val speakerName: String?,
    val enabled: Boolean,
    /** False when the alarm's voice was deleted or its consent revoked: pick another voice. */
    val voiceAvailable: Boolean,
    val nextDue: String?,
    val time: String?,
    val toneLabel: String,
    val request: ScheduleRequest,
)

enum class Weekday(val slug: String) {
    MON("mon"),
    TUE("tue"),
    WED("wed"),
    THU("thu"),
    FRI("fri"),
    SAT("sat"),
    SUN("sun"),
    ;

    companion object {
        val WORKDAYS = listOf(MON, TUE, WED, THU, FRI)
        val WEEKEND = listOf(SAT, SUN)

        /** "mon,wed,fri" (also "Monday", any case) → ordered, distinct days; unknown tokens are skipped. */
        fun parseList(text: String?): List<Weekday> =
            text.orEmpty()
                .split(',')
                .mapNotNull { token ->
                    val key = token.trim().lowercase().take(3)
                    values().firstOrNull { it.slug == key }
                }
                .distinct()
                .sorted()
    }
}

/** "HH:MM-HH:MM". */
data class TimeWindow(val from: String, val to: String) : Serializable {
    fun format(): String = "$from-$to"

    companion object {
        fun parse(text: String?): TimeWindow? {
            val parts = text?.trim()?.split('-') ?: return null
            if (parts.size != 2 || parts.any { it.isBlank() }) return null
            return TimeWindow(parts[0].trim(), parts[1].trim())
        }
    }
}

/** How an alarm repeats. Times are "HH:MM", dates "YYYY-MM-DD". */
sealed interface Repeat {
    data class Daily(val at: String) : Repeat
    data class Weekly(val days: List<Weekday>, val at: String) : Repeat
    data class Interval(val everyMinutes: Int, val within: TimeWindow?) : Repeat
    data class Once(val date: String, val time: String) : Repeat
}

/** What the core needs to create or update a schedule. */
data class ScheduleRequest(
    val speakerId: String,
    val repeat: Repeat,
    val quiet: TimeWindow?,
    val category: String?,
    val text: String?,
    val tone: String?,
    val pitchShift: Int,
)

/** An error message reported by the Rust core (already clear and actionable). */
class NativeException(message: String) : Exception(message)

/** "good-morning" → "Good morning". */
fun humanize(slug: String): String =
    slug.replace('-', ' ').replace('_', ' ').replaceFirstChar { it.uppercase() }
