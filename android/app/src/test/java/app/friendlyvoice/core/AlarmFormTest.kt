package app.friendlyvoice.core

import org.junit.Assert.assertEquals
import org.junit.Test

class AlarmFormTest {

    private val valid = AlarmForm(speakerId = "grandma", category = "good-morning")

    private fun invalid(form: AlarmForm) = (form.toRequest() as FormResult.Invalid).error

    @Test
    fun `a voice and a theme are required`() {
        assertEquals(FormError.NO_VOICE, invalid(valid.copy(speakerId = null)))
        assertEquals(FormError.NO_THEME, invalid(valid.copy(category = null)))
    }

    @Test
    fun `weekly needs at least one day`() =
        assertEquals(FormError.NO_DAYS, invalid(valid.copy(kind = RepeatKind.WEEKLY, days = emptyList())))

    @Test
    fun `interval needs at least one minute`() {
        assertEquals(FormError.BAD_INTERVAL, invalid(valid.copy(kind = RepeatKind.INTERVAL, everyMinutes = 0)))
        assertEquals(FormError.BAD_INTERVAL, invalid(valid.copy(kind = RepeatKind.INTERVAL, everyMinutes = null)))
    }

    @Test
    fun `once needs a date`() = assertEquals(FormError.NO_DATE, invalid(valid.copy(kind = RepeatKind.ONCE)))

    @Test
    fun `quiet hours are sent only when switched on`() {
        val off = (valid.toRequest() as FormResult.Ok).request
        val on = (valid.copy(quietEnabled = true).toRequest() as FormResult.Ok).request
        assertEquals(null, off.quiet)
        assertEquals(TimeWindow("22:00", "07:00"), on.quiet)
    }

    @Test
    fun `editing a saved schedule gives back the same request`() {
        val requests = listOf(
            ScheduleRequest("g", Repeat.Daily("06:30"), null, "calm", null, "warm", 2),
            ScheduleRequest("g", Repeat.Weekly(listOf(Weekday.TUE, Weekday.SAT), "07:00"), TimeWindow("23:00", "06:00"), "gratitude", null, null, 0),
            ScheduleRequest("g", Repeat.Interval(45, TimeWindow("10:00", "16:00")), null, "motivation", null, "cheerful", -3),
            ScheduleRequest("g", Repeat.Once("2026-12-24", "20:00"), null, "celebration", "Feliz Natal", null, 0),
        )
        for (r in requests) {
            assertEquals(r, (AlarmForm.from(r).toRequest() as FormResult.Ok).request)
        }
    }

    @Test
    fun `times are formatted and parsed as HH MM`() {
        assertEquals("07:05", AlarmForm.hhmm(7, 5))
        assertEquals(21 to 30, AlarmForm.parseHhmm("21:30"))
        assertEquals(8 to 0, AlarmForm.parseHhmm("bad"))
    }

    @Test
    fun `a new alarm starts with the first ready voice`() =
        assertEquals("a", AlarmForm().withSelectableVoice(listOf("a", "b"), editing = false).speakerId)

    @Test
    fun `editing keeps a voice that is still selectable`() =
        assertEquals("b", valid.copy(speakerId = "b").withSelectableVoice(listOf("a", "b"), editing = true).speakerId)

    @Test
    fun `editing an alarm whose voice is gone leaves the voice empty and blocks saving`() {
        val form = valid.copy(speakerId = "gone").withSelectableVoice(listOf("a"), editing = true)
        assertEquals(null, form.speakerId)
        assertEquals(FormError.NO_VOICE, invalid(form))
    }
}
