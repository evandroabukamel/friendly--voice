package app.friendlyvoice.ui.voices

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import app.friendlyvoice.core.Language
import app.friendlyvoice.core.Speaker
import app.friendlyvoice.data.NativeRepository
import app.friendlyvoice.ui.UiMessage
import kotlinx.coroutines.channels.Channel
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.receiveAsFlow
import kotlinx.coroutines.launch

data class VoicesState(
    val loading: Boolean = true,
    val voices: List<Speaker> = emptyList(),
    val languages: List<Language> = emptyList(),
)

class VoicesViewModel(private val repository: NativeRepository) : ViewModel() {

    private val _state = MutableStateFlow(VoicesState())
    val state: StateFlow<VoicesState> = _state.asStateFlow()

    private val _messages = Channel<UiMessage>(Channel.BUFFERED)
    val messages: Flow<UiMessage> = _messages.receiveAsFlow()

    /** Called on resume only (no polling). */
    fun refresh() {
        viewModelScope.launch {
            val languages = repository.catalog().map { it.languages }.getOrDefault(emptyList())
            repository.speakers()
                .onSuccess { _state.value = VoicesState(loading = false, voices = it, languages = languages) }
                .onFailure {
                    _state.value = _state.value.copy(loading = false)
                    _messages.send(UiMessage.error(it))
                }
        }
    }
}
