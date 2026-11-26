import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_riverpod/misc.dart' show NotifierProviderFamily;
import 'package:friendly_voice/domain/audio_files.dart';
import 'package:friendly_voice/domain/voice_core.dart';
import 'package:friendly_voice/l10n/app_localizations.dart';
import 'package:friendly_voice/presentation/state/people.dart';
import 'package:friendly_voice/presentation/state/providers.dart';
import 'package:friendly_voice/presentation/state/reminders.dart';

/// Least recorded material to create a voice (the core's EnrollmentPolicy).
const minVoiceSecs = 10.0;

/// What the voice screen is busy with.
enum VoiceWork { idle, adding, creating }

/// What the voice screen shows.
final class VoiceEditorState {
  const new({
    this.person,
    this.name = '',
    this.consentBy = '',
    this.language,
    this.listenerName = '',
    this.samples = const [],
    this.missingName = false,
    this.missingConsent = false,
    this.work = VoiceWork.idle,
    this.nicknameChanged = false,
  });

  /// The registered person (`null` while a new voice is being filled in).
  final SpeakerSummary? person;
  final String name;
  final String consentBy;
  final String? language;
  final String listenerName;
  final List<SampleSummary> samples;
  final bool missingName;
  final bool missingConsent;
  final VoiceWork work;

  /// The nickname of a registered voice was edited (and saved) here.
  final bool nicknameChanged;

  bool get registered => person != null;
  bool get busy => work != VoiceWork.idle;

  double get recordedSecs => samples.fold(0, (t, s) => t + s.durationSecs);

  VoiceEditorState copyWith({
    SpeakerSummary? person,
    String? name,
    String? consentBy,
    String? language,
    String? listenerName,
    List<SampleSummary>? samples,
    bool? missingName,
    bool? missingConsent,
    VoiceWork? work,
    bool? nicknameChanged,
  }) => VoiceEditorState(
    person: person ?? this.person,
    name: name ?? this.name,
    consentBy: consentBy ?? this.consentBy,
    language: language ?? this.language,
    listenerName: listenerName ?? this.listenerName,
    samples: samples ?? this.samples,
    missingName: missingName ?? this.missingName,
    missingConsent: missingConsent ?? this.missingConsent,
    work: work ?? this.work,
    nicknameChanged: nicknameChanged ?? this.nicknameChanged,
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

  MessagesNotifier get _messages => ref.read(messagesProvider.notifier);

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
      state = state.copyWith(name: v, missingName: false);

  void editConsent(String v) =>
      state = state.copyWith(consentBy: v, missingConsent: false);

  void editLanguage(String code) => state = state.copyWith(language: code);

  /// Saved right away for a registered voice.
  void editListenerName(String v) {
    state = state.copyWith(listenerName: v);
    final person = state.person;
    if (person == null) return;
    final saved = requireCore(ref).setListenerName(person.id, v);
    state = state.copyWith(person: saved, nicknameChanged: true);
    ref.read(peopleProvider.notifier).refresh();
  }

  /// Registers a new voice; marks what is missing instead.
  bool register() {
    if (state.registered) return true;
    final missingName = state.name.trim().isEmpty;
    final missingConsent = state.consentBy.trim().isEmpty;
    if (missingName || missingConsent) {
      state = state.copyWith(
        missingName: missingName,
        missingConsent: missingConsent,
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
      _messages.show((_) => f.message, isError: true);
      return false;
    }
  }

  /// Leaving the screen keeps a new voice whose name and consent are
  /// filled in, and confirms what was saved.
  void leave() {
    if (state.registered) {
      if (state.nicknameChanged) _messages.show((l) => l.voiceSaved);
      return;
    }
    final blank = state.name.trim().isEmpty && state.consentBy.trim().isEmpty;
    if (!blank && register()) _messages.show((l) => l.voiceRegistered);
  }

  /// Lets the user choose audio files and adds each as a recording.
  Future<void> addRecordings() async {
    if (!register()) return;
    final person = state.person!;
    await _work(VoiceWork.adding, () async {
      final List<PickedAudio> picked;
      try {
        picked = await ref.read(audioPickerProvider).pick();
      } on UnreadableAudio {
        throw Notice((l) => l.errorFileOpen);
      }
      if (picked.isEmpty) return null;
      final core = requireCore(ref);
      for (final file in picked) {
        await core.addSample(personId: person.id, audio: file.bytes);
      }
      String added(AppLocalizations l) => l.samplesAdded(picked.length);
      return added;
    });
  }

  /// Creates (or improves) the voice from every recording.
  Future<void> createVoice() => _work(VoiceWork.creating, () async {
    await requireCore(ref).buildVoice(state.person!.id);
    String created(AppLocalizations l) => l.voiceCreated;
    return created;
  });

  /// Deletes the person, their recordings and voice; their reminders are
  /// switched off and kept.
  Future<bool> delete() async {
    final person = state.person;
    if (person == null) return true;
    return await _messages.run(() async {
      await requireCore(ref).deletePerson(person.id);
      ref.read(peopleProvider.notifier).refresh();
      ref.read(remindersProvider.notifier).changed();
      String deleted(AppLocalizations l) => l.voiceDeleted;
      return deleted;
    });
  }

  Future<void> _work(VoiceWork work, Future<Phrase?> Function() action) async {
    state = state.copyWith(work: work);
    await _messages.run(action);
    if (!ref.mounted) return;
    final person = state.person!;
    ref.read(peopleProvider.notifier).refresh();
    state = state.copyWith(
      work: VoiceWork.idle,
      person: ref
          .read(peopleProvider)
          .where((p) => p.id == person.id)
          .firstOrNull,
      samples: requireCore(ref).listSamples(person.id),
    );
  }
}
