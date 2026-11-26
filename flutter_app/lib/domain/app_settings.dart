/// The user's preferences: their own name, used in the messages when a
/// voice has no nickname for them. Blank means "not set".
final class AppSettings {
  new({String? listener}) : listener = _clean(listener);

  final String? listener;
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
