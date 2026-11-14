package app.friendlyvoice.ui.alarms

import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import androidx.recyclerview.widget.DiffUtil
import androidx.recyclerview.widget.ListAdapter
import androidx.recyclerview.widget.RecyclerView
import app.friendlyvoice.R
import app.friendlyvoice.core.Repeat
import app.friendlyvoice.core.RepeatLabel
import app.friendlyvoice.core.RepeatWords
import app.friendlyvoice.core.Schedule
import app.friendlyvoice.core.humanize
import app.friendlyvoice.databinding.MoleculeAlarmCardBinding

/** Alarm cards (layout/molecule_alarm_card.xml). Tap opens the editor; the switch toggles. */
class AlarmAdapter(
    private val words: RepeatWords,
    private val onOpen: (Schedule) -> Unit,
    private val onToggle: (id: String, enabled: Boolean) -> Unit,
) : ListAdapter<Schedule, AlarmAdapter.Holder>(Diff) {

    class Holder(val binding: MoleculeAlarmCardBinding) : RecyclerView.ViewHolder(binding.root)

    override fun onCreateViewHolder(parent: ViewGroup, viewType: Int) =
        Holder(MoleculeAlarmCardBinding.inflate(LayoutInflater.from(parent.context), parent, false))

    override fun onBindViewHolder(holder: Holder, position: Int) {
        val item = getItem(position)
        val b = holder.binding
        val res = b.root.resources
        val repeat = item.request.repeat
        b.alarmTime.text = when {
            repeat is Repeat.Interval -> res.getString(R.string.alarm_time_interval, repeat.everyMinutes)
            !item.time.isNullOrBlank() -> item.time
            repeat is Repeat.Once -> repeat.time
            repeat is Repeat.Daily -> repeat.at
            repeat is Repeat.Weekly -> repeat.at
            else -> ""
        }
        val theme = item.request.category?.let(::humanize) ?: res.getString(R.string.alarm_own_text)
        b.alarmTheme.text = res.getString(R.string.alarm_theme_line, theme, item.toneLabel)
        b.alarmRepeat.text = RepeatLabel.format(repeat, words)
        // A deleted voice: the switch stays off and locked; tapping the card opens edit.
        b.alarmVoice.visibility = if (item.voiceAvailable) View.VISIBLE else View.GONE
        b.alarmVoiceUnavailable.visibility = if (item.voiceAvailable) View.GONE else View.VISIBLE
        b.alarmVoice.text = item.speakerName.orEmpty()
        b.root.alpha = if (item.enabled && item.voiceAvailable) 1f else DISABLED_ALPHA

        b.alarmSwitch.setOnCheckedChangeListener(null)
        b.alarmSwitch.isChecked = item.enabled && item.voiceAvailable
        b.alarmSwitch.isEnabled = item.voiceAvailable
        b.alarmSwitch.setOnCheckedChangeListener { _, checked -> onToggle(item.id, checked) }
        b.root.setOnClickListener { onOpen(item) }
    }

    private object Diff : DiffUtil.ItemCallback<Schedule>() {
        override fun areItemsTheSame(a: Schedule, b: Schedule) = a.id == b.id
        override fun areContentsTheSame(a: Schedule, b: Schedule) = a == b
    }

    private companion object {
        const val DISABLED_ALPHA = 0.7f
    }
}
