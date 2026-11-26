/// Organisms of the reminder screen: what is said and by whom, the tone,
/// how it repeats, and the quiet hours. Each reports the edited form; the
/// screen saves it.
library;

import 'package:friendly_voice/domain/voice_core.dart';
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
    final own = form.text != null;
    return SoftCard(
      children: [
        if (voices.isEmpty) ...[
          const Text('No voice is ready yet. Create one in Voices first.'),
          const SizedBox(height: Space.m),
          OutlinedButton.icon(
            onPressed: onGoToVoices,
            icon: const Icon(Icons.mic_none),
            label: const Text('Go to Voices'),
          ),
          const SizedBox(height: Space.m),
        ] else
          LabeledDropdown<String>(
            label: 'Voice',
            value: form.speakerId,
            options: [for (final v in voices) (v.id, v.name)],
            onChanged: (id) => onEdit((f) => f.copyWith(speakerId: () => id)),
          ),
        LabeledDropdown<String>(
          label: 'Theme',
          value: own ? ownText : form.theme,
          options: [
            for (final t in catalog?.themes ?? const <ThemeOption>[])
              (t.slug, humanize(t.slug)),
            (ownText, 'Your own text'),
          ],
          onChanged: (slug) => onEdit(
            (f) => slug == ownText
                ? f.copyWith(text: () => f.text ?? '')
                : f.copyWith(theme: () => slug, text: () => null),
          ),
        ),
        if (own)
          LiveField(
            label: 'What should the voice say?',
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

  String _suggested() {
    for (final t in catalog?.themes ?? const <ThemeOption>[]) {
      if (t.slug == form.theme && form.text == null) {
        return 'Suggested by the theme (${humanize(t.defaultTone)})';
      }
    }
    return 'Suggested by the theme';
  }

  @override
  Widget build(BuildContext context) {
    final max = (catalog?.maxPitchShift ?? 6).round();
    final pitch = form.pitch.clamp(-max, max);
    return SoftCard(
      children: [
        LabeledDropdown<String?>(
          label: 'Tone of voice',
          value: form.tone,
          options: [
            (null, _suggested()),
            for (final t in catalog?.tones ?? const <ToneOption>[])
              (t.slug, '${humanize(t.slug)} — ${t.description}'),
          ],
          onChanged: (tone) => onEdit((f) => f.copyWith(tone: () => tone)),
        ),
        Text(switch (pitch) {
          0 => 'Pitch: natural',
          > 0 => 'Pitch: $pitch semitone${pitch == 1 ? '' : 's'} higher',
          _ => 'Pitch: ${-pitch} semitone${pitch == -1 ? '' : 's'} lower',
        }),
        Slider(
          value: pitch.toDouble(),
          min: -max.toDouble(),
          max: max.toDouble(),
          divisions: max * 2,
          label: pitch == 0 ? 'natural' : '${pitch > 0 ? '+' : ''}$pitch',
          semanticFormatterCallback: (v) =>
              'Pitch shift ${v.round()} semitones',
          onChanged: (v) => onEdit((f) => f.copyWith(pitch: v.round())),
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

  static const List<(ScheduleKind, String)> _kinds = [
    (ScheduleKind.daily, 'Daily'),
    (ScheduleKind.weekly, 'Chosen days'),
    (ScheduleKind.interval, 'Every N minutes'),
    (ScheduleKind.once, 'Once'),
  ];

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
    final kind = form.kind;
    final date = form.date;
    return SoftCard(
      children: [
        const CardTitle('Repeat'),
        const SizedBox(height: Space.m),
        Wrap(
          spacing: Space.s,
          runSpacing: Space.s,
          children: [
            for (final (k, label) in _kinds)
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
                  tooltip: weekdayNames[d],
                  label: Text(weekdayInitials[d]!),
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
          const CardLabel('On'),
          OutlinedButton(
            onPressed: () => _pickDate(context),
            child: Text(
              date == null
                  ? 'Pick a date'
                  : '${date.day.toString().padLeft(2, '0')}/'
                        '${date.month.toString().padLeft(2, '0')}/${date.year}',
              style: const TextStyle(
                fontSize: TypeScale.picker,
                fontWeight: FontWeight.w300,
              ),
            ),
          ),
        ],
        if (kind != ScheduleKind.interval) ...[
          const CardLabel('At'),
          ClockButton(
            label: 'At',
            time: form.time,
            onChanged: (t) => onEdit((f) => f.copyWith(time: t)),
          ),
        ] else ...[
          const SizedBox(height: Space.m),
          LiveField(
            label: 'Every … minutes',
            initial: '${form.everyMinutes ?? ''}',
            keyboardType: TextInputType.number,
            onChanged: (v) => onEdit(
              (f) => f.copyWith(everyMinutes: int.tryParse(v.trim()) ?? 0),
            ),
          ),
          const CardLabel('Between'),
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
  Widget build(BuildContext context) => SoftCard(
    children: [
      Row(
        children: [
          const Expanded(child: CardTitle('Quiet hours')),
          Switch(
            value: form.quietEnabled,
            onChanged: (on) => onEdit((f) => f.copyWith(quietEnabled: on)),
          ),
        ],
      ),
      const MutedText('No messages between these times.'),
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
