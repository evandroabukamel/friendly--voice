import 'package:friendly_voice/domain/app_settings.dart';
import 'package:shared_preferences/shared_preferences.dart';

/// [SettingsStore] on the platform preferences (same keys as the Kotlin app).
final class PreferencesSettingsStore implements SettingsStore {
  final SharedPreferencesAsync _prefs = SharedPreferencesAsync();

  static const _listener = 'listener';
  static const _engineUrl = 'engine_url';

  @override
  Future<AppSettings> load() async => AppSettings(
    listener: await _prefs.getString(_listener),
    engineUrl: await _prefs.getString(_engineUrl),
  );

  @override
  Future<void> save(AppSettings settings) async {
    await _write(_listener, settings.listener);
    await _write(_engineUrl, settings.engineUrl);
  }

  Future<void> _write(String key, String? value) =>
      value == null ? _prefs.remove(key) : _prefs.setString(key, value);
}
