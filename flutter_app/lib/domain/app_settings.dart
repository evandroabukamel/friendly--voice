/// The user's preferences. Blank means "not set".
final class AppSettings {
  new({String? listener, String? language})
    : listener = _clean(listener),
      language = _clean(language);

  /// Their own name, used in the messages when a voice has no nickname.
  final String? listener;

  /// The app language as a BCP-47 tag ("pt-BR"); `null` follows the phone.
  final String? language;

  AppSettings copyWith({
    String? Function()? listener,
    String? Function()? language,
  }) => AppSettings(
    listener: listener == null ? this.listener : listener(),
    language: language == null ? this.language : language(),
  );
}

/// The voice server URL compiled into the app
/// (`--dart-define=FRIENDLY_VOICE_ENGINE_URL=...`); blank means none, and the
/// local sketch engine is used. Users never type it.
String? engineUrlFromBuild(String raw) => _clean(raw);

String? _clean(String? v) {
  final t = v?.trim();
  return (t == null || t.isEmpty) ? null : t;
}

/// Where the preferences are kept.
abstract interface class SettingsStore {
  Future<AppSettings> load();
  Future<void> save(AppSettings settings);
}
