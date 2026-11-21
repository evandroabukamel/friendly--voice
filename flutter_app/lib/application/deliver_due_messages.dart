import 'package:friendly_voice/domain/alarms.dart';
import 'package:friendly_voice/domain/audio_player.dart';
import 'package:friendly_voice/domain/voice_core.dart';

/// Speaks the schedules that are due (the core fires each one once), plays
/// them in order and keeps the OS alarms in sync with the next firings.
final class DeliverDueMessages {
  const new({required this.core, required this.player, required this.alarms});

  /// iOS keeps at most 64 pending notifications; leave room for the OS.
  static const maxPending = 60;

  final VoiceCore core;
  final AudioPlayer player;
  final AlarmScheduler alarms;

  /// Returns the status to show, or `null` when nothing was due.
  Future<String?> call({String? listener}) async {
    final due = await core.runDueSchedules(listener: listener);
    String? spoken;
    final errors = <String>[];
    for (final m in due) {
      if (m.audioPath case final path?) {
        await player.play(path);
        spoken = '💬 ${m.text}\nTone: ${m.tone}';
      } else {
        errors.add('⚠️ ${m.error}');
      }
    }
    await reschedule();
    final lines = [?spoken, ...errors];
    return lines.isEmpty ? null : lines.join('\n');
  }

  Future<void> reschedule() =>
      alarms.replaceAll(core.upcomingAlarms(maxPending));
}
