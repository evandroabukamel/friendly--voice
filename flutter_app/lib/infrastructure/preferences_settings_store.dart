import 'package:friendly_voice/domain/app_settings.dart';
import 'package:shared_preferences/shared_preferences.dart';

/// [SettingsStore] on the platform preferences (same key as the Kotlin app).
final class PreferencesSettingsStore implements SettingsStore {
  final SharedPreferencesAsync _prefs = SharedPreferencesAsync();

  static const _listener = 'listener';

  @override
  Future<AppSettings> load() async =>
      AppSettings(listener: await _prefs.getString(_listener));

  @override
  Future<void> save(AppSettings settings) async {
    final name = settings.listener;
    await (name == null
        ? _prefs.remove(_listener)
        : _prefs.setString(_listener, name));
  }
}
