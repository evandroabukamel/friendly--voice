package app.friendlyvoice.core

import org.json.JSONObject
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertNull
import org.junit.Assert.assertTrue
import org.junit.Test

class ScheduleJsonTest {

    private fun roundTrip(request: ScheduleRequest): ScheduleRequest {
        val json = JSONObject(ScheduleJson.toJson(request).toString())
        return ScheduleJson.parseRequest(json.getString("speaker_id"), json)
    }

    private fun request(repeat: Repeat, quiet: TimeWindow? = null) = ScheduleRequest(
        speakerId = "grandma",
        repeat = repeat,
        quiet = quiet,
        category = "good-morning",
        text = null,
        tone = "warm",
        pitchShift = -2,
    )

    @Test
    fun `daily request survives a round trip`() {
        val r = request(Repeat.Daily("08:00"))
        assertEquals(r, roundTrip(r))
    }

    @Test
    fun `weekly request survives a round trip with ordered days`() {
        val r = request(Repeat.Weekly(listOf(Weekday.MON, Weekday.WED, Weekday.FRI), "07:30"), TimeWindow("22:00", "07:00"))
        assertEquals(r, roundTrip(r))
    }

    @Test
    fun `interval request survives a round trip`() {
        val r = request(Repeat.Interval(30, TimeWindow("09:00", "18:00")))
        assertEquals(r, roundTrip(r))
    }

    @Test
    fun `once request survives a round trip`() {
        val r = request(Repeat.Once("2026-11-20", "06:45"))
        assertEquals(r, roundTrip(r))
    }

    @Test
    fun `json uses the core field names`() {
        val json = ScheduleJson.toJson(request(Repeat.Weekly(listOf(Weekday.FRI, Weekday.MON), "07:30"), TimeWindow("22:00", "07:00")))

        assertEquals("grandma", json.getString("speaker_id"))
        assertEquals("weekly", json.getString("kind"))
        assertEquals("07:30", json.getString("at"))
        assertEquals("mon,fri", json.getString("days"))
        assertEquals("22:00-07:00", json.getString("quiet"))
        assertEquals("good-morning", json.getString("category"))
        assertEquals("warm", json.getString("tone"))
        assertEquals(-2, json.getInt("pitch_shift"))
        assertFalse(json.has("text"))
        assertFalse(json.has("every_minutes"))
    }

    @Test
    fun `once is sent as date and time in one field`() {
        val json = ScheduleJson.toJson(request(Repeat.Once("2026-11-20", "06:45")))
        assertEquals("once", json.getString("kind"))
        assertEquals("2026-11-20 06:45", json.getString("at"))
    }

    @Test
    fun `nulls from the core are read as absent`() {
        val json = JSONObject(
            """{"kind":"interval","at":null,"days":null,"every_minutes":45,"within":null,
               "quiet":null,"category":null,"text":"Hi","tone":null,"pitch_shift":1.0}""",
        )
        val r = ScheduleJson.parseRequest("x", json)

        assertEquals(Repeat.Interval(45, null), r.repeat)
        assertNull(r.quiet)
        assertNull(r.category)
        assertNull(r.tone)
        assertEquals("Hi", r.text)
        assertEquals(1, r.pitchShift)
    }

    @Test
    fun `a schedule item is parsed with its request`() {
        val item = JSONObject(
            """{"id":"s1","speaker_id":"grandma","speaker_name":"Grandma","enabled":true,
               "next_due":"14/11 08:00","time":"08:00","tone_label":"warm",
               "request":{"kind":"daily","at":"08:00","category":"calm","pitch_shift":0}}""",
        )
        val s = NativeJson.schedule(item)

        assertEquals("s1", s.id)
        assertEquals("Grandma", s.speakerName)
        assertTrue(s.enabled)
        assertEquals("14/11 08:00", s.nextDue)
        assertEquals("08:00", s.time)
        assertEquals(Repeat.Daily("08:00"), s.request.repeat)
        assertEquals("grandma", s.request.speakerId)
    }

    @Test
    fun `an alarm whose voice was deleted is parsed as unavailable`() {
        val item = JSONObject(
            """{"id":"s2","speaker_id":"gone","speaker_name":null,"enabled":false,"voice_available":false,
               "next_due":null,"time":"07:00","tone_label":"warm",
               "request":{"speaker_id":"gone","kind":"daily","at":"07:00","category":"calm","pitch_shift":0}}""",
        )
        val s = NativeJson.schedule(item)

        assertFalse(s.voiceAvailable)
        assertNull(s.speakerName)
        assertFalse(s.enabled)
    }

    @Test
    fun `an item without voice_available counts as available`() {
        val item = JSONObject("""{"id":"s3","speaker_id":"g","speaker_name":"Grandma","request":{"kind":"daily","at":"08:00"}}""")
        assertTrue(NativeJson.schedule(item).voiceAvailable)
    }

    @Test
    fun `registering returns the id of the speaker object`() {
        val data = NativeJson.unwrap("""{"ok":true,"data":{"id":"grandma-lucia","name":"Grandma Lucia","language":"pt-BR"}}""")
        assertEquals("grandma-lucia", NativeJson.speakerId(data as JSONObject))
    }

    @Test
    fun `an error envelope becomes an exception with the core message`() {
        val error = runCatching { NativeJson.unwrap("""{"ok":false,"error":"sample too long: split it into pieces"}""") }
            .exceptionOrNull()

        assertTrue(error is NativeException)
        assertEquals("sample too long: split it into pieces", error?.message)
    }
}
