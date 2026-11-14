package app.friendlyvoice.ui.settings

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import app.friendlyvoice.R
import app.friendlyvoice.core.SingleFlight
import app.friendlyvoice.data.NativeRepository
import app.friendlyvoice.data.Prefs
import app.friendlyvoice.ui.UiMessage
import kotlinx.coroutines.channels.Channel
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.receiveAsFlow

class SettingsViewModel(
    private val prefs: Prefs,
    private val repository: NativeRepository,
) : ViewModel() {

    val initialListener: String = prefs.listenerName.orEmpty()
    val initialEngineUrl: String = prefs.engineUrl.orEmpty()

    private val _messages = Channel<UiMessage>(Channel.BUFFERED)
    val messages: Flow<UiMessage> = _messages.receiveAsFlow()

    /** Save reconfigures the native library: one at a time. */
    private val saveFlight = SingleFlight()
    val saving: StateFlow<Boolean> = saveFlight.running

    fun save(listener: String, engineUrl: String) {
        saveFlight.launch(viewModelScope) {
            prefs.listenerName = listener
            prefs.engineUrl = engineUrl
            repository.reconfigure()
                .onSuccess { _messages.send(UiMessage.Res(R.string.settings_saved)) }
                .onFailure { _messages.send(UiMessage.error(it)) }
        }
    }
}
