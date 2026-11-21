import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:friendly_voice/domain/audio_files.dart';
import 'package:friendly_voice/domain/voice_core.dart';
import 'package:friendly_voice/presentation/state/people.dart';
import 'package:friendly_voice/presentation/state/providers.dart';

export 'package:friendly_voice/presentation/state/providers.dart'
    show audioPickerProvider;

/// The person every action applies to, or a notice to register one.
SpeakerSummary requirePerson(Ref ref) =>
    ref.read(selectedPersonProvider) ??
    (throw const Notice('Register and select a person first.'));

/// The chosen recording waiting to be added.
final samplesProvider = NotifierProvider<SamplesNotifier, PickedAudio?>(
  SamplesNotifier.new,
);

class SamplesNotifier extends Notifier<PickedAudio?> {
  @override
  PickedAudio? build() => null;

  /// Lets the user choose a file; `null` (status unchanged) when cancelled.
  Future<String?> choose() async {
    final picked = await ref.read(audioPickerProvider).pick();
    if (picked == null) return null;
    state = picked;
    return 'File chosen (${picked.kilobytes} KB). Fill in the text and add it.';
  }

  Future<String> add(String transcript) async {
    final audio =
        state ?? (throw const Notice('Choose a .wav or .flac file first.'));
    final person = requirePerson(ref);
    final s = await requireCore(ref).addSample(
      personId: person.id,
      audio: audio.bytes,
      transcript: transcript,
    );
    state = null;
    return 'Sample added (${s.durationSecs.toStringAsFixed(1)}s, '
        'mean pitch ${s.meanPitchHz.toStringAsFixed(0)} Hz)';
  }

  Future<String> generateVoice() async {
    final person = requirePerson(ref);
    final v = await requireCore(ref).buildVoice(person.id);
    ref.read(peopleProvider.notifier).refresh();
    return 'Voice generated! ${v.language} · ${genderLabel(v.gender)} · '
        '${v.sampleCount} sample(s), ${v.totalAudioSecs.toStringAsFixed(0)}s';
  }
}

String genderLabel(Gender? g) => switch (g) {
  Gender.female => 'female voice',
  Gender.male => 'male voice',
  null => 'gender not detected',
};
