package app.friendlyvoice.ui.alarms

import android.os.Bundle
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import androidx.core.widget.doAfterTextChanged
import androidx.fragment.app.Fragment
import androidx.navigation.fragment.findNavController
import app.friendlyvoice.R
import app.friendlyvoice.audio.MessagePlayer
import app.friendlyvoice.core.AlarmForm
import app.friendlyvoice.core.Catalog
import app.friendlyvoice.core.RepeatKind
import app.friendlyvoice.core.RepeatLabel
import app.friendlyvoice.core.Speaker
import app.friendlyvoice.core.Weekday
import app.friendlyvoice.core.humanize
import app.friendlyvoice.databinding.FragmentEditAlarmBinding
import app.friendlyvoice.ui.UiMessage
import app.friendlyvoice.ui.appViewModels
import app.friendlyvoice.ui.collectWhileStarted
import app.friendlyvoice.ui.dropdownAdapter
import app.friendlyvoice.ui.showMessage
import com.google.android.material.chip.Chip
import com.google.android.material.datepicker.MaterialDatePicker
import com.google.android.material.timepicker.MaterialTimePicker
import com.google.android.material.timepicker.TimeFormat
import java.time.Instant
import java.time.LocalDate
import java.time.ZoneOffset
import kotlin.math.roundToInt

/** Create or edit an alarm. Views only forward choices to the ViewModel and render its state. */
class EditAlarmFragment : Fragment() {

    private val viewModel by appViewModels { app, saved -> EditAlarmViewModel(app.repository, saved) }
    private var binding: FragmentEditAlarmBinding? = null
    private val player = MessagePlayer(onError = { showMessage(UiMessage.Res(R.string.error_playback)) })

    // What each dropdown currently lists (positions map to these).
    private var shownVoices: List<Speaker>? = null
    private var shownCatalog: Catalog? = null
    private var everyFilled = false

    override fun onCreateView(inflater: LayoutInflater, container: ViewGroup?, state: Bundle?): View =
        FragmentEditAlarmBinding.inflate(inflater, container, false).also { binding = it }.root

    override fun onViewCreated(view: View, savedInstanceState: Bundle?) {
        val b = binding ?: return
        shownVoices = null
        shownCatalog = null
        everyFilled = false
        viewLifecycleOwner.lifecycle.addObserver(player)

        b.header.backButton.setOnClickListener { findNavController().navigateUp() }
        b.goToVoicesButton.setOnClickListener {
            findNavController().navigate(R.id.voicesFragment)
        }
        b.voiceInput.setOnItemClickListener { _, _, position, _ ->
            shownVoices?.getOrNull(position)?.let { viewModel.selectVoice(it.id) }
        }
        b.themeInput.setOnItemClickListener { _, _, position, _ ->
            shownCatalog?.categories?.getOrNull(position)?.let { viewModel.selectCategory(it.slug) }
        }
        b.tonePitch.toneInput.setOnItemClickListener { _, _, position, _ ->
            // Position 0 is "Suggested by the theme".
            viewModel.selectTone(shownCatalog?.tones?.getOrNull(position - 1)?.slug)
        }
        b.tonePitch.pitchSlider.addOnChangeListener { _, value, fromUser ->
            if (fromUser) viewModel.setPitch(value.roundToInt())
        }

        val r = b.repeat
        val kindChips = mapOf(
            RepeatKind.DAILY to r.kindDaily,
            RepeatKind.WEEKLY to r.kindWeekly,
            RepeatKind.INTERVAL to r.kindInterval,
            RepeatKind.ONCE to r.kindOnce,
        )
        kindChips.forEach { (kind, chip) ->
            chip.setOnCheckedChangeListener { _, checked -> if (checked) viewModel.setKind(kind) }
        }
        dayChips().forEach { (day, chip) ->
            chip.setOnCheckedChangeListener { _, checked -> viewModel.setDay(day, checked) }
        }
        r.timeButton.setOnClickListener { pickTime(TimeField.TIME) }
        r.dateButton.setOnClickListener { pickDate() }
        r.window.fromButton.setOnClickListener { pickTime(TimeField.WINDOW_FROM) }
        r.window.toButton.setOnClickListener { pickTime(TimeField.WINDOW_TO) }
        r.everyInput.doAfterTextChanged { viewModel.setEveryMinutes(it?.toString()?.toIntOrNull()) }

        b.quiet.quietSwitch.setOnCheckedChangeListener { _, checked -> viewModel.setQuiet(checked) }
        b.quiet.quietRange.fromButton.setOnClickListener { pickTime(TimeField.QUIET_FROM) }
        b.quiet.quietRange.toButton.setOnClickListener { pickTime(TimeField.QUIET_TO) }

        b.playButton.setOnClickListener { viewModel.playNow() }
        b.saveButton.setOnClickListener { viewModel.save() }

        collectWhileStarted(viewModel.state) { render(it) }
        collectWhileStarted(viewModel.events) { event ->
            when (event) {
                is EditAlarmEvent.Message -> showMessage(event.message)
                is EditAlarmEvent.Play -> player.play(event.audioPath)
                EditAlarmEvent.Saved -> findNavController().navigateUp()
            }
        }
    }

    override fun onDestroyView() {
        player.release()
        binding = null
        super.onDestroyView()
    }

    private fun dayChips(): Map<Weekday, Chip> {
        val r = binding!!.repeat
        return mapOf(
            Weekday.MON to r.dayMon,
            Weekday.TUE to r.dayTue,
            Weekday.WED to r.dayWed,
            Weekday.THU to r.dayThu,
            Weekday.FRI to r.dayFri,
            Weekday.SAT to r.daySat,
            Weekday.SUN to r.daySun,
        )
    }

    private fun render(state: EditAlarmState) {
        val b = binding ?: return
        val form = state.form
        b.header.title.setText(if (state.editing) R.string.edit_alarm_title_edit else R.string.edit_alarm_title_new)

        // Dropdown contents change only when the data loads.
        if (shownVoices != state.voices) {
            shownVoices = state.voices
            b.voiceInput.setAdapter(requireContext().dropdownAdapter(state.voices.map { it.name }))
        }
        val catalog = state.catalog
        if (catalog != null && shownCatalog != catalog) {
            shownCatalog = catalog
            b.themeInput.setAdapter(requireContext().dropdownAdapter(catalog.categories.map { humanize(it.slug) }))
            val tones = listOf(getString(R.string.tone_suggested)) +
                catalog.tones.map { getString(R.string.tone_option, humanize(it.slug), it.description) }
            b.tonePitch.toneInput.setAdapter(requireContext().dropdownAdapter(tones))
        }

        val noVoices = !state.loading && state.voices.isEmpty()
        b.noVoices.visibility = if (noVoices) View.VISIBLE else View.GONE
        b.voiceLayout.visibility = if (noVoices) View.GONE else View.VISIBLE
        val voiceName = state.voices.firstOrNull { it.id == form.speakerId }?.name.orEmpty()
        b.voiceInput.setText(voiceName, false)

        b.themeInput.setText(form.category?.let(::humanize) ?: getString(R.string.alarm_own_text), false)
        val suggested = form.category?.let { slug -> catalog?.categories?.firstOrNull { it.slug == slug }?.defaultTone }
        b.tonePitch.toneInput.setText(
            form.tone?.let(::humanize)
                ?: if (suggested != null) getString(R.string.tone_suggested_with, humanize(suggested)) else getString(R.string.tone_suggested),
            false,
        )

        val pitch = form.pitch.coerceIn(MIN_PITCH, MAX_PITCH)
        if (b.tonePitch.pitchSlider.value.roundToInt() != pitch) b.tonePitch.pitchSlider.value = pitch.toFloat()
        b.tonePitch.pitchLabel.text = when {
            pitch > 0 -> resources.getQuantityString(R.plurals.pitch_higher, pitch, pitch)
            pitch < 0 -> resources.getQuantityString(R.plurals.pitch_lower, -pitch, -pitch)
            else -> getString(R.string.pitch_natural)
        }

        renderRepeat(form, loaded = !state.loading)

        b.quiet.quietSwitch.isChecked = form.quietEnabled
        b.quiet.quietRange.root.visibility = if (form.quietEnabled) View.VISIBLE else View.GONE
        b.quiet.quietRange.fromButton.text = form.quietFrom
        b.quiet.quietRange.toButton.text = form.quietTo

        val busy = state.loading || state.saving || state.playing
        b.playButton.isEnabled = !busy && form.speakerId != null && !noVoices
        b.playButton.setText(if (state.playing) R.string.alarm_playing else R.string.alarm_play_now)
        b.saveButton.isEnabled = !busy && !noVoices
        b.progress.visibility = if (busy) View.VISIBLE else View.GONE
    }

    private fun renderRepeat(form: AlarmForm, loaded: Boolean) {
        val r = binding?.repeat ?: return
        val kindChip = when (form.kind) {
            RepeatKind.DAILY -> r.kindDaily
            RepeatKind.WEEKLY -> r.kindWeekly
            RepeatKind.INTERVAL -> r.kindInterval
            RepeatKind.ONCE -> r.kindOnce
        }
        if (!kindChip.isChecked) kindChip.isChecked = true
        dayChips().forEach { (day, chip) ->
            val on = day in form.days
            if (chip.isChecked != on) chip.isChecked = on
        }
        r.daysGroup.visibility = visibleIf(form.kind == RepeatKind.WEEKLY)
        r.dateSection.visibility = visibleIf(form.kind == RepeatKind.ONCE)
        r.timeSection.visibility = visibleIf(form.kind != RepeatKind.INTERVAL)
        r.intervalSection.visibility = visibleIf(form.kind == RepeatKind.INTERVAL)
        r.timeButton.text = form.time
        r.dateButton.text = if (form.date.isBlank()) getString(R.string.repeat_pick_date) else getString(R.string.repeat_date_value, RepeatLabel.dayMonth(form.date), form.date.take(4))
        r.window.fromButton.text = form.windowFrom
        r.window.toButton.text = form.windowTo
        // Typed text is never overwritten while the user edits; fill it once.
        if (!everyFilled && loaded) {
            everyFilled = true
            r.everyInput.setText(form.everyMinutes?.toString().orEmpty())
        }
    }

    private fun visibleIf(condition: Boolean) = if (condition) View.VISIBLE else View.GONE

    private fun pickTime(field: TimeField) {
        val (hour, minute) = AlarmForm.parseHhmm(viewModel.timeOf(field))
        val picker = MaterialTimePicker.Builder()
            .setTimeFormat(TimeFormat.CLOCK_24H)
            .setHour(hour)
            .setMinute(minute)
            .build()
        picker.addOnPositiveButtonClickListener {
            viewModel.setTime(field, AlarmForm.hhmm(picker.hour, picker.minute))
        }
        picker.show(childFragmentManager, "time")
    }

    private fun pickDate() {
        val current = runCatching { LocalDate.parse(viewModel.state.value.form.date) }.getOrNull() ?: LocalDate.now()
        val picker = MaterialDatePicker.Builder.datePicker()
            .setSelection(current.atStartOfDay(ZoneOffset.UTC).toInstant().toEpochMilli())
            .build()
        picker.addOnPositiveButtonClickListener { millis ->
            viewModel.setDate(Instant.ofEpochMilli(millis).atZone(ZoneOffset.UTC).toLocalDate().toString())
        }
        picker.show(childFragmentManager, "date")
    }

    private companion object {
        const val MIN_PITCH = -6
        const val MAX_PITCH = 6
    }
}
