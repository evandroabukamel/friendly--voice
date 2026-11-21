import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:friendly_voice/app/app.dart';
import 'package:friendly_voice/presentation/state/providers.dart';
import 'package:material_ui/material_ui.dart';

import '../support/fakes.dart';

void main() {
  testWidgets('opens the home page on a light Material 3 theme', (
    tester,
  ) async {
    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          settingsStoreProvider.overrideWithValue(FakeSettingsStore()),
          voiceCoreFactoryProvider.overrideWithValue(FakeVoiceCoreFactory()),
        ],
        child: const FriendlyVoiceApp(),
      ),
    );
    await tester.pumpAndSettle();

    expect(find.text('Settings'), findsOneWidget);
    final theme = Theme.of(tester.element(find.byType(Scaffold)));
    expect(theme.useMaterial3, isTrue);
    expect(theme.brightness, Brightness.light);
  });
}
