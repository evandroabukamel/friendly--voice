package app.friendlyvoice.ui

import android.content.Context
import android.widget.ArrayAdapter
import androidx.annotation.StringRes
import androidx.fragment.app.Fragment
import androidx.fragment.app.viewModels
import androidx.lifecycle.Lifecycle
import androidx.lifecycle.SavedStateHandle
import androidx.lifecycle.ViewModel
import androidx.lifecycle.ViewModelProvider
import androidx.lifecycle.createSavedStateHandle
import androidx.lifecycle.lifecycleScope
import androidx.lifecycle.repeatOnLifecycle
import androidx.lifecycle.viewmodel.initializer
import androidx.lifecycle.viewmodel.viewModelFactory
import app.friendlyvoice.FriendlyVoiceApp
import app.friendlyvoice.R
import app.friendlyvoice.audio.AudioError
import app.friendlyvoice.core.RepeatWords
import app.friendlyvoice.core.Weekday
import com.google.android.material.snackbar.Snackbar
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.launch

/** A message for the user, resolved to text by the screen (ViewModels hold no Context). */
sealed interface UiMessage {
    data class Res(@StringRes val id: Int, val args: List<Any> = emptyList()) : UiMessage
    data class Text(val text: String) : UiMessage

    companion object {
        /** The core's error messages are already clear and actionable: show them as they are. */
        fun error(t: Throwable): UiMessage =
            (t as? AudioError)?.let { Res(it.messageId) } ?: t.message?.takeIf { it.isNotBlank() }?.let(::Text) ?: Res(R.string.error_generic)
    }
}

fun Context.resolve(message: UiMessage): String = when (message) {
    is UiMessage.Res -> getString(message.id, *message.args.toTypedArray())
    is UiMessage.Text -> message.text
}

fun Fragment.showMessage(message: UiMessage) {
    val view = view ?: return
    Snackbar.make(view, requireContext().resolve(message), Snackbar.LENGTH_LONG).show()
}

/** Collects [flow] only while the view is at least STARTED. */
fun <T> Fragment.collectWhileStarted(flow: Flow<T>, block: suspend (T) -> Unit) {
    viewLifecycleOwner.lifecycleScope.launch {
        viewLifecycleOwner.repeatOnLifecycle(Lifecycle.State.STARTED) {
            flow.collect { block(it) }
        }
    }
}

/** ViewModel built with the app (repository, prefs) and a SavedStateHandle holding the nav arguments. */
inline fun <reified VM : ViewModel> Fragment.appViewModels(
    crossinline create: (FriendlyVoiceApp, SavedStateHandle) -> VM,
) = viewModels<VM>(
    factoryProducer = {
        viewModelFactory {
            initializer {
                val app = this[ViewModelProvider.AndroidViewModelFactory.APPLICATION_KEY] as FriendlyVoiceApp
                create(app, createSavedStateHandle())
            }
        }
    },
)

/** Dropdown adapter whose rows come from layout/atom_dropdown_item.xml. */
fun Context.dropdownAdapter(labels: List<String>): ArrayAdapter<String> =
    ArrayAdapter(this, R.layout.atom_dropdown_item, labels)

/** Repeat-label words from strings.xml. */
fun Context.repeatWords(): RepeatWords = RepeatWords(
    daily = getString(R.string.repeat_label_daily),
    weekdays = getString(R.string.repeat_label_weekdays),
    weekends = getString(R.string.repeat_label_weekends),
    dayNames = mapOf(
        Weekday.MON to getString(R.string.day_mon_abbr),
        Weekday.TUE to getString(R.string.day_tue_abbr),
        Weekday.WED to getString(R.string.day_wed_abbr),
        Weekday.THU to getString(R.string.day_thu_abbr),
        Weekday.FRI to getString(R.string.day_fri_abbr),
        Weekday.SAT to getString(R.string.day_sat_abbr),
        Weekday.SUN to getString(R.string.day_sun_abbr),
    ),
    everyMinutes = { n -> resources.getQuantityString(R.plurals.repeat_label_every, n, n) },
    everyMinutesWithin = { n, from, to ->
        getString(R.string.repeat_label_within, resources.getQuantityString(R.plurals.repeat_label_every, n, n), from, to)
    },
    once = { date -> getString(R.string.repeat_label_once, date) },
)
