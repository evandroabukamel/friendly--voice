import 'package:flutter_soloud/flutter_soloud.dart';
import 'package:friendly_voice/domain/audio_player.dart';

/// [AudioPlayer] on flutter_soloud; a new message stops the previous one,
/// like the Kotlin MediaPlayer.
final class SoloudAudioPlayer implements AudioPlayer {
  final SoLoud _soloud = SoLoud.instance;
  SoundHandle? _current;

  @override
  Future<void> play(String path) async {
    if (!_soloud.isInitialized) await _soloud.init();
    if (_current case final handle?) await _soloud.stop(handle);
    final source = await _soloud.loadFile(path, autoDispose: true);
    _current = _soloud.play(source);
  }
}
