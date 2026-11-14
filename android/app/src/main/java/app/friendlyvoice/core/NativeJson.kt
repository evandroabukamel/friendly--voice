package app.friendlyvoice.core

import org.json.JSONArray
import org.json.JSONObject

/** Reads a string field, treating a missing key, JSON null and blank as absent. */
internal fun JSONObject.str(key: String): String? =
    if (isNull(key)) null else optString(key).takeIf { it.isNotBlank() }

private inline fun <T> JSONArray.mapObjects(f: (JSONObject) -> T): List<T> =
    (0 until length()).map { f(getJSONObject(it)) }

/** Parses the bridge's JSON envelopes into [Models]. Pure: tested on the JVM. */
object NativeJson {

    /** `{"ok":true,"data":X}` → X (JSONObject, JSONArray, primitive or null); errors throw [NativeException]. */
    fun unwrap(json: String): Any? {
        val root = JSONObject(json)
        if (!root.optBoolean("ok", false)) {
            // No message: the UI shows its own translated generic error.
            throw NativeException(root.str("error").orEmpty())
        }
        return if (root.isNull("data")) null else root.get("data")
    }

    fun catalog(o: JSONObject) = Catalog(
        categories = o.getJSONArray("categories").mapObjects {
            Category(it.getString("slug"), it.str("default_tone") ?: "natural")
        },
        tones = o.getJSONArray("tones").mapObjects { Tone(it.getString("slug"), it.str("description").orEmpty()) },
        maxPitchShift = o.optInt("max_pitch_shift", 6),
        languages = o.optJSONArray("languages")?.mapObjects { Language(it.getString("code"), it.getString("name")) }
            ?: emptyList(),
    )

    fun speaker(o: JSONObject) = Speaker(
        id = o.getString("id"),
        name = o.str("name").orEmpty(),
        language = o.str("language") ?: "pt-BR",
        consentGrantedBy = o.str("consent_granted_by"),
        listenerName = o.str("listener_name"),
        voiceReady = o.optBoolean("voice_ready", false),
        gender = o.str("gender"),
        sampleCount = o.optInt("sample_count", 0),
        totalSampleSecs = o.optDouble("total_sample_secs", 0.0).takeUnless { it.isNaN() } ?: 0.0,
    )

    /** `registerSpeaker` returns the speaker object; the app needs its id. */
    fun speakerId(o: JSONObject): String = o.getString("id")

    fun speakers(a: JSONArray): List<Speaker> = a.mapObjects(::speaker)

    fun samples(a: JSONArray): List<Sample> = a.mapObjects {
        Sample(
            id = it.getString("id"),
            durationSecs = it.optDouble("duration_secs", 0.0),
            transcript = it.str("transcript"),
            meanPitchHz = if (it.isNull("mean_pitch_hz")) null else it.optDouble("mean_pitch_hz"),
        )
    }

    fun voice(o: JSONObject) = VoiceSummary(
        language = o.str("language").orEmpty(),
        gender = o.str("gender"),
        sampleCount = o.optInt("sample_count", 0),
        totalAudioSecs = o.optDouble("total_audio_secs", 0.0),
    )

    fun spoken(o: JSONObject) = SpokenMessage(
        text = o.str("text").orEmpty(),
        tone = o.str("tone").orEmpty(),
        language = o.str("language").orEmpty(),
        audioPath = o.getString("audio_path"),
    )

    fun fired(a: JSONArray): List<FiredMessage> = a.mapObjects {
        FiredMessage(
            ok = it.optBoolean("ok", false),
            text = it.str("text").orEmpty(),
            tone = it.str("tone").orEmpty(),
            audioPath = it.str("audio_path"),
        )
    }

    fun schedule(o: JSONObject): Schedule {
        val speakerId = o.str("speaker_id").orEmpty()
        val request = o.optJSONObject("request") ?: o
        return Schedule(
            id = o.getString("id"),
            speakerId = speakerId,
            speakerName = o.str("speaker_name"),
            enabled = o.optBoolean("enabled", true),
            voiceAvailable = o.optBoolean("voice_available", true),
            nextDue = o.str("next_due"),
            time = o.str("time"),
            toneLabel = o.str("tone_label").orEmpty(),
            request = ScheduleJson.parseRequest(speakerId, request),
        )
    }

    fun schedules(a: JSONArray): List<Schedule> = a.mapObjects(::schedule)
}

/** [ScheduleRequest] ⇄ the core's request JSON (`createSchedule` / `updateSchedule`). */
object ScheduleJson {

    fun toJson(r: ScheduleRequest): JSONObject = JSONObject().apply {
        put("speaker_id", r.speakerId)
        when (val repeat = r.repeat) {
            is Repeat.Daily -> {
                put("kind", "daily")
                put("at", repeat.at)
            }
            is Repeat.Weekly -> {
                put("kind", "weekly")
                put("at", repeat.at)
                put("days", repeat.days.distinct().sorted().joinToString(",") { it.slug })
            }
            is Repeat.Interval -> {
                put("kind", "interval")
                put("every_minutes", repeat.everyMinutes)
                repeat.within?.let { put("within", it.format()) }
            }
            is Repeat.Once -> {
                put("kind", "once")
                put("at", "${repeat.date} ${repeat.time}")
            }
        }
        r.quiet?.let { put("quiet", it.format()) }
        r.category?.let { put("category", it) }
        r.text?.let { put("text", it) }
        r.tone?.let { put("tone", it) }
        put("pitch_shift", r.pitchShift)
    }

    fun parseRequest(speakerId: String, o: JSONObject): ScheduleRequest {
        val at = o.str("at").orEmpty()
        val repeat = when (o.str("kind")) {
            "weekly", "semanal" -> Repeat.Weekly(Weekday.parseList(o.str("days")), at)
            "interval", "intervalo" -> Repeat.Interval(o.optInt("every_minutes", 60), TimeWindow.parse(o.str("within")))
            "once", "uma-vez" -> {
                val parts = at.split(' ', 'T')
                Repeat.Once(parts.getOrElse(0) { "" }, parts.getOrElse(1) { "" })
            }
            else -> Repeat.Daily(at)
        }
        return ScheduleRequest(
            speakerId = o.str("speaker_id") ?: speakerId,
            repeat = repeat,
            quiet = TimeWindow.parse(o.str("quiet")),
            category = o.str("category"),
            text = o.str("text"),
            tone = o.str("tone"),
            pitchShift = Math.round(o.optDouble("pitch_shift", 0.0)).toInt(),
        )
    }
}
