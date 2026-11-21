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
    final settings = await PreferencesSettingsStore().load();
    expect((settings.listener, settings.engineUrl), (null, null));
  });

  test('saved settings are loaded back, and cleared when blank', () async {
    final store = PreferencesSettingsStore();
    await store.save(
      AppSettings(listener: 'Evandro', engineUrl: 'http://pc:8020'),
    );
    var loaded = await PreferencesSettingsStore().load();
    expect((loaded.listener, loaded.engineUrl), ('Evandro', 'http://pc:8020'));

    await store.save(AppSettings(listener: ''));
    loaded = await store.load();
    expect((loaded.listener, loaded.engineUrl), (null, null));
  });
}
