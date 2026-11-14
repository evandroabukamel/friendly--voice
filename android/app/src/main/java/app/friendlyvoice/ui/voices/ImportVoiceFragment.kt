package app.friendlyvoice.ui.voices

import android.Manifest
import android.content.pm.PackageManager
import android.os.Bundle
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import androidx.activity.result.contract.ActivityResultContracts
import androidx.core.content.ContextCompat
import androidx.core.widget.doAfterTextChanged
import androidx.fragment.app.Fragment
import androidx.navigation.fragment.findNavController
import app.friendlyvoice.R
import app.friendlyvoice.audio.AudioImporter
import app.friendlyvoice.audio.VoiceRecorder
import app.friendlyvoice.core.Language
import app.friendlyvoice.databinding.FragmentImportVoiceBinding
import app.friendlyvoice.ui.UiMessage
import app.friendlyvoice.ui.appViewModels
import app.friendlyvoice.ui.collectWhileStarted
import app.friendlyvoice.ui.dropdownAdapter
import app.friendlyvoice.ui.showMessage
import com.google.android.material.dialog.MaterialAlertDialogBuilder
import java.io.File
import kotlin.math.roundToInt

/** Register a voice (with consent) and add recordings: files or the microphone. */
class ImportVoiceFragment : Fragment() {

    private val viewModel by appViewModels { app, saved ->
        ImportVoiceViewModel(
            repository = app.repository,
            importer = AudioImporter(app),
            recordingsDir = File(app.filesDir, "recordings"),
            defaultListenerName = app.prefs.listenerName,
            savedState = saved,
        )
    }
    private var binding: FragmentImportVoiceBinding? = null
    private var shownLanguages: List<Language>? = null
    private var formFilled = false

    private val pickFiles = registerForActivityResult(ActivityResultContracts.OpenMultipleDocuments()) { uris ->
        viewModel.importFiles(uris)
    }

    private val micPermission = registerForActivityResult(ActivityResultContracts.RequestPermission()) { granted ->
        if (granted) viewModel.toggleRecording() else showMessage(UiMessage.Res(R.string.record_permission_denied))
    }

    override fun onCreateView(inflater: LayoutInflater, container: ViewGroup?, state: Bundle?): View =
        FragmentImportVoiceBinding.inflate(inflater, container, false).also { binding = it }.root

    override fun onViewCreated(view: View, savedInstanceState: Bundle?) {
        val b = binding ?: return
        shownLanguages = null
        formFilled = false
        val samples = SampleAdapter()
        b.sampleList.adapter = samples

        b.header.backButton.setOnClickListener { findNavController().navigateUp() }
        b.nameInput.doAfterTextChanged { viewModel.setName(it?.toString().orEmpty()) }
        b.consentInput.doAfterTextChanged { viewModel.setConsent(it?.toString().orEmpty()) }
        b.listenerInput.doAfterTextChanged { viewModel.setListener(it?.toString().orEmpty()) }
        b.languageInput.setOnItemClickListener { _, _, position, _ ->
            shownLanguages?.getOrNull(position)?.let { viewModel.setLanguage(it.code) }
        }
        b.saveButton.setOnClickListener { viewModel.save() }
        b.sampleActions.addFilesButton.setOnClickListener { pickFiles.launch(arrayOf("audio/*")) }
        b.sampleActions.recordButton.setOnClickListener { onRecordTapped() }
        b.createVoiceButton.setOnClickListener { viewModel.createVoice() }
        b.deleteVoiceButton.setOnClickListener { confirmDelete() }

        collectWhileStarted(viewModel.state) { state ->
            render(state)
            samples.submitList(state.samples)
        }
        collectWhileStarted(viewModel.messages) { showMessage(it) }
        collectWhileStarted(viewModel.closed) { findNavController().navigateUp() }
    }

    /** Dialog text and buttons come from strings.xml; no view is built here. */
    private fun confirmDelete() {
        MaterialAlertDialogBuilder(requireContext())
            .setTitle(R.string.delete_voice_title)
            .setMessage(R.string.delete_voice_message)
            .setNegativeButton(R.string.action_cancel, null)
            .setPositiveButton(R.string.delete_voice_confirm) { _, _ -> viewModel.deleteVoice() }
            .show()
    }

    override fun onStop() {
        // Leaving the screen ends the recording and keeps what was recorded (not on rotation).
        if (!requireActivity().isChangingConfigurations) viewModel.stopRecording()
        super.onStop()
    }

    override fun onDestroyView() {
        binding = null
        super.onDestroyView()
    }

    private fun onRecordTapped() {
        val granted = ContextCompat.checkSelfPermission(requireContext(), Manifest.permission.RECORD_AUDIO) ==
            PackageManager.PERMISSION_GRANTED
        if (granted || viewModel.state.value.recording) {
            viewModel.toggleRecording()
        } else {
            micPermission.launch(Manifest.permission.RECORD_AUDIO)
        }
    }

    private fun render(state: ImportVoiceState) {
        val b = binding ?: return
        b.header.title.setText(if (state.registered) R.string.import_voice_title_edit else R.string.import_voice_title_new)

        if (shownLanguages != state.languages) {
            shownLanguages = state.languages
            b.languageInput.setAdapter(requireContext().dropdownAdapter(state.languages.map { it.name }))
        }
        b.languageInput.setText(state.languages.firstOrNull { it.code == state.language }?.name ?: state.language, false)

        // Text fields are filled once (after loading); then the user's typing drives the state.
        if (!formFilled && !state.loading) {
            formFilled = true
            b.nameInput.setText(state.name)
            b.consentInput.setText(state.consentBy)
            b.listenerInput.setText(state.listenerName)
        }
        // Name, consent and language are fixed once the voice is registered.
        b.nameLayout.isEnabled = !state.registered
        b.consentLayout.isEnabled = !state.registered
        b.languageLayout.isEnabled = !state.registered
        b.nameLayout.error = if (state.nameMissing) getString(R.string.voice_name_required) else null
        b.consentLayout.error = if (state.consentMissing) getString(R.string.voice_consent_required) else null
        b.saveButton.isEnabled = !state.saving && !state.loading

        val actions = b.sampleActions
        actions.addFilesButton.isEnabled = state.registered && !state.busy
        actions.recordButton.isEnabled = state.registered && (state.recording || !state.busy)
        actions.recordButton.setText(if (state.recording) R.string.samples_stop else R.string.samples_record)
        actions.recordButton.setIconResource(if (state.recording) R.drawable.ic_stop else R.drawable.ic_record)
        actions.recordingStatus.visibility = if (state.recording) View.VISIBLE else View.GONE
        actions.recordingStatus.text = getString(
            R.string.recording_status,
            clock(state.recordSeconds),
            clock(VoiceRecorder.MAX_SECONDS),
        )

        val secs = state.totalSecs.roundToInt()
        b.samplesSummary.text = if (!state.registered) {
            getString(R.string.samples_save_first)
        } else {
            getString(
                R.string.samples_count_and_secs,
                resources.getQuantityString(R.plurals.samples_count, state.samples.size, state.samples.size),
                resources.getQuantityString(R.plurals.seconds, secs, secs),
            )
        }
        b.createHint.text = getString(
            if (state.speaker?.voiceReady == true) R.string.create_voice_hint_ready else R.string.create_voice_hint,
        )
        b.deleteVoiceButton.visibility = if (state.registered) View.VISIBLE else View.GONE
        b.deleteVoiceButton.isEnabled = !state.busy
        b.createVoiceButton.isEnabled = state.canCreateVoice
        b.createVoiceButton.setText(if (state.building) R.string.create_voice_running else R.string.create_voice)
        b.progress.visibility = if (state.loading || state.busy && !state.recording) View.VISIBLE else View.GONE
    }

    private fun clock(seconds: Int) = getString(R.string.clock_format, seconds / 60, seconds % 60)
}
