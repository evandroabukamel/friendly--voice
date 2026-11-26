import 'dart:typed_data';

/// A recording chosen by the user (format is checked by the core from the
/// bytes: WAV, FLAC, MP3, M4A or OGG).
final class PickedAudio {
  const new({required this.name, required this.bytes});

  final String name;
  final Uint8List bytes;

  int get kilobytes => bytes.length ~/ 1024;
}

/// Lets the user choose one or more audio files; empty when cancelled.
/// Throws [UnreadableAudio] when a chosen file cannot be read.
abstract interface class AudioFilePicker {
  Future<List<PickedAudio>> pick();
}

/// A chosen file could not be opened.
final class UnreadableAudio implements Exception {
  const new(this.name);

  final String name;
}

/// Records the user's voice from the microphone, as WAV.
abstract interface class VoiceRecorder {
  /// Asks for the microphone; `false` when the user refuses.
  Future<bool> requestPermission();

  /// Starts recording. Throws [MicrophoneUnavailable].
  Future<void> start();

  /// Stops and returns the recording as WAV. Throws [RecordingFailed].
  Future<Uint8List> stop();

  /// Stops and throws the recording away.
  Future<void> cancel();
}

/// There is no microphone, or another app holds it.
final class MicrophoneUnavailable implements Exception {
  const new();
}

/// The recording could not be finished or read back.
final class RecordingFailed implements Exception {
  const new();
}
