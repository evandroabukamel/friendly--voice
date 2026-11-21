import 'dart:async';

import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:friendly_voice/application/deliver_due_messages.dart';
import 'package:friendly_voice/presentation/state/providers.dart';

export 'package:friendly_voice/presentation/state/providers.dart'
    show alarmSchedulerProvider, audioPlayerProvider, deliveryIntervalProvider;

/// Keeps the OS alarms in sync and delivers due messages: once the core is
/// connected (single catch-up), when an alarm is tapped and, while the app
/// is open, every [deliveryIntervalProvider].
final alarmsProvider = NotifierProvider<AlarmsNotifier, void>(
  AlarmsNotifier.new,
);

class AlarmsNotifier extends Notifier<void> {
  DeliverDueMessages? _deliver;

  @override
  void build() {
    final connection = ref.watch(connectionProvider).value;
    if (connection == null) return;
    final scheduler = ref.read(alarmSchedulerProvider);
    _deliver = DeliverDueMessages(
      core: connection.core,
      player: ref.read(audioPlayerProvider),
      alarms: scheduler,
    );
    final taps = scheduler.taps.listen((_) => deliver());
    final interval = ref.read(deliveryIntervalProvider);
    final timer = interval == null
        ? null
        : Timer.periodic(interval, (_) => deliver(quiet: true));
    ref.onDispose(() {
      unawaited(taps.cancel());
      timer?.cancel();
    });
    unawaited(_start(scheduler.requestPermissions));
  }

  Future<void> _start(Future<void> Function() requestPermissions) async {
    await requestPermissions();
    await deliver();
  }

  /// Speaks and plays what is due, then reschedules the alarms.
  Future<void> deliver({bool quiet = false}) async {
    final deliver = _deliver;
    if (deliver == null) return;
    final listener = ref.read(connectionProvider).value?.settings.listener;
    await ref
        .read(statusProvider.notifier)
        .run(() => deliver(listener: listener), quiet: quiet);
  }

  /// Called after the schedules change.
  Future<void> reschedule() async => await _deliver?.reschedule();
}
