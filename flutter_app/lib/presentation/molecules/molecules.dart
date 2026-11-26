/// Molecules: small groups of atoms with one job each.
library;

import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:friendly_voice/domain/voice_core.dart';
import 'package:friendly_voice/l10n/app_localizations.dart';
import 'package:friendly_voice/presentation/atoms/atoms.dart';
import 'package:friendly_voice/presentation/design/daylight.dart';
import 'package:friendly_voice/presentation/design/tokens.dart';
import 'package:friendly_voice/presentation/state/labels.dart';
import 'package:friendly_voice/presentation/state/providers.dart';
import 'package:friendly_voice/presentation/state/reminder_form.dart';
import 'package:material_ui/material_ui.dart';

/// Back button and title of a pushed screen.
class ScreenHeader extends ConsumerWidget {
  const new(this.title, {super.key});

  final String title;

  @override
  Widget build(BuildContext context, WidgetRef ref) => Row(
    children: [
      IconButton(
        tooltip: AppLocalizations.of(context).actionBack,
        iconSize: Space.icon,
        color: ref.watch(daylightProvider).onGradient,
        icon: const BackButtonIcon(),
        onPressed: () => Navigator.of(context).maybePop(),
      ),
      const SizedBox(width: Space.xs),
      Expanded(child: ScreenTitle(title)),
    ],
  );
}

/// A clock time the user taps to change ("08:00", large and light).
class ClockButton extends StatelessWidget {
  const new({
    required this.time,
    required this.onChanged,
    this.label,
    super.key,
  });

  final ClockTime time;
  final ValueChanged<ClockTime> onChanged;

  /// Read by screen readers ("From", "At").
  final String? label;

  Future<void> _pick(BuildContext context) async {
    final picked = await showTimePicker(
      context: context,
      initialTime: TimeOfDay(hour: time.hour, minute: time.minute),
      helpText: label,
    );
    if (picked != null) onChanged(ClockTime(picked.hour, picked.minute));
  }

  @override
  Widget build(BuildContext context) => Semantics(
    label: label,
    child: OutlinedButton(
      onPressed: () => _pick(context),
      child: Text(
        '$time',
        style: const TextStyle(
          fontSize: TypeScale.picker,
          fontWeight: FontWeight.w300,
        ),
      ),
    ),
  );
}

/// From – to, two clock buttons.
class TimeRangePicker extends StatelessWidget {
  const new({required this.range, required this.onChanged, super.key});

  final TimeRange range;
  final ValueChanged<TimeRange> onChanged;

  @override
  Widget build(BuildContext context) => Row(
    children: [
      Expanded(
        child: ClockButton(
          time: range.from,
          onChanged: (t) => onChanged(TimeRange(t, range.to)),
        ),
      ),
      Padding(
        padding: const EdgeInsets.symmetric(horizontal: Space.s),
        child: Text(
          AppLocalizations.of(context).rangeSeparator,
          style: const TextStyle(fontSize: TypeScale.picker),
        ),
      ),
      Expanded(
        child: ClockButton(
          time: range.to,
          onChanged: (t) => onChanged(TimeRange(range.from, t)),
        ),
      ),
    ],
  );
}

/// One choice among labelled options, full width, with a floating label.
class LabeledDropdown<T> extends StatelessWidget {
  const new({
    required this.label,
    required this.options,
    required this.value,
    required this.onChanged,
    super.key,
  });

  final String label;

  /// `(value, label)` pairs, in display order.
  final List<(T, String)> options;
  final T? value;

  /// `null` shows the choice without letting it change.
  final ValueChanged<T?>? onChanged;

  @override
  Widget build(BuildContext context) => Padding(
    padding: const EdgeInsets.only(bottom: Space.m),
    child: DropdownButtonFormField<T>(
      key: ValueKey((label, value)),
      initialValue: options.any((o) => o.$1 == value) ? value : null,
      isExpanded: true,
      decoration: InputDecoration(labelText: label),
      style: const TextStyle(fontSize: TypeScale.body, color: Palette.cocoa),
      items: [
        for (final (v, text) in options)
          DropdownMenuItem<T>(
            value: v,
            child: Text(text, overflow: TextOverflow.ellipsis),
          ),
      ],
      onChanged: onChanged,
    ),
  );
}

/// A reminder in the list: its time in large thin digits, a stripe in the
/// colours of that time of day, the on/off switch, what it says, how it
/// repeats and whose voice speaks it.
class ReminderCard extends StatelessWidget {
  const new({
    required this.reminder,
    required this.catalog,
    required this.onTap,
    required this.onToggle,
    super.key,
  });

  final ScheduleSummary reminder;
  final Catalog? catalog;
  final VoidCallback onTap;
  final ValueChanged<bool> onToggle;

  @override
  Widget build(BuildContext context) {
    final l = AppLocalizations.of(context);
    final r = reminder;
    final request = r.request;
    final when =
        r.time ?? TimeRange.parse(request.within)?.from.toString() ?? '09:00';
    final big = request.kind == ScheduleKind.interval
        ? l.reminderTimeInterval(request.everyMinutes ?? 0)
        : r.time ?? '';
    // A deleted voice: the switch stays off and locked; tapping the card
    // opens the reminder to choose another voice (as in the Kotlin app).
    final on = r.enabled && r.voiceAvailable;
    return Card(
      clipBehavior: Clip.antiAlias,
      child: InkWell(
        onTap: onTap,
        child: IntrinsicHeight(
          child: Row(
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: [
              DecoratedBox(
                decoration: BoxDecoration(
                  gradient: Daylight.at(ClockTime.parse(when).hour).linear,
                ),
                child: const SizedBox(width: 10),
              ),
              Expanded(
                child: Opacity(
                  opacity: on ? 1 : 0.7,
                  child: Padding(
                    padding: const EdgeInsets.fromLTRB(
                      Space.l,
                      Space.m,
                      Space.m,
                      Space.l,
                    ),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Row(
                          children: [
                            Expanded(child: TimeText(big)),
                            Semantics(
                              label: l.reminderSwitchDescription,
                              child: Switch(
                                value: on,
                                onChanged: r.voiceAvailable ? onToggle : null,
                              ),
                            ),
                          ],
                        ),
                        Text(messageLabel(l, request, catalog)),
                        const SizedBox(height: Space.xs),
                        MutedText(repeatLabel(l, request)),
                        if (r.voiceAvailable)
                          MutedText(r.speakerName ?? '')
                        else
                          ProblemText(l.reminderVoiceUnavailable),
                      ],
                    ),
                  ),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}

/// A voice in the list: name, language, state and recordings.
class VoiceCard extends StatelessWidget {
  const new({
    required this.person,
    required this.languageName,
    required this.onTap,
    super.key,
  });

  final SpeakerSummary person;
  final String languageName;
  final VoidCallback onTap;

  @override
  Widget build(BuildContext context) => Card(
    clipBehavior: Clip.antiAlias,
    child: InkWell(
      onTap: onTap,
      child: Padding(
        padding: const EdgeInsets.all(Space.l),
        child: Row(
          children: [
            const CircleAvatar(
              radius: Space.touch / 2,
              backgroundColor: Palette.honey,
              foregroundColor: Palette.cocoa,
              child: Icon(Icons.record_voice_over_outlined, size: Space.icon),
            ),
            const SizedBox(width: Space.m),
            Expanded(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  CardTitle(person.name),
                  MutedText(languageName),
                  Text(
                    person.voiceReady
                        ? AppLocalizations.of(context).voiceStatusReady
                        : AppLocalizations.of(context).voiceStatusNotCreated,
                  ),
                  MutedText(
                    recordingsLabel(
                      AppLocalizations.of(context),
                      person.sampleCount,
                      person.totalSampleSecs,
                    ),
                  ),
                ],
              ),
            ),
          ],
        ),
      ),
    ),
  );
}

/// One recording of a voice.
class SampleRow extends StatelessWidget {
  const new({required this.number, required this.sample, super.key});

  final int number;
  final SampleSummary sample;

  @override
  Widget build(BuildContext context) => Padding(
    padding: const EdgeInsets.symmetric(vertical: Space.s),
    child: Row(
      children: [
        const Icon(Icons.graphic_eq, size: Space.icon, color: Palette.cocoa),
        const SizedBox(width: Space.m),
        Expanded(child: Text(AppLocalizations.of(context).sampleTitle(number))),
        MutedText(
          AppLocalizations.of(context)
              .sampleDuration(sample.durationSecs.toStringAsFixed(1)),
        ),
      ],
    ),
  );
}
