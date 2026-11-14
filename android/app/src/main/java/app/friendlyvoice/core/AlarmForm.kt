package app.friendlyvoice.core

import java.io.Serializable
import java.util.Locale

enum class RepeatKind { DAILY, WEEKLY, INTERVAL, ONCE }

/** Why a form cannot be saved yet. The UI maps each to a string. */
enum class FormError { NO_VOICE, NO_THEME, NO_DAYS, BAD_INTERVAL, NO_DATE }

sealed interface FormResult {
    data class Ok(val request: ScheduleRequest) : FormResult
    data class Invalid(val error: FormError) : FormResult
}

/**
 * Every choice on the Create alarm screen. Serializable so it survives process
 * death in the ViewModel's SavedStateHandle. Keeps `text` from an existing
 * schedule so editing never loses it.
 */
data class AlarmForm(
    val speakerId: String? = null,
    val category: String? = null,
    val text: String? = null,
    val tone: String? = null,
    val pitch: Int = 0,
    val kind: RepeatKind = RepeatKind.DAILY,
    val time: String = "08:00",
    val days: List<Weekday> = Weekday.WORKDAYS,
    val everyMinutes: Int? = 60,
    val windowFrom: String = "09:00",
    val windowTo: String = "18:00",
    val date: String = "",
    val quietEnabled: Boolean = false,
    val quietFrom: String = "22:00",
    val quietTo: String = "07:00",
) : Serializable {

    fun toRequest(): FormResult {
        val speaker = speakerId ?: return FormResult.Invalid(FormError.NO_VOICE)
        if (category == null && text.isNullOrBlank()) return FormResult.Invalid(FormError.NO_THEME)
        val repeat = when (kind) {
            RepeatKind.DAILY -> Repeat.Daily(time)
            RepeatKind.WEEKLY -> {
                if (days.isEmpty()) return FormResult.Invalid(FormError.NO_DAYS)
                Repeat.Weekly(days.distinct().sorted(), time)
            }
            RepeatKind.INTERVAL -> {
                val minutes = everyMinutes?.takeIf { it >= 1 } ?: return FormResult.Invalid(FormError.BAD_INTERVAL)
                Repeat.Interval(minutes, TimeWindow(windowFrom, windowTo))
            }
            RepeatKind.ONCE -> {
                if (date.isBlank()) return FormResult.Invalid(FormError.NO_DATE)
                Repeat.Once(date, time)
            }
        }
        return FormResult.Ok(
            ScheduleRequest(
                speakerId = speaker,
                repeat = repeat,
                quiet = if (quietEnabled) TimeWindow(quietFrom, quietTo) else null,
                category = category,
                text = text,
                tone = tone,
                pitchShift = pitch,
            ),
        )
    }

    companion object {
        /** Pre-fills the form from a saved schedule (tap on an alarm card). */
        fun from(r: ScheduleRequest): AlarmForm {
            val base = AlarmForm(
                speakerId = r.speakerId,
                category = r.category,
                text = r.text,
                tone = r.tone,
                pitch = r.pitchShift,
                quietEnabled = r.quiet != null,
                quietFrom = r.quiet?.from ?: "22:00",
                quietTo = r.quiet?.to ?: "07:00",
            )
            return when (val repeat = r.repeat) {
                is Repeat.Daily -> base.copy(kind = RepeatKind.DAILY, time = repeat.at)
                is Repeat.Weekly -> base.copy(kind = RepeatKind.WEEKLY, time = repeat.at, days = repeat.days)
                is Repeat.Interval -> base.copy(
                    kind = RepeatKind.INTERVAL,
                    everyMinutes = repeat.everyMinutes,
                    windowFrom = repeat.within?.from ?: base.windowFrom,
                    windowTo = repeat.within?.to ?: base.windowTo,
                )
                is Repeat.Once -> base.copy(kind = RepeatKind.ONCE, date = repeat.date, time = repeat.time)
            }
        }

        /** "HH:MM" from hour and minute. */
        fun hhmm(hour: Int, minute: Int): String = "%02d:%02d".format(Locale.ROOT, hour, minute)

        /** "HH:MM" → (hour, minute); falls back to 08:00. */
        fun parseHhmm(text: String): Pair<Int, Int> {
            val parts = text.split(':')
            val h = parts.getOrNull(0)?.toIntOrNull()?.takeIf { it in 0..23 } ?: 8
            val m = parts.getOrNull(1)?.toIntOrNull()?.takeIf { it in 0..59 } ?: 0
            return h to m
        }
    }
}
