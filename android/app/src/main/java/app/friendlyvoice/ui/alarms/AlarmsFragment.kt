package app.friendlyvoice.ui.alarms

import android.os.Bundle
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import androidx.core.os.bundleOf
import androidx.fragment.app.Fragment
import androidx.navigation.fragment.findNavController
import app.friendlyvoice.R
import app.friendlyvoice.databinding.FragmentAlarmsBinding
import app.friendlyvoice.ui.appViewModels
import app.friendlyvoice.ui.collectWhileStarted
import app.friendlyvoice.ui.repeatWords
import app.friendlyvoice.ui.showMessage

/** Home: the alarm list. */
class AlarmsFragment : Fragment() {

    private val viewModel by appViewModels { app, _ -> AlarmsViewModel(app.repository) }
    private var binding: FragmentAlarmsBinding? = null

    override fun onCreateView(inflater: LayoutInflater, container: ViewGroup?, state: Bundle?): View =
        FragmentAlarmsBinding.inflate(inflater, container, false).also { binding = it }.root

    override fun onViewCreated(view: View, savedInstanceState: Bundle?) {
        val b = binding ?: return
        val adapter = AlarmAdapter(
            words = requireContext().repeatWords(),
            onOpen = { open(it.id) },
            onToggle = viewModel::onToggle,
        )
        b.alarmList.adapter = adapter
        b.addAlarm.setOnClickListener { open(null) }

        collectWhileStarted(viewModel.state) { state ->
            adapter.submitList(state.alarms)
            b.emptyText.visibility = if (!state.loading && state.alarms.isEmpty()) View.VISIBLE else View.GONE
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

    private fun open(scheduleId: String?) {
        findNavController().navigate(R.id.editAlarmFragment, bundleOf(EditAlarmViewModel.ARG_SCHEDULE_ID to scheduleId))
    }
}
