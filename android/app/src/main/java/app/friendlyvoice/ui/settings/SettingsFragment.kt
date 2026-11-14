package app.friendlyvoice.ui.settings

import android.os.Bundle
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import androidx.appcompat.app.AppCompatDelegate
import androidx.core.os.LocaleListCompat
import androidx.fragment.app.Fragment
import app.friendlyvoice.R
import app.friendlyvoice.core.AppLanguage
import app.friendlyvoice.databinding.FragmentSettingsBinding
import app.friendlyvoice.ui.appViewModels
import app.friendlyvoice.ui.collectWhileStarted
import app.friendlyvoice.ui.dropdownAdapter
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
        bindLanguagePicker(b)
        collectWhileStarted(viewModel.saving) { saving -> b.saveButton.isEnabled = !saving }
        collectWhileStarted(viewModel.messages) { showMessage(it) }
    }

    /**
     * App language: "System default" or one of the arrays in values/arrays.xml.
     * Applied at once through AppCompat per-app locales (stored by AndroidX
     * below Android 13; the activity is recreated in the new language).
     */
    private fun bindLanguagePicker(b: FragmentSettingsBinding) {
        val tags = resources.getStringArray(R.array.app_language_tags)
        val names = resources.getStringArray(R.array.app_language_names)
        val current = AppCompatDelegate.getApplicationLocales().toLanguageTags()
        val index = AppLanguage.indexOf(current, tags.toList())
        b.languageInput.setAdapter(requireContext().dropdownAdapter(names.toList()))
        b.languageInput.setText(names[index], false)
        b.languageInput.setOnItemClickListener { _, _, position, _ ->
            val tag = tags[position]
            if (position != index) {
                AppCompatDelegate.setApplicationLocales(LocaleListCompat.forLanguageTags(tag))
            }
        }
    }

    override fun onDestroyView() {
        binding = null
        super.onDestroyView()
    }
}
