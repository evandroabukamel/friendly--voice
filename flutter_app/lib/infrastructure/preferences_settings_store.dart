import 'package:friendly_voice/domain/app_settings.dart';
import 'package:shared_preferences/shared_preferences.dart';

/// [SettingsStore] on the platform preferences (same key as the Kotlin app).
final class PreferencesSettingsStore implements SettingsStore {
  final SharedPreferencesAsync _prefs = SharedPreferencesAsync();

  static const _listener = 'listener';
  static const _language = 'app_language';

  @override
  Future<AppSettings> load() async => AppSettings(
    listener: await _prefs.getString(_listener),
    language: await _prefs.getString(_language),
  );

  @override
  Future<void> save(AppSettings settings) async {
    await _put(_listener, settings.listener);
    await _put(_language, settings.language);
  }

  Future<void> _put(String key, String? value) =>
      value == null ? _prefs.remove(key) : _prefs.setString(key, value);
}
