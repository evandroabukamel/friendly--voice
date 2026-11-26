import 'package:flutter_test/flutter_test.dart';
import 'package:friendly_voice/domain/app_settings.dart';
import 'package:friendly_voice/infrastructure/preferences_settings_store.dart';
import 'package:shared_preferences_platform_interface/in_memory_shared_preferences_async.dart';
import 'package:shared_preferences_platform_interface/shared_preferences_async_platform_interface.dart';

void main() {
  setUp(() {
    SharedPreferencesAsyncPlatform.instance =
        InMemorySharedPreferencesAsync.empty();
  });

  test('starts empty', () async {
    expect((await PreferencesSettingsStore().load()).listener, isNull);
  });

  test('the saved name is loaded back, and cleared when blank', () async {
    final store = PreferencesSettingsStore();
    await store.save(AppSettings(listener: 'Evandro'));
    expect((await PreferencesSettingsStore().load()).listener, 'Evandro');

    await store.save(AppSettings(listener: ''));
    expect((await store.load()).listener, isNull);
  });
}
