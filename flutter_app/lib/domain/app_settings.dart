/// The user's preferences: their own name (to personalize messages) and the
/// optional neural voice server. Blank means "not set".
final class AppSettings {
  new({String? listener, String? engineUrl})
    : listener = _clean(listener),
      engineUrl = _clean(engineUrl);

  final String? listener;
  final String? engineUrl;

  static String? _clean(String? v) {
    final t = v?.trim();
    return (t == null || t.isEmpty) ? null : t;
  }
}

/// Where the preferences are kept.
abstract interface class SettingsStore {
  Future<AppSettings> load();
  Future<void> save(AppSettings settings);
}
