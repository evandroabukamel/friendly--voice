import 'package:friendly_voice/domain/voice_core.dart';

/// The words of the alarm notifications, in the app language.
final class NotificationTexts {
  const new({required this.channelName, required this.title});

  /// Name of the notification channel in the phone's settings.
  final String channelName;
  final String title;
}

/// Exact OS notifications that wake the user when a schedule is due.
abstract interface class AlarmScheduler {
  /// Asks for the notification (and, on Android, exact alarm) permission.
  Future<void> requestPermissions();

  /// Replaces every pending alarm with [alarms].
  Future<void> replaceAll(List<Alarm> alarms, NotificationTexts texts);

  /// The user tapped an alarm notification while the app was running.
  Stream<void> get taps;
}
