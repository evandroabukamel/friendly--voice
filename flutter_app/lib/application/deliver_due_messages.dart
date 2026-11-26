import 'package:friendly_voice/domain/alarms.dart';
import 'package:friendly_voice/domain/audio_player.dart';
import 'package:friendly_voice/domain/voice_core.dart';

/// Speaks the schedules that are due (the core fires each one once), plays
/// them in order and keeps the OS alarms in sync with the next firings.
final class DeliverDueMessages {
  const new({
    required this.core,
    required this.player,
    required this.alarms,
    required this.texts,
  });

  /// iOS keeps at most 64 pending notifications; leave room for the OS.
  static const maxPending = 60;

  final VoiceCore core;
  final AudioPlayer player;
  final AlarmScheduler alarms;

  /// The notification words in the current app language.
  final NotificationTexts Function() texts;

  /// Returns what was due (spoken or failed), in order; the screens word it.
  Future<List<DueMessage>> call({String? listener}) async {
    final due = await core.runDueSchedules(listener: listener);
    for (final m in due) {
      if (m.audioPath case final path?) await player.play(path);
    }
    await reschedule();
    return due;
  }

  Future<void> reschedule() =>
      alarms.replaceAll(core.upcomingAlarms(maxPending), texts());
}
