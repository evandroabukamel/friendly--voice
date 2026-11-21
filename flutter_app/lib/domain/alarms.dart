import 'package:friendly_voice/domain/voice_core.dart';

/// Exact OS notifications that wake the user when a schedule is due.
abstract interface class AlarmScheduler {
  /// Asks for the notification (and, on Android, exact alarm) permission.
  Future<void> requestPermissions();

  /// Replaces every pending alarm with [alarms].
  Future<void> replaceAll(List<Alarm> alarms);

  /// The user tapped an alarm notification while the app was running.
  Stream<void> get taps;
}
