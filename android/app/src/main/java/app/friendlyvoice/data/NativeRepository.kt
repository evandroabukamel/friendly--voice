package app.friendlyvoice.data

import app.friendlyvoice.NativeBridge
import app.friendlyvoice.core.Catalog
import app.friendlyvoice.core.FiredMessage
import app.friendlyvoice.core.NativeJson
import app.friendlyvoice.core.Sample
import app.friendlyvoice.core.Schedule
import app.friendlyvoice.core.ScheduleJson
import app.friendlyvoice.core.ScheduleRequest
import app.friendlyvoice.core.Speaker
import app.friendlyvoice.core.SpokenMessage
import app.friendlyvoice.core.VoiceSummary
import kotlinx.coroutines.CoroutineDispatcher
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.sync.Mutex
import kotlinx.coroutines.sync.withLock
import kotlinx.coroutines.withContext
import org.json.JSONArray
import org.json.JSONObject
import kotlin.coroutines.cancellation.CancellationException

/**
 * The only owner of the JNI bridge. Every call:
 * - runs on [io] (never the main thread: the core does file I/O, DSP and HTTP);
 * - is serialized by one [Mutex] (the core writes JSON files: never two writers,
 *   including the background ScheduleWorker, which uses this same instance);
 * - parses the JSON once, here, into core models; errors become [Result.failure]
 *   carrying the core's message.
 */
class NativeRepository(
    private val dataDir: String,
    private val prefs: Prefs,
    private val io: CoroutineDispatcher = Dispatchers.IO,
) {
    private val mutex = Mutex()
    private var configured = false // guarded by mutex

    @Volatile
    private var catalogCache: Catalog? = null

    private suspend fun <T> call(native: () -> String, parse: (Any?) -> T): Result<T> = withContext(io) {
        mutex.withLock {
            try {
                if (!configured) {
                    NativeJson.unwrap(NativeBridge.configure(dataDir, prefs.engineUrl))
                    configured = true
                }
                Result.success(parse(NativeJson.unwrap(native())))
            } catch (e: CancellationException) {
                throw e
            } catch (e: Throwable) {
                // Includes UnsatisfiedLinkError when the native library is missing.
                Result.failure(e)
            }
        }
    }

    private fun obj(data: Any?) = data as JSONObject
    private fun arr(data: Any?) = data as JSONArray

    /** Points the core at the (possibly new) voice-server URL from Settings. */
    suspend fun reconfigure(): Result<Unit> = withContext(io) {
        mutex.withLock {
            try {
                NativeJson.unwrap(NativeBridge.configure(dataDir, prefs.engineUrl))
                configured = true
                Result.success(Unit)
            } catch (e: CancellationException) {
                throw e
            } catch (e: Throwable) {
                Result.failure(e)
            }
        }
    }

    /** Loaded once per process and kept in memory: it never changes at runtime. */
    suspend fun catalog(): Result<Catalog> {
        catalogCache?.let { return Result.success(it) }
        return call(NativeBridge::catalog) { NativeJson.catalog(obj(it)) }
            .onSuccess { catalogCache = it }
    }

    suspend fun speakers(): Result<List<Speaker>> =
        call(NativeBridge::listSpeakers) { NativeJson.speakers(arr(it)) }

    /** Registers a person (consent is mandatory, ADR 0004). Returns the new speaker id. */
    suspend fun registerSpeaker(
        name: String,
        consentBy: String,
        language: String?,
        listenerName: String?,
    ): Result<String> = call({
        NativeBridge.registerSpeaker(name, null, consentBy, language, listenerName)
    }) { NativeJson.speakerId(obj(it)) }

    /** Deletes the person, samples and voice; their alarms are disabled but kept. */
    suspend fun deleteSpeaker(speakerId: String): Result<Unit> =
        call({ NativeBridge.deleteSpeaker(speakerId) }) { }

    suspend fun setListenerName(speakerId: String, listenerName: String?): Result<Unit> =
        call({ NativeBridge.setListenerName(speakerId, listenerName) }) { }

    suspend fun samples(speakerId: String): Result<List<Sample>> =
        call({ NativeBridge.listSamples(speakerId) }) { NativeJson.samples(arr(it)) }

    /** [audio] is WAV or FLAC bytes; the core detects the format by magic bytes. */
    suspend fun addSample(speakerId: String, audio: ByteArray): Result<Unit> =
        call({ NativeBridge.addSample(speakerId, audio, null) }) { }

    suspend fun buildVoice(speakerId: String): Result<VoiceSummary> =
        call({ NativeBridge.buildVoice(speakerId) }) { NativeJson.voice(obj(it)) }

    suspend fun speak(
        speakerId: String,
        category: String?,
        text: String?,
        tone: String?,
        pitchShift: Int,
    ): Result<SpokenMessage> = call({
        NativeBridge.speak(speakerId, category, text, tone, pitchShift.toFloat(), prefs.listenerName)
    }) { NativeJson.spoken(obj(it)) }

    suspend fun schedules(): Result<List<Schedule>> =
        call(NativeBridge::listSchedules) { NativeJson.schedules(arr(it)) }

    suspend fun createSchedule(request: ScheduleRequest): Result<Schedule> =
        call({ NativeBridge.createSchedule(ScheduleJson.toJson(request).toString()) }) { NativeJson.schedule(obj(it)) }

    suspend fun updateSchedule(id: String, request: ScheduleRequest): Result<Schedule> =
        call({ NativeBridge.updateSchedule(id, ScheduleJson.toJson(request).toString()) }) {
            NativeJson.schedule(obj(it))
        }

    suspend fun setScheduleEnabled(id: String, enabled: Boolean): Result<Schedule> =
        call({ NativeBridge.setScheduleEnabled(id, enabled) }) { NativeJson.schedule(obj(it)) }

    /** Fires the due schedules (ScheduleWorker). */
    suspend fun runDueSchedules(): Result<List<FiredMessage>> =
        call({ NativeBridge.runDueSchedules(prefs.listenerName) }) { NativeJson.fired(arr(it)) }
}
