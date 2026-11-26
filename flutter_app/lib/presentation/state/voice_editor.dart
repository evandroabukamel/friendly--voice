import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_riverpod/misc.dart' show NotifierProviderFamily;
import 'package:friendly_voice/domain/voice_core.dart';
import 'package:friendly_voice/presentation/state/people.dart';
import 'package:friendly_voice/presentation/state/providers.dart';
import 'package:friendly_voice/presentation/state/reminders.dart';

/// Least recorded material to create a voice (the core's EnrollmentPolicy).
const minVoiceSecs = 10.0;

/// What the voice screen shows.
final class VoiceEditorState {
  const new({
    this.person,
    this.name = '',
    this.consentBy = '',
    this.language,
    this.listenerName = '',
    this.samples = const [],
    this.nameError,
    this.consentError,
    this.busy = false,
  });

  /// The registered person (`null` while a new voice is being filled in).
  final SpeakerSummary? person;
  final String name;
  final String consentBy;
  final String? language;
  final String listenerName;
  final List<SampleSummary> samples;
  final String? nameError;
  final String? consentError;

  /// A recording is being added or the voice created.
  final bool busy;

  bool get registered => person != null;

  double get recordedSecs => samples.fold(0, (t, s) => t + s.durationSecs);

  VoiceEditorState copyWith({
    SpeakerSummary? person,
    String? name,
    String? consentBy,
    String? language,
    String? listenerName,
    List<SampleSummary>? samples,
    String? Function()? nameError,
    String? Function()? consentError,
    bool? busy,
  }) => VoiceEditorState(
    person: person ?? this.person,
    name: name ?? this.name,
    consentBy: consentBy ?? this.consentBy,
    language: language ?? this.language,
    listenerName: listenerName ?? this.listenerName,
    samples: samples ?? this.samples,
    nameError: nameError == null ? this.nameError : nameError(),
    consentError: consentError == null ? this.consentError : consentError(),
    busy: busy ?? this.busy,
  );
}

/// The voice screen for one person (`null` = a new voice). There is no Save
/// button: a new voice is registered — with the recorded consent — at the
/// first step that needs it (adding recordings) or when the screen is left
/// with the name and the consent filled in; the listener name is saved as
/// it is typed.
final NotifierProviderFamily<VoiceEditor, VoiceEditorState, String?>
voiceEditorProvider = NotifierProvider.autoDispose
    .family<VoiceEditor, VoiceEditorState, String?>(VoiceEditor.new);

class VoiceEditor extends Notifier<VoiceEditorState> {
  new(this.personId);

  final String? personId;

  @override
  VoiceEditorState build() {
    final person = ref
        .read(peopleProvider)
        .where((p) => p.id == personId)
        .firstOrNull;
    if (person == null) return const VoiceEditorState();
    return VoiceEditorState(
      person: person,
      name: person.name,
      consentBy: person.consentBy,
      language: person.language,
      listenerName: person.listenerName ?? '',
      samples: requireCore(ref).listSamples(person.id),
    );
  }

  void editName(String v) =>
      state = state.copyWith(name: v, nameError: () => null);

  void editConsent(String v) =>
      state = state.copyWith(consentBy: v, consentError: () => null);

  void editLanguage(String code) => state = state.copyWith(language: code);

  /// Saved right away for a registered voice.
  void editListenerName(String v) {
    state = state.copyWith(listenerName: v);
    final person = state.person;
    if (person == null) return;
    final saved = requireCore(ref).setListenerName(person.id, v);
    state = state.copyWith(person: saved);
    ref.read(peopleProvider.notifier).refresh();
  }

  /// Registers a new voice; tells what is missing instead.
  bool register() {
    if (state.registered) return true;
    final nameError = state.name.trim().isEmpty ? 'Enter the name.' : null;
    final consentError = state.consentBy.trim().isEmpty
        ? 'Enter who authorized the use of this voice.'
        : null;
    if (nameError != null || consentError != null) {
      state = state.copyWith(
        nameError: () => nameError,
        consentError: () => consentError,
      );
      return false;
    }
    try {
      final person = requireCore(ref).registerPerson(
        name: state.name.trim(),
        consentBy: state.consentBy,
        language: state.language,
        listenerName: state.listenerName,
      );
      state = state.copyWith(person: person);
      ref.read(peopleProvider.notifier).refresh();
      return true;
    } on AppFailure catch (f) {
      ref.read(messagesProvider.notifier).show(f.message, isError: true);
      return false;
    }
  }

  /// Leaving the screen keeps a new voice whose name and consent are filled.
  void leave() {
    final blank = state.name.trim().isEmpty && state.consentBy.trim().isEmpty;
    if (!state.registered && !blank) register();
  }

  /// Lets the user choose an audio file and adds it as a recording.
  Future<void> addRecording() async {
    if (!register()) return;
    final person = state.person!;
    final picked = await ref.read(audioPickerProvider).pick();
    if (picked == null || !ref.mounted) return;
    await _busy(() async {
      await requireCore(ref)
          .addSample(personId: person.id, audio: picked.bytes);
      return 'Recording added.';
    });
  }

  /// Creates (or improves) the voice from every recording.
  Future<void> createVoice() => _busy(() async {
    final person =
        state.person ?? (throw const Notice('Save the voice first.'));
    await requireCore(ref).buildVoice(person.id);
    return 'Voice created! It is ready for your reminders.';
  });

  /// Deletes the person, their recordings and voice; their reminders are
  /// switched off and kept.
  Future<bool> delete() async {
    final person = state.person;
    if (person == null) return true;
    return await ref.read(messagesProvider.notifier).run(() async {
      await requireCore(ref).deletePerson(person.id);
      ref.read(peopleProvider.notifier).refresh();
      ref.read(remindersProvider.notifier).changed();
      return 'Voice deleted.';
    });
  }

  Future<void> _busy(Future<String> Function() action) async {
    state = state.copyWith(busy: true);
    await ref.read(messagesProvider.notifier).run(action);
    if (!ref.mounted) return;
    final person = state.person!;
    ref.read(peopleProvider.notifier).refresh();
    state = state.copyWith(
      busy: false,
      person: ref
          .read(peopleProvider)
          .where((p) => p.id == person.id)
          .firstOrNull,
      samples: requireCore(ref).listSamples(person.id),
    );
  }
}
