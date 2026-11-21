import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:friendly_voice/domain/voice_core.dart';
import 'package:friendly_voice/presentation/state/people.dart';
import 'package:friendly_voice/presentation/state/providers.dart';
import 'package:friendly_voice/presentation/state/samples.dart';

export 'package:friendly_voice/presentation/state/providers.dart'
    show audioPlayerProvider;

/// Themes, tones and limits from the core (empty until connected).
final catalogProvider = Provider<Catalog?>(
  (ref) => ref.watch(connectionProvider).value?.core.catalog(),
);

/// What the next message will be: used by "Speak now" and by new
/// schedules, as in the Kotlin app.
final class MessageDraft {
  const new({this.theme, this.text = '', this.tone, this.pitchShift = 0});

  /// `null` = the first theme of the catalog.
  final String? theme;
  final String text;

  /// `null` = the tone suggested by the theme.
  final String? tone;
  final int pitchShift;

  MessageDraft copyWith({
    String? theme,
    String? text,
    String? Function()? tone,
    int? pitchShift,
  }) => MessageDraft(
    theme: theme ?? this.theme,
    text: text ?? this.text,
    tone: tone == null ? this.tone : tone(),
    pitchShift: pitchShift ?? this.pitchShift,
  );
}

final messageDraftProvider =
    NotifierProvider<MessageDraftNotifier, MessageDraft>(
      MessageDraftNotifier.new,
    );

class MessageDraftNotifier extends Notifier<MessageDraft> {
  @override
  MessageDraft build() => const MessageDraft();

  void update(MessageDraft Function(MessageDraft) change) =>
      state = change(state);

  /// The theme actually selected (defaults to the first one).
  String? get theme =>
      state.theme ?? ref.read(catalogProvider)?.themes.firstOrNull?.slug;

  /// Speaks the draft now with the selected person's voice and plays it.
  Future<String> speakNow() async {
    final person = requirePerson(ref);
    final listener = ref.read(connectionProvider).value?.settings.listener;
    final m = await requireCore(ref).speak(
      personId: person.id,
      theme: theme,
      text: state.text,
      tone: state.tone,
      pitchShift: state.pitchShift,
      listener: listener,
    );
    await ref.read(audioPlayerProvider).play(m.audioPath);
    return '💬 ${m.text}\nTone: ${m.tone} · ${m.language}';
  }
}

/// Initial tone of [theme] in the catalog (natural when unknown).
String suggestedTone(Catalog? catalog, String? theme) {
  for (final t in catalog?.themes ?? const <ThemeOption>[]) {
    if (t.slug == theme) return t.defaultTone;
  }
  return 'natural';
}
