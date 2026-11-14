package app.friendlyvoice.ui.voices

import android.net.Uri
import androidx.lifecycle.SavedStateHandle
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import app.friendlyvoice.R
import app.friendlyvoice.audio.AudioImporter
import app.friendlyvoice.audio.VoiceRecorder
import app.friendlyvoice.core.Language
import app.friendlyvoice.core.LanguageMatch
import app.friendlyvoice.core.Sample
import app.friendlyvoice.core.SingleFlight
import app.friendlyvoice.core.Speaker
import app.friendlyvoice.data.NativeRepository
import app.friendlyvoice.ui.UiMessage
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.Job
import kotlinx.coroutines.channels.Channel
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.receiveAsFlow
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import java.io.File
import kotlin.coroutines.cancellation.CancellationException

data class ImportVoiceState(
    val loading: Boolean = true,
    /** Null until the voice is registered. */
    val speaker: Speaker? = null,
    val languages: List<Language> = emptyList(),
    val samples: List<Sample> = emptyList(),
    val name: String = "",
    val consentBy: String = "",
    val language: String = DEFAULT_LANGUAGE,
    val listenerName: String = "",
    val nameMissing: Boolean = false,
    val consentMissing: Boolean = false,
    val saving: Boolean = false,
    val building: Boolean = false,
    val importing: Boolean = false,
    val recording: Boolean = false,
    val recordSeconds: Int = 0,
) {
    val registered: Boolean get() = speaker != null
    val totalSecs: Double get() = samples.sumOf { it.durationSecs }
    val busy: Boolean get() = saving || building || importing || recording

    /** "Create voice" needs >= 10 s of recordings and no request running. */
    val canCreateVoice: Boolean get() = registered && totalSecs >= MIN_VOICE_SECS && !busy

    companion object {
        const val DEFAULT_LANGUAGE = "pt-BR"
        const val MIN_VOICE_SECS = 10.0
    }
}

class ImportVoiceViewModel(
    private val repository: NativeRepository,
    private val importer: AudioImporter,
    private val recordingsDir: File,
    defaultListenerName: String?,
    /** The app's current locale (BCP 47), used to pre-select a new voice's language. */
    private val appLocaleTag: String,
    private val savedState: SavedStateHandle,
) : ViewModel() {

    private val _state = MutableStateFlow(
        ImportVoiceState(
            name = savedState.get<String>(KEY_NAME) ?: "",
            consentBy = savedState.get<String>(KEY_CONSENT) ?: "",
            language = savedState.get<String>(KEY_LANGUAGE) ?: ImportVoiceState.DEFAULT_LANGUAGE,
            listenerName = savedState.get<String>(KEY_LISTENER) ?: defaultListenerName.orEmpty(),
        ),
    )
    val state: StateFlow<ImportVoiceState> = _state.asStateFlow()

    private val _messages = Channel<UiMessage>(Channel.BUFFERED)
    val messages: Flow<UiMessage> = _messages.receiveAsFlow()

    /** Emits once the voice is deleted: the screen goes back to the list. */
    private val _closed = Channel<Unit>(Channel.CONFLATED)
    val closed: Flow<Unit> = _closed.receiveAsFlow()

    /** Save and Create voice are single-flight: repeated taps never reach the core twice. */
    private val saveFlight = SingleFlight()
    private val buildFlight = SingleFlight()
    private val deleteFlight = SingleFlight()

    private var recorder: VoiceRecorder? = null
    private var recordJob: Job? = null

    private val speakerId: String?
        get() = savedState[ARG_SPEAKER_ID]

    init {
        load(fillForm = !savedState.contains(KEY_NAME))
    }

    private fun load(fillForm: Boolean) {
        viewModelScope.launch {
            val languages = repository.catalog().map { it.languages }.getOrDefault(emptyList())
            _state.update { it.copy(languages = languages) }
            val id = speakerId
            if (id == null && languages.isNotEmpty() && !savedState.contains(KEY_LANGUAGE)) {
                setLanguage(LanguageMatch.best(appLocaleTag, languages.map { it.code }))
            }
            if (id == null) {
                _state.update { it.copy(loading = false) }
                return@launch
            }
            reloadSpeaker(id, fillForm)
            _state.update { it.copy(loading = false) }
        }
    }

    private suspend fun reloadSpeaker(id: String, fillForm: Boolean) {
        val speaker = repository.speakers().getOrElse { return send(UiMessage.error(it)) }
            .firstOrNull { it.id == id }
        val samples = repository.samples(id).getOrElse { return send(UiMessage.error(it)) }
        _state.update { s ->
            val base = s.copy(speaker = speaker, samples = samples)
            if (!fillForm || speaker == null) {
                base
            } else {
                base.copy(
                    name = speaker.name,
                    consentBy = speaker.consentGrantedBy.orEmpty(),
                    language = speaker.language,
                    listenerName = speaker.listenerName.orEmpty(),
                )
            }
        }
    }

    private suspend fun send(message: UiMessage) = _messages.send(message)

    fun setName(v: String) = setField(KEY_NAME, v) { it.copy(name = v, nameMissing = false) }
    fun setConsent(v: String) = setField(KEY_CONSENT, v) { it.copy(consentBy = v, consentMissing = false) }
    fun setLanguage(code: String) = setField(KEY_LANGUAGE, code) { it.copy(language = code) }
    fun setListener(v: String) = setField(KEY_LISTENER, v) { it.copy(listenerName = v) }

    private fun setField(key: String, value: String, change: (ImportVoiceState) -> ImportVoiceState) {
        savedState[key] = value
        _state.update(change)
    }

    /** New voice: registers it once (consent required). Existing voice: saves "how this voice calls you". */
    fun save() {
        val s = _state.value
        val existing = s.speaker
        if (existing == null) {
            val nameMissing = s.name.isBlank()
            val consentMissing = s.consentBy.isBlank()
            if (nameMissing || consentMissing) {
                _state.update { it.copy(nameMissing = nameMissing, consentMissing = consentMissing) }
                return
            }
        }
        saveFlight.launch(viewModelScope) {
            _state.update { it.copy(saving = true) }
            val listener = s.listenerName.trim().ifBlank { null }
            if (existing == null) {
                repository.registerSpeaker(s.name.trim(), s.consentBy.trim(), s.language, listener)
                    .onSuccess { id ->
                        savedState[ARG_SPEAKER_ID] = id
                        reloadSpeaker(id, fillForm = false)
                        send(UiMessage.Res(R.string.voice_registered))
                    }
                    .onFailure { send(UiMessage.error(it)) }
            } else {
                repository.setListenerName(existing.id, listener)
                    .onSuccess { send(UiMessage.Res(R.string.voice_saved)) }
                    .onFailure { send(UiMessage.error(it)) }
            }
            _state.update { it.copy(saving = false) }
        }
    }

    /** Adds the picked files one by one (MP3 decoded off the main thread). */
    fun importFiles(uris: List<Uri>) {
        val id = speakerId ?: return
        if (uris.isEmpty() || _state.value.busy) return
        viewModelScope.launch {
            _state.update { it.copy(importing = true) }
            var added = 0
            for (uri in uris) {
                try {
                    val bytes = importer.load(uri)
                    repository.addSample(id, bytes)
                        .onSuccess { added++ }
                        .onFailure { send(UiMessage.error(it)) }
                } catch (e: CancellationException) {
                    throw e
                } catch (e: Exception) {
                    send(UiMessage.error(e))
                }
            }
            if (added > 0) send(UiMessage.Res(R.string.samples_added, listOf(added)))
            reloadSpeaker(id, fillForm = false)
            _state.update { it.copy(importing = false) }
        }
    }

    fun toggleRecording() {
        if (_state.value.recording) stopRecording() else startRecording()
    }

    /** Records into filesDir/recordings/<speakerId>/<timestamp>.wav, then adds it as a sample. */
    private fun startRecording() {
        val id = speakerId ?: return
        if (recordJob?.isActive == true || _state.value.busy) return
        val rec = VoiceRecorder()
        recorder = rec
        val file = File(recordingsFolder(id), "${System.currentTimeMillis()}.wav")
        recordJob = viewModelScope.launch {
            _state.update { it.copy(recording = true, recordSeconds = 0) }
            try {
                rec.record(file) { secs -> _state.update { it.copy(recordSeconds = secs) } }
                _state.update { it.copy(recording = false, importing = true) }
                val bytes = withContext(Dispatchers.IO) { file.readBytes() }
                repository.addSample(id, bytes)
                    .onSuccess { send(UiMessage.Res(R.string.samples_added, listOf(1))) }
                    .onFailure {
                        withContext(Dispatchers.IO) { file.delete() }
                        send(UiMessage.error(it))
                    }
                reloadSpeaker(id, fillForm = false)
            } catch (e: CancellationException) {
                throw e
            } catch (e: Exception) {
                send(UiMessage.error(e))
            } finally {
                recorder = null
                _state.update { it.copy(recording = false, importing = false) }
            }
        }
    }

    fun stopRecording() {
        recorder?.stop()
    }

    /** Regenerates the voice on the server from every recording. */
    fun createVoice() {
        val id = speakerId ?: return
        if (!_state.value.canCreateVoice) return
        buildFlight.launch(viewModelScope) {
            _state.update { it.copy(building = true) }
            repository.buildVoice(id)
                .onSuccess { send(UiMessage.Res(R.string.voice_created)) }
                .onFailure { send(UiMessage.error(it)) }
            reloadSpeaker(id, fillForm = false)
            _state.update { it.copy(building = false) }
        }
    }

    /** Deletes the voice in the core, then the app's own recordings folder (off the main thread). */
    fun deleteVoice() {
        val id = speakerId ?: return
        if (_state.value.busy) return
        deleteFlight.launch(viewModelScope) {
            _state.update { it.copy(saving = true) }
            repository.deleteSpeaker(id)
                .onSuccess {
                    withContext(Dispatchers.IO) { recordingsFolder(id).deleteRecursively() }
                    send(UiMessage.Res(R.string.voice_deleted))
                    _closed.send(Unit)
                }
                .onFailure { send(UiMessage.error(it)) }
            _state.update { it.copy(saving = false) }
        }
    }

    private fun recordingsFolder(id: String) = File(recordingsDir, id.replace(Regex("[^A-Za-z0-9_-]"), "_"))

    override fun onCleared() {
        recorder?.stop()
        super.onCleared()
    }

    companion object {
        /** Navigation argument (nav_graph.xml): the voice to open, absent for a new one. */
        const val ARG_SPEAKER_ID = "speakerId"
        private const val KEY_NAME = "name"
        private const val KEY_CONSENT = "consent"
        private const val KEY_LANGUAGE = "language"
        private const val KEY_LISTENER = "listener"
    }
}
