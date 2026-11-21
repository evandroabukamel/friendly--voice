import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:friendly_voice/domain/voice_core.dart';
import 'package:friendly_voice/presentation/state/message.dart';
import 'package:friendly_voice/presentation/state/people.dart';
import 'package:friendly_voice/presentation/state/providers.dart';
import 'package:friendly_voice/presentation/state/samples.dart';

/// The rule fields typed in the schedule section.
final class ScheduleForm {
  const new({
    required this.kind,
    this.at = '',
    this.days = '',
    this.everyMinutes = '',
    this.within = '',
    this.quiet = '',
  });

  final ScheduleKind kind;
  final String at;
  final String days;
  final String everyMinutes;
  final String within;
  final String quiet;
}

/// Schedules of all people, reloaded after each change.
final schedulesProvider =
    NotifierProvider<SchedulesNotifier, List<ScheduleSummary>>(
      SchedulesNotifier.new,
    );

class SchedulesNotifier extends Notifier<List<ScheduleSummary>> {
  @override
  List<ScheduleSummary> build() =>
      ref.watch(connectionProvider).value?.core.listSchedules() ?? const [];

  void refresh() => state = requireCore(ref).listSchedules();

  /// Creates a schedule speaking the message drafted above.
  String create(ScheduleForm form) {
    final person = requirePerson(ref);
    final draft = ref.read(messageDraftProvider);
    final created = requireCore(ref).createSchedule(
      ScheduleRequest(
        speakerId: person.id,
        kind: form.kind,
        at: form.at,
        days: form.days,
        everyMinutes: int.tryParse(form.everyMinutes.trim()),
        within: form.within,
        quiet: form.quiet,
        theme: ref.read(messageDraftProvider.notifier).theme,
        text: draft.text,
        tone: draft.tone,
        pitchShift: draft.pitchShift.toDouble(),
      ),
    );
    refresh();
    return 'Scheduled! Next: ${created.nextDue ?? '—'}';
  }

  String remove(String id) {
    requireCore(ref).removeSchedule(id.trim());
    refresh();
    return 'Removed.';
  }
}

/// One line of the schedule list, as in the Kotlin app.
String scheduleLine(ScheduleSummary s) =>
    '• ${s.id} — tone: ${s.toneLabel} — next: ${s.nextDue ?? '—'}';
