/// Organisms of the reminder screen: what is said and by whom, the tone,
/// how it repeats, and the quiet hours. Each reports the edited form; the
/// screen saves it.
library;

import 'package:friendly_voice/domain/voice_core.dart';
import 'package:friendly_voice/l10n/app_localizations.dart';
import 'package:friendly_voice/presentation/atoms/atoms.dart';
import 'package:friendly_voice/presentation/design/tokens.dart';
import 'package:friendly_voice/presentation/molecules/molecules.dart';
import 'package:friendly_voice/presentation/state/labels.dart';
import 'package:friendly_voice/presentation/state/reminder_form.dart';
import 'package:material_ui/material_ui.dart';

typedef FormEdit = void Function(ReminderForm Function(ReminderForm) edit);

/// The `theme` choice meaning "my own words".
const ownText = '__own_text__';

/// Voice, theme (or own words).
class MessageCard extends StatelessWidget {
  const new({
    required this.form,
    required this.voices,
    required this.catalog,
    required this.onEdit,
    required this.onGoToVoices,
    super.key,
  });

  final ReminderForm form;
  final List<SpeakerSummary> voices;
  final Catalog? catalog;
  final FormEdit onEdit;
  final VoidCallback onGoToVoices;

  @override
  Widget build(BuildContext context) {
    final l = AppLocalizations.of(context);
    final own = form.text != null;
    return SoftCard(
      children: [
        if (voices.isEmpty) ...[
          Text(l.reminderNoVoices),
          const SizedBox(height: Space.m),
          OutlinedButton.icon(
            onPressed: onGoToVoices,
            icon: const Icon(Icons.mic_none),
            label: Text(l.reminderGoToVoices),
          ),
          const SizedBox(height: Space.m),
        ] else
          LabeledDropdown<String>(
            label: l.reminderVoiceHint,
            value: form.speakerId,
            options: [for (final v in voices) (v.id, v.name)],
            onChanged: (id) => onEdit((f) => f.copyWith(speakerId: () => id)),
          ),
        LabeledDropdown<String>(
          label: l.reminderThemeHint,
          value: own ? ownText : form.theme,
          options: [
            for (final t in catalog?.themes ?? const <ThemeOption>[])
              (t.slug, humanize(t.slug)),
            (ownText, l.reminderOwnText),
          ],
          onChanged: (slug) => onEdit(
            (f) => slug == ownText
                ? f.copyWith(text: () => f.text ?? '')
                : f.copyWith(theme: () => slug, text: () => null),
          ),
        ),
        if (own)
          LiveField(
            label: l.reminderOwnText,
            initial: form.text ?? '',
            maxLines: 3,
            capitalization: TextCapitalization.sentences,
            onChanged: (t) => onEdit((f) => f.copyWith(text: () => t)),
          ),
      ],
    );
  }
}

/// Tone of voice and pitch shift.
class TonePitchCard extends StatelessWidget {
  const new({
    required this.form,
    required this.catalog,
    required this.onEdit,
    super.key,
  });

  final ReminderForm form;
  final Catalog? catalog;
  final FormEdit onEdit;

  @override
  Widget build(BuildContext context) {
    final l = AppLocalizations.of(context);
    final max = (catalog?.maxPitchShift ?? 6).round();
    final pitch = form.pitch.clamp(-max, max);
    return SoftCard(
      children: [
        LabeledDropdown<String?>(
          label: l.reminderToneHint,
          value: form.tone,
          options: [
            (
              null,
              suggestedTone(l, form.text == null ? form.theme : null, catalog),
            ),
            for (final t in catalog?.tones ?? const <ToneOption>[])
              (t.slug, l.toneOption(humanize(t.slug), t.description)),
          ],
          onChanged: (tone) => onEdit((f) => f.copyWith(tone: () => tone)),
        ),
        Text(switch (pitch) {
          0 => l.pitchNatural,
          > 0 => l.pitchHigher(pitch),
          _ => l.pitchLower(-pitch),
        }),
        Semantics(
          label: l.pitchDescription,
          child: Slider(
            value: pitch.toDouble(),
            min: -max.toDouble(),
            max: max.toDouble(),
            divisions: max * 2,
            label: '${pitch > 0 ? '+' : ''}$pitch',
            onChanged: (v) => onEdit((f) => f.copyWith(pitch: v.round())),
          ),
        ),
      ],
    );
  }
}

/// How the reminder repeats: daily, chosen days, every N minutes, once.
class RepeatCard extends StatelessWidget {
  const new({required this.form, required this.onEdit, super.key});

  final ReminderForm form;
  final FormEdit onEdit;

  Future<void> _pickDate(BuildContext context) async {
    final today = DateUtils.dateOnly(DateTime.now());
    final picked = await showDatePicker(
      context: context,
      initialDate: form.date ?? today,
      firstDate: today,
      lastDate: today.add(const Duration(days: 365 * 2)),
    );
    if (picked != null) onEdit((f) => f.copyWith(date: picked));
  }

  @override
  Widget build(BuildContext context) {
    final l = AppLocalizations.of(context);
    final kind = form.kind;
    final date = form.date;
    final kinds = [
      (ScheduleKind.daily, l.repeatKindDaily),
      (ScheduleKind.weekly, l.repeatKindWeekly),
      (ScheduleKind.interval, l.repeatKindInterval),
      (ScheduleKind.once, l.repeatKindOnce),
    ];
    return SoftCard(
      children: [
        CardTitle(l.repeatTitle),
        const SizedBox(height: Space.m),
        Wrap(
          spacing: Space.s,
          runSpacing: Space.s,
          children: [
            for (final (k, label) in kinds)
              ChoiceChip(
                label: Text(label),
                selected: k == kind,
                onSelected: (_) => onEdit((f) => f.copyWith(kind: k)),
              ),
          ],
        ),
        if (kind == ScheduleKind.weekly) ...[
          const SizedBox(height: Space.m),
          Wrap(
            spacing: Space.xs,
            runSpacing: Space.xs,
            children: [
              for (final d in Weekday.values)
                FilterChip(
                  tooltip: weekdayName(l, d),
                  label: Text(weekdayInitial(l, d)),
                  selected: form.days.contains(d),
                  onSelected: (on) => onEdit(
                    (f) => f.copyWith(
                      days: on ? {...f.days, d} : ({...f.days}..remove(d)),
                    ),
                  ),
                ),
            ],
          ),
        ],
        if (kind == ScheduleKind.once) ...[
          CardLabel(l.repeatDateLabel),
          OutlinedButton(
            onPressed: () => _pickDate(context),
            child: Text(
              date == null
                  ? l.repeatPickDate
                  : l.repeatDateValue(
                      date.day.toString().padLeft(2, '0'),
                      date.month.toString().padLeft(2, '0'),
                    ),
              style: const TextStyle(
                fontSize: TypeScale.picker,
                fontWeight: FontWeight.w300,
              ),
            ),
          ),
        ],
        if (kind != ScheduleKind.interval) ...[
          CardLabel(l.repeatTimeLabel),
          ClockButton(
            label: l.repeatTimeLabel,
            time: form.time,
            onChanged: (t) => onEdit((f) => f.copyWith(time: t)),
          ),
        ] else ...[
          const SizedBox(height: Space.m),
          LiveField(
            label: l.repeatEveryHint,
            suffix: l.repeatEverySuffix,
            initial: '${form.everyMinutes ?? ''}',
            keyboardType: TextInputType.number,
            onChanged: (v) => onEdit(
              (f) => f.copyWith(everyMinutes: int.tryParse(v.trim()) ?? 0),
            ),
          ),
          CardLabel(l.repeatWindowLabel),
          TimeRangePicker(
            range: form.window,
            onChanged: (w) => onEdit((f) => f.copyWith(window: w)),
          ),
        ],
      ],
    );
  }
}

/// No messages between two times.
class QuietHoursCard extends StatelessWidget {
  const new({required this.form, required this.onEdit, super.key});

  final ReminderForm form;
  final FormEdit onEdit;

  @override
  Widget build(BuildContext context) {
    final l = AppLocalizations.of(context);
    return SoftCard(
      children: [
        Row(
          children: [
            Expanded(child: CardTitle(l.quietTitle)),
            Switch(
              value: form.quietEnabled,
              onChanged: (on) => onEdit((f) => f.copyWith(quietEnabled: on)),
            ),
          ],
        ),
        MutedText(l.quietHelp),
        if (form.quietEnabled) ...[
          const SizedBox(height: Space.m),
          TimeRangePicker(
            range: form.quiet,
            onChanged: (q) => onEdit((f) => f.copyWith(quiet: q)),
          ),
        ],
      ],
    );
  }
}
