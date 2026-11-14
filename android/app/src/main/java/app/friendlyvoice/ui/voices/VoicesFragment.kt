package app.friendlyvoice.ui.voices

import android.os.Bundle
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import androidx.core.os.bundleOf
import androidx.fragment.app.Fragment
import androidx.navigation.fragment.findNavController
import app.friendlyvoice.R
import app.friendlyvoice.databinding.FragmentVoicesBinding
import app.friendlyvoice.ui.appViewModels
import app.friendlyvoice.ui.collectWhileStarted
import app.friendlyvoice.ui.showMessage

/** The list of voices. */
class VoicesFragment : Fragment() {

    private val viewModel by appViewModels { app, _ -> VoicesViewModel(app.repository) }
    private var binding: FragmentVoicesBinding? = null

    override fun onCreateView(inflater: LayoutInflater, container: ViewGroup?, state: Bundle?): View =
        FragmentVoicesBinding.inflate(inflater, container, false).also { binding = it }.root

    override fun onViewCreated(view: View, savedInstanceState: Bundle?) {
        val b = binding ?: return
        val adapter = VoiceAdapter(onOpen = { open(it.id) })
        b.voiceList.adapter = adapter
        b.addVoice.setOnClickListener { open(null) }

        collectWhileStarted(viewModel.state) { state ->
            adapter.languageNames = state.languages.associate { it.code to it.name }
            adapter.submitList(state.voices)
            b.emptyText.visibility = if (!state.loading && state.voices.isEmpty()) View.VISIBLE else View.GONE
        }
        collectWhileStarted(viewModel.messages) { showMessage(it) }
    }

    override fun onResume() {
        super.onResume()
        viewModel.refresh()
    }

    override fun onDestroyView() {
        binding = null
        super.onDestroyView()
    }

    private fun open(speakerId: String?) {
        findNavController().navigate(
            R.id.importVoiceFragment,
            bundleOf(ImportVoiceViewModel.ARG_SPEAKER_ID to speakerId),
        )
    }
}
