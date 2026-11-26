import 'dart:io';
import 'dart:typed_data';

import 'package:friendly_voice/domain/audio_files.dart';
import 'package:record/record.dart';

/// [VoiceRecorder] on the `record` plugin: mono 16-bit WAV in a temporary
/// file, read back and deleted when the recording stops.
final class RecordVoiceRecorder implements VoiceRecorder {
  new({required this.tempDir});

  /// Where the recording is written while it lasts.
  final Future<Directory> Function() tempDir;

  final _recorder = AudioRecorder();
  String? _path;

  @override
  Future<bool> requestPermission() => _recorder.hasPermission();

  @override
  Future<void> start() async {
    try {
      final dir = await tempDir();
      final path =
          '${dir.path}/voice-${DateTime.now().millisecondsSinceEpoch}.wav';
      _path = path;
      await _recorder.start(
        const RecordConfig(encoder: AudioEncoder.wav, numChannels: 1),
        path: path,
      );
    } on Exception {
      throw const MicrophoneUnavailable();
    }
  }

  @override
  Future<Uint8List> stop() async {
    final path = await _recorder.stop() ?? _path;
    if (path == null) throw const RecordingFailed();
    final file = File(path);
    try {
      final bytes = await file.readAsBytes();
      if (bytes.isEmpty) throw const RecordingFailed();
      return bytes;
    } on FileSystemException {
      throw const RecordingFailed();
    } finally {
      if (file.existsSync()) file.deleteSync();
    }
  }

  @override
  Future<void> cancel() async {
    await _recorder.cancel();
    _path = null;
  }
}
