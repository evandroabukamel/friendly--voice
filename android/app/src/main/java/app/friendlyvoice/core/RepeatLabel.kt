package app.friendlyvoice.core

/**
 * The localized words a repeat label is made of. The app fills it from
 * strings.xml; tests fill it with plain English.
 */
class RepeatWords(
    val daily: String,
    val weekdays: String,
    val weekends: String,
    val dayNames: Map<Weekday, String>,
    val everyMinutes: (Int) -> String,
    val everyMinutesWithin: (Int, String, String) -> String,
    val once: (String) -> String,
)

/** Short human label for how an alarm repeats ("Weekdays", "Mon, Wed, Fri"…). */
object RepeatLabel {

    fun format(repeat: Repeat, words: RepeatWords): String = when (repeat) {
        is Repeat.Daily -> words.daily
        is Repeat.Weekly -> weekly(repeat.days.distinct().sorted(), words)
        is Repeat.Interval -> repeat.within
            ?.let { words.everyMinutesWithin(repeat.everyMinutes, it.from, it.to) }
            ?: words.everyMinutes(repeat.everyMinutes)
        is Repeat.Once -> words.once(dayMonth(repeat.date))
    }

    private fun weekly(days: List<Weekday>, words: RepeatWords): String = when (days) {
        Weekday.values().toList() -> words.daily
        Weekday.WORKDAYS -> words.weekdays
        Weekday.WEEKEND -> words.weekends
        else -> days.joinToString(", ") { words.dayNames[it] ?: it.slug }
    }

    /** "2026-11-20" → "20/11" (the same style as the core's next_due). */
    fun dayMonth(isoDate: String): String {
        val parts = isoDate.split('-')
        return if (parts.size == 3) "${parts[2]}/${parts[1]}" else isoDate
    }
}
