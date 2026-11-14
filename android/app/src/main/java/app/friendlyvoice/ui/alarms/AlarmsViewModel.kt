package app.friendlyvoice.ui.alarms

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import app.friendlyvoice.core.Schedule
import app.friendlyvoice.core.ToggleCoalescer
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

data class AlarmsState(val loading: Boolean = true, val alarms: List<Schedule> = emptyList())

class AlarmsViewModel(private val repository: NativeRepository) : ViewModel() {

    private val _state = MutableStateFlow(AlarmsState())
    val state: StateFlow<AlarmsState> = _state.asStateFlow()

    private val _messages = Channel<UiMessage>(Channel.BUFFERED)
    val messages: Flow<UiMessage> = _messages.receiveAsFlow()

    /** Rapid ON/OFF taps on one card reach the core once, with the last state. */
    private val toggles = ToggleCoalescer(viewModelScope, TOGGLE_DEBOUNCE_MS) { id, enabled ->
        sendEnabled(id, enabled)
    }

    /** Called on resume only (no polling): reads the local schedules file. */
    fun refresh() {
        viewModelScope.launch {
            repository.schedules()
                .onSuccess { list ->
                    list.forEach { toggles.markPersisted(it.id, it.enabled) }
                    // A toggle still waiting for its debounce keeps what the user sees.
                    val shown = list.map { s -> toggles.pending(s.id)?.let { s.copy(enabled = it) } ?: s }
                    _state.value = AlarmsState(loading = false, alarms = shown)
                }
                .onFailure {
                    _state.update { s -> s.copy(loading = false) }
                    _messages.send(UiMessage.error(it))
                }
        }
    }

    fun onToggle(id: String, enabled: Boolean) {
        _state.update { s -> s.copy(alarms = s.alarms.map { if (it.id == id) it.copy(enabled = enabled) else it }) }
        toggles.request(id, enabled)
    }

    private suspend fun sendEnabled(id: String, enabled: Boolean): Boolean =
        repository.setScheduleEnabled(id, enabled).fold(
            onSuccess = { updated ->
                _state.update { s ->
                    s.copy(
                        alarms = s.alarms.map {
                            if (it.id != id) it else updated.copy(enabled = toggles.pending(id) ?: updated.enabled)
                        },
                    )
                }
                true
            },
            onFailure = { error ->
                // E.g. a one-time alarm whose date passed: show why, put the switch back.
                _messages.send(UiMessage.error(error))
                refresh()
                false
            },
        )

    private companion object {
        const val TOGGLE_DEBOUNCE_MS = 400L
    }
}
