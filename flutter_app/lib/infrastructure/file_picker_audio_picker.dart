import 'package:file_picker/file_picker.dart';
import 'package:friendly_voice/domain/audio_files.dart';

/// [AudioFilePicker] on the platform's file picker (audio files, like the
/// Kotlin app's `audio/*`).
final class FilePickerAudioPicker implements AudioFilePicker {
  @override
  Future<PickedAudio?> pick() async {
    final files = await FilePicker.pickFiles(type: FileType.audio);
    if (files.isEmpty) return null;
    final file = files.first;
    return PickedAudio(name: file.name, bytes: await file.readAsBytes());
  }
}
