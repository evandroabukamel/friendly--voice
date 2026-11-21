import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:friendly_voice/domain/voice_core.dart';
import 'package:friendly_voice/presentation/state/providers.dart';

/// The connected core; actions before the connection fail clearly.
VoiceCore requireCore(Ref ref) =>
    ref.read(connectionProvider).value?.core ??
    (throw const AppFailure(message: 'the voice engine is not connected yet'));

/// Registered people, reloaded after each change.
final peopleProvider = NotifierProvider<PeopleNotifier, List<SpeakerSummary>>(
  PeopleNotifier.new,
);

class PeopleNotifier extends Notifier<List<SpeakerSummary>> {
  @override
  List<SpeakerSummary> build() =>
      ref.watch(connectionProvider).value?.core.listPeople() ?? const [];

  void refresh() => state = requireCore(ref).listPeople();

  /// Registers a person; returns the status message.
  String register({
    required String name,
    String? relationship,
    String? consentBy,
    String? language,
  }) {
    final person = requireCore(ref).registerPerson(
      name: name,
      relationship: relationship,
      consentBy: consentBy,
      language: language,
    );
    refresh();
    ref.read(selectedPersonIdProvider.notifier).selected = person.id;
    return 'Person registered: ${person.name}';
  }
}

/// The person chosen in the dropdown (`null` = the first one).
final selectedPersonIdProvider = NotifierProvider<SelectedPersonId, String?>(
  SelectedPersonId.new,
);

class SelectedPersonId extends Notifier<String?> {
  @override
  String? build() => null;

  String? get selected => state;
  set selected(String? id) => state = id;
}

/// The selected person, defaulting to the first one like the Kotlin spinner.
final selectedPersonProvider = Provider<SpeakerSummary?>((ref) {
  final people = ref.watch(peopleProvider);
  final id = ref.watch(selectedPersonIdProvider);
  for (final p in people) {
    if (p.id == id) return p;
  }
  return people.firstOrNull;
});

/// Dropdown label of a person, as in the Kotlin app.
String personLabel(SpeakerSummary p) {
  final voice = p.voiceReady
      ? 'voice ready · ${p.gender?.name ?? 'gender not detected'}'
      : 'voice not generated';
  return '${p.name} · ${p.language} ($voice)';
}
