import 'dart:typed_data';

import 'package:file_picker/file_picker.dart';
import 'package:friendly_voice/domain/audio_files.dart';

/// [AudioFilePicker] on the platform's file picker: several audio files at
/// once, like the Kotlin app's `audio/*` multiple choice.
final class FilePickerAudioPicker implements AudioFilePicker {
  @override
  Future<List<PickedAudio>> pick() async => [
    for (final file in await FilePicker.pickFiles(type: FileType.audio))
      PickedAudio(name: file.name, bytes: await _read(file)),
  ];

  static Future<Uint8List> _read(PlatformFile file) async {
    try {
      return await file.readAsBytes();
    } on Exception {
      throw UnreadableAudio(file.name);
    }
  }
}
