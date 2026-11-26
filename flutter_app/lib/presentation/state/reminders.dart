import 'dart:async';

import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:friendly_voice/domain/voice_core.dart';
import 'package:friendly_voice/presentation/state/alarms.dart';
import 'package:friendly_voice/presentation/state/providers.dart';

/// Every reminder, soonest clock time first; reloaded after each change.
final remindersProvider =
    NotifierProvider<RemindersNotifier, List<ScheduleSummary>>(
      RemindersNotifier.new,
    );

class RemindersNotifier extends Notifier<List<ScheduleSummary>> {
  @override
  List<ScheduleSummary> build() =>
      _sorted(ref.watch(connectionProvider).value?.core.listSchedules());

  static List<ScheduleSummary> _sorted(List<ScheduleSummary>? all) =>
      (all ?? const <ScheduleSummary>[]).toList()
        ..sort((a, b) => (a.time ?? '~').compareTo(b.time ?? '~'));

  /// Reloads the list and puts the OS alarms in step with it.
  void changed() {
    state = _sorted(requireCore(ref).listSchedules());
    unawaited(ref.read(alarmsProvider.notifier).reschedule());
  }

  /// The on/off switch; the core explains when it cannot switch on.
  Future<void> setEnabled(String id, {required bool enabled}) => ref
      .read(messagesProvider.notifier)
      .run(() async {
        requireCore(ref).setScheduleEnabled(id, enabled);
        return null;
      })
      .whenComplete(changed);

  Future<void> remove(String id) =>
      ref.read(messagesProvider.notifier).run(() async {
        requireCore(ref).removeSchedule(id);
        changed();
        return 'Reminder deleted.';
      });
}
