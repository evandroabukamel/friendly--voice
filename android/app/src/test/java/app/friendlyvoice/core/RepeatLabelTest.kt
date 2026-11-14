package app.friendlyvoice.core

import org.junit.Assert.assertEquals
import org.junit.Test

class RepeatLabelTest {

    private val words = RepeatWords(
        daily = "Daily",
        weekdays = "Weekdays",
        weekends = "Weekends",
        dayNames = mapOf(
            Weekday.MON to "Mon", Weekday.TUE to "Tue", Weekday.WED to "Wed", Weekday.THU to "Thu",
            Weekday.FRI to "Fri", Weekday.SAT to "Sat", Weekday.SUN to "Sun",
        ),
        everyMinutes = { n -> "Every $n min" },
        everyMinutesWithin = { n, from, to -> "Every $n min, $from–$to" },
        once = { date -> "Once, $date" },
    )

    private fun label(repeat: Repeat) = RepeatLabel.format(repeat, words)

    @Test
    fun `daily reads Daily`() = assertEquals("Daily", label(Repeat.Daily("08:00")))

    @Test
    fun `monday to friday reads Weekdays`() = assertEquals(
        "Weekdays",
        label(Repeat.Weekly(listOf(Weekday.MON, Weekday.TUE, Weekday.WED, Weekday.THU, Weekday.FRI), "08:00")),
    )

    @Test
    fun `saturday and sunday read Weekends`() =
        assertEquals("Weekends", label(Repeat.Weekly(listOf(Weekday.SUN, Weekday.SAT), "08:00")))

    @Test
    fun `all seven days read Daily`() = assertEquals("Daily", label(Repeat.Weekly(Weekday.values().toList(), "08:00")))

    @Test
    fun `other days are listed in week order`() =
        assertEquals("Mon, Wed, Fri", label(Repeat.Weekly(listOf(Weekday.FRI, Weekday.MON, Weekday.WED), "08:00")))

    @Test
    fun `interval with a window shows both`() =
        assertEquals("Every 30 min, 09:00–18:00", label(Repeat.Interval(30, TimeWindow("09:00", "18:00"))))

    @Test
    fun `interval without a window shows the minutes only`() =
        assertEquals("Every 90 min", label(Repeat.Interval(90, null)))

    @Test
    fun `once shows the day and month`() = assertEquals("Once, 20/11", label(Repeat.Once("2026-11-20", "06:45")))

    @Test
    fun `weekday lists from the core are parsed leniently`() = assertEquals(
        listOf(Weekday.MON, Weekday.WED, Weekday.SUN),
        Weekday.parseList(" sun, Monday,wed,xyz,mon"),
    )
}
