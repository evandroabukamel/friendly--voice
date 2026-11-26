import 'dart:typed_data';

/// A recording chosen by the user (format is checked by the core from the
/// bytes: WAV or FLAC).
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
