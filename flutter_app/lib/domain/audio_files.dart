import 'dart:typed_data';

/// A recording chosen by the user (format is checked by the core from the
/// bytes: WAV or FLAC).
final class PickedAudio {
  const new({required this.name, required this.bytes});

  final String name;
  final Uint8List bytes;

  int get kilobytes => bytes.length ~/ 1024;
}

/// Lets the user choose an audio file; `null` when cancelled.
abstract interface class AudioFilePicker {
  Future<PickedAudio?> pick();
}
