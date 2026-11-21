import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:friendly_voice/domain/app_settings.dart';
import 'package:friendly_voice/presentation/pages/home_page.dart';
import 'package:friendly_voice/presentation/state/providers.dart';
import 'package:material_ui/material_ui.dart';

import '../../support/fakes.dart';

void main() {
  testWidgets('opens the core with the saved settings and shows the engine', (
    tester,
  ) async {
    final store = FakeSettingsStore(AppSettings(listener: 'Evandro'));
    final factory = FakeVoiceCoreFactory();
    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          settingsStoreProvider.overrideWithValue(store),
          voiceCoreFactoryProvider.overrideWithValue(factory),
        ],
        child: const MaterialApp(home: HomePage()),
      ),
    );
    await tester.pumpAndSettle();

    expect(find.text('Settings'), findsOneWidget);
    expect(find.text('Voice engine: local (prosody sketch)'), findsOneWidget);
    expect(find.widgetWithText(TextField, 'Evandro'), findsOneWidget);
    expect(factory.openedWith, [null]);
  });

  testWidgets('saving the settings stores them and reconnects the engine', (
    tester,
  ) async {
    final store = FakeSettingsStore();
    final factory = FakeVoiceCoreFactory();
    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          settingsStoreProvider.overrideWithValue(store),
          voiceCoreFactoryProvider.overrideWithValue(factory),
        ],
        child: const MaterialApp(home: HomePage()),
      ),
    );
    await tester.pumpAndSettle();

    await tester.enterText(
      find.widgetWithText(TextField, 'Your name (to personalize the messages)'),
      'Evandro',
    );
    await tester.enterText(
      find.widgetWithText(TextField, 'Neural voice server URL (optional)'),
      'http://pc:8020',
    );
    await tester.tap(find.text('Save settings'));
    await tester.pumpAndSettle();

    expect(store.saved.listener, 'Evandro');
    expect(store.saved.engineUrl, 'http://pc:8020');
    expect(factory.openedWith, [null, 'http://pc:8020']);
    expect(find.text('Voice engine: neural server (HTTP)'), findsOneWidget);
  });
}
