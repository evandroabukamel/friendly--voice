package app.friendlyvoice.ui.alarms

import androidx.lifecycle.SavedStateHandle
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import app.friendlyvoice.R
import app.friendlyvoice.core.AlarmForm
import app.friendlyvoice.core.Catalog
import app.friendlyvoice.core.FormError
import app.friendlyvoice.core.FormResult
import app.friendlyvoice.core.RepeatKind
import app.friendlyvoice.core.SingleFlight
import app.friendlyvoice.core.Speaker
import app.friendlyvoice.core.Weekday
import app.friendlyvoice.data.NativeRepository
import app.friendlyvoice.ui.UiMessage
import kotlinx.coroutines.channels.Channel
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.receiveAsFlow
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch
import java.time.LocalDate

data class EditAlarmState(
    val loading: Boolean = true,
    val editing: Boolean = false,
    val catalog: Catalog? = null,
    /** Only voices that are ready to speak. */
    val voices: List<Speaker> = emptyList(),
    val form: AlarmForm = AlarmForm(),
    val saving: Boolean = false,
    val playing: Boolean = false,
)

sealed interface EditAlarmEvent {
    data class Message(val message: UiMessage) : EditAlarmEvent
    data class Play(val audioPath: String) : EditAlarmEvent
    data object Saved : EditAlarmEvent
}

/** The time fields a time picker can change. */
enum class TimeField { TIME, WINDOW_FROM, WINDOW_TO, QUIET_FROM, QUIET_TO }

class EditAlarmViewModel(
    private val repository: NativeRepository,
    private val savedState: SavedStateHandle,
) : ViewModel() {

    private val scheduleId: String? = savedState[ARG_SCHEDULE_ID]

    private val _state = MutableStateFlow(
        EditAlarmState(editing = scheduleId != null, form = savedState.get<AlarmForm>(KEY_FORM) ?: AlarmForm(date = LocalDate.now().toString())),
    )
    val state: StateFlow<EditAlarmState> = _state.asStateFlow()

    private val _events = Channel<EditAlarmEvent>(Channel.BUFFERED)
    val events: Flow<EditAlarmEvent> = _events.receiveAsFlow()

    /** Play now and Save touch the server: one request at a time, extra taps are ignored. */
    private val playFlight = SingleFlight()
    private val saveFlight = SingleFlight()

    init {
        load(restored = savedState.contains(KEY_FORM))
    }

    private fun load(restored: Boolean) {
        viewModelScope.launch {
            val catalog = repository.catalog().getOrElse { return@launch fail(it) }
            val voices = repository.speakers().getOrElse { return@launch fail(it) }.filter { it.voiceReady }
            var form = _state.value.form
            if (!restored && scheduleId != null) {
                val schedule = repository.schedules().getOrElse { return@launch fail(it) }
                    .firstOrNull { it.id == scheduleId }
                if (schedule != null) form = AlarmForm.from(schedule.request)
            }
            if (form.speakerId == null || voices.none { it.id == form.speakerId }) {
                form = form.copy(speakerId = voices.firstOrNull()?.id ?: form.speakerId)
            }
            if (form.category == null && form.text.isNullOrBlank()) {
                form = form.copy(category = catalog.categories.firstOrNull()?.slug)
            }
            setForm(form)
            _state.update { it.copy(loading = false, catalog = catalog, voices = voices) }
        }
    }

    private suspend fun fail(error: Throwable) {
        _state.update { it.copy(loading = false) }
        _events.send(EditAlarmEvent.Message(UiMessage.error(error)))
    }

    private fun setForm(form: AlarmForm) {
        savedState[KEY_FORM] = form
        _state.update { it.copy(form = form) }
    }

    private fun edit(change: (AlarmForm) -> AlarmForm) = setForm(change(_state.value.form))

    fun selectVoice(id: String) = edit { it.copy(speakerId = id) }
    fun selectCategory(slug: String) = edit { it.copy(category = slug) }

    /** null = "Suggested by the theme". */
    fun selectTone(slug: String?) = edit { it.copy(tone = slug) }
    fun setPitch(semitones: Int) = edit { it.copy(pitch = semitones) }
    fun setKind(kind: RepeatKind) = edit { it.copy(kind = kind) }
    fun setDay(day: Weekday, on: Boolean) = edit { f ->
        f.copy(days = if (on) (f.days + day).distinct().sorted() else f.days - day)
    }
    fun setEveryMinutes(minutes: Int?) = edit { it.copy(everyMinutes = minutes) }
    fun setDate(isoDate: String) = edit { it.copy(date = isoDate) }
    fun setQuiet(on: Boolean) = edit { it.copy(quietEnabled = on) }

    fun setTime(field: TimeField, hhmm: String) = edit {
        when (field) {
            TimeField.TIME -> it.copy(time = hhmm)
            TimeField.WINDOW_FROM -> it.copy(windowFrom = hhmm)
            TimeField.WINDOW_TO -> it.copy(windowTo = hhmm)
            TimeField.QUIET_FROM -> it.copy(quietFrom = hhmm)
            TimeField.QUIET_TO -> it.copy(quietTo = hhmm)
        }
    }

    fun timeOf(field: TimeField): String = _state.value.form.let {
        when (field) {
            TimeField.TIME -> it.time
            TimeField.WINDOW_FROM -> it.windowFrom
            TimeField.WINDOW_TO -> it.windowTo
            TimeField.QUIET_FROM -> it.quietFrom
            TimeField.QUIET_TO -> it.quietTo
        }
    }

    /** Speaks the message with the current choices (the core caches the audio). */
    fun playNow() {
        val form = _state.value.form
        val speakerId = form.speakerId ?: return message(R.string.form_error_no_voice)
        playFlight.launch(viewModelScope) {
            _state.update { it.copy(playing = true) }
            repository.speak(speakerId, form.category, form.text, form.tone, form.pitch)
                .onSuccess { _events.send(EditAlarmEvent.Play(it.audioPath)) }
                .onFailure { _events.send(EditAlarmEvent.Message(UiMessage.error(it))) }
            _state.update { it.copy(playing = false) }
        }
    }

    fun save() {
        val request = when (val result = _state.value.form.toRequest()) {
            is FormResult.Invalid -> return message(errorText(result.error))
            is FormResult.Ok -> result.request
        }
        saveFlight.launch(viewModelScope) {
            _state.update { it.copy(saving = true) }
            val id = scheduleId
            val outcome = if (id == null) repository.createSchedule(request) else repository.updateSchedule(id, request)
            outcome
                .onSuccess { _events.send(EditAlarmEvent.Saved) }
                .onFailure { _events.send(EditAlarmEvent.Message(UiMessage.error(it))) }
            _state.update { it.copy(saving = false) }
        }
    }

    private fun message(id: Int) {
        viewModelScope.launch { _events.send(EditAlarmEvent.Message(UiMessage.Res(id))) }
    }

    private fun errorText(error: FormError): Int = when (error) {
        FormError.NO_VOICE -> R.string.form_error_no_voice
        FormError.NO_THEME -> R.string.form_error_no_theme
        FormError.NO_DAYS -> R.string.form_error_no_days
        FormError.BAD_INTERVAL -> R.string.form_error_interval
        FormError.NO_DATE -> R.string.form_error_no_date
    }

    companion object {
        /** Navigation argument (nav_graph.xml): the schedule to edit, absent to create. */
        const val ARG_SCHEDULE_ID = "scheduleId"
        private const val KEY_FORM = "form"
    }
}
