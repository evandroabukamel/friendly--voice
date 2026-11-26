import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:friendly_voice/domain/voice_core.dart';
import 'package:friendly_voice/presentation/state/providers.dart';

/// Themes, tones, languages and limits from the core (`null` until
/// connected).
final catalogProvider = Provider<Catalog?>(
  (ref) => ref.watch(connectionProvider).value?.core.catalog(),
);

/// The people whose voice may be used, reloaded after each change.
final peopleProvider = NotifierProvider<PeopleNotifier, List<SpeakerSummary>>(
  PeopleNotifier.new,
);

class PeopleNotifier extends Notifier<List<SpeakerSummary>> {
  @override
  List<SpeakerSummary> build() =>
      ref.watch(connectionProvider).value?.core.listPeople() ?? const [];

  void refresh() => state = requireCore(ref).listPeople();
}

/// The voices a reminder can use (generated ones).
final readyVoicesProvider = Provider<List<SpeakerSummary>>(
  (ref) => [
    for (final p in ref.watch(peopleProvider))
      if (p.voiceReady) p,
  ],
);

/// Name of a language code in the catalog ("pt-BR" → "Portuguese (Brazil)").
String languageName(Catalog? catalog, String code) {
  for (final l in catalog?.languages ?? const <LanguageOption>[]) {
    if (l.code == code) return l.name;
  }
  return code;
}

/// The catalog language closest to the phone's locale tag: the exact tag,
/// then the same language, then the first one.
String? matchLanguage(List<String> codes, String localeTag) {
  String base(String tag) => tag.split(RegExp('[-_]')).first.toLowerCase();
  final tag = localeTag.replaceAll('_', '-').toLowerCase();
  return codes.where((c) => c.toLowerCase() == tag).firstOrNull ??
      codes.where((c) => base(c) == base(tag)).firstOrNull ??
      codes.firstOrNull;
}
