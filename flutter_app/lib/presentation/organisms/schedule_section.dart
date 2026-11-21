import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:friendly_voice/domain/voice_core.dart';
import 'package:friendly_voice/presentation/atoms/atoms.dart';
import 'package:friendly_voice/presentation/state/providers.dart';
import 'package:friendly_voice/presentation/state/schedules.dart';
import 'package:material_ui/material_ui.dart';

/// "4. Schedule": a rule for the message drafted above, the list of
/// schedules and removal by id.
class ScheduleSection extends ConsumerStatefulWidget {
  const new({super.key});

  @override
  ConsumerState<ScheduleSection> createState() => _ScheduleSectionState();
}

class _ScheduleSectionState extends ConsumerState<ScheduleSection> {
  ScheduleKind _kind = ScheduleKind.daily;
  final _at = TextEditingController();
  final _days = TextEditingController();
  final _every = TextEditingController();
  final _within = TextEditingController();
  final _quiet = TextEditingController();
  final _removeId = TextEditingController();

  @override
  void dispose() {
    for (final c in [_at, _days, _every, _within, _quiet, _removeId]) {
      c.dispose();
    }
    super.dispose();
  }

  Future<void> _run(String Function(SchedulesNotifier) action) => ref
      .read(statusProvider.notifier)
      .run(() async => action(ref.read(schedulesProvider.notifier)));

  ScheduleForm get _form => ScheduleForm(
    kind: _kind,
    at: _at.text,
    days: _days.text,
    everyMinutes: _every.text,
    within: _within.text,
    quiet: _quiet.text,
  );

  @override
  Widget build(BuildContext context) {
    final schedules = ref.watch(schedulesProvider);
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        const SectionTitle('4. Schedule (uses the theme, text and tone above)'),
        OptionDropdown<ScheduleKind>(
          options: [for (final k in ScheduleKind.values) (k, k.name)],
          value: _kind,
          onChanged: (k) => setState(() => _kind = k ?? _kind),
        ),
        HintTextField(
          hint: 'Time: HH:MM (or YYYY-MM-DD HH:MM for once)',
          controller: _at,
        ),
        HintTextField(hint: 'Days (weekly): mon,wed,fri', controller: _days),
        HintTextField(
          hint: 'Every how many minutes (interval)',
          controller: _every,
          keyboardType: TextInputType.number,
        ),
        HintTextField(
          hint: 'Only between (interval): 09:00-18:00',
          controller: _within,
        ),
        HintTextField(hint: 'Do not disturb: 22:00-07:00', controller: _quiet),
        ActionButton(
          label: 'Schedule',
          onPressed: () => _run((s) => s.create(_form)),
        ),
        SelectableText(
          schedules.isEmpty
              ? 'No schedules.'
              : schedules.map(scheduleLine).join('\n'),
        ),
        HintTextField(
          hint: 'Id of the schedule to remove',
          controller: _removeId,
        ),
        ActionButton(
          label: 'Remove schedule',
          onPressed: () => _run((s) => s.remove(_removeId.text)),
        ),
      ],
    );
  }
}
