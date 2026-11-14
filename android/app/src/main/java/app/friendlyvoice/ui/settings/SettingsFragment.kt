package app.friendlyvoice.ui.settings

import android.os.Bundle
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import androidx.fragment.app.Fragment
import app.friendlyvoice.databinding.FragmentSettingsBinding
import app.friendlyvoice.ui.appViewModels
import app.friendlyvoice.ui.collectWhileStarted
import app.friendlyvoice.ui.showMessage

/** Your name (default listener) and the voice-server URL. */
class SettingsFragment : Fragment() {

    private val viewModel by appViewModels { app, _ -> SettingsViewModel(app.prefs, app.repository) }
    private var binding: FragmentSettingsBinding? = null

    override fun onCreateView(inflater: LayoutInflater, container: ViewGroup?, state: Bundle?): View =
        FragmentSettingsBinding.inflate(inflater, container, false).also { binding = it }.root

    override fun onViewCreated(view: View, savedInstanceState: Bundle?) {
        val b = binding ?: return
        if (savedInstanceState == null) {
            b.listenerInput.setText(viewModel.initialListener)
            b.engineInput.setText(viewModel.initialEngineUrl)
        }
        b.saveButton.setOnClickListener {
            viewModel.save(b.listenerInput.text?.toString().orEmpty(), b.engineInput.text?.toString().orEmpty())
        }
        collectWhileStarted(viewModel.saving) { saving -> b.saveButton.isEnabled = !saving }
        collectWhileStarted(viewModel.messages) { showMessage(it) }
    }

    override fun onDestroyView() {
        binding = null
        super.onDestroyView()
    }
}
