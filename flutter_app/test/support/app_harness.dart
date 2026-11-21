import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_riverpod/misc.dart' show Override;
import 'package:flutter_test/flutter_test.dart';
import 'package:friendly_voice/domain/app_settings.dart';
import 'package:friendly_voice/presentation/pages/home_page.dart';
import 'package:friendly_voice/presentation/state/providers.dart';
import 'package:material_ui/material_ui.dart';

import 'fakes.dart';

/// Pumps the home page over fakes and returns the fake core.
Future<FakeVoiceCore> pumpHome(
  WidgetTester tester, {
  FakeVoiceCore? core,
  AppSettings? settings,
  List<Override> overrides = const [],
}) async {
  final fake = core ?? FakeVoiceCore();
  await tester.pumpWidget(
    ProviderScope(
      overrides: [
        settingsStoreProvider.overrideWithValue(FakeSettingsStore(settings)),
        voiceCoreFactoryProvider.overrideWithValue(FakeVoiceCoreFactory(fake)),
        ...overrides,
      ],
      child: const MaterialApp(home: HomePage()),
    ),
  );
  await tester.pumpAndSettle();
  return fake;
}

/// Types into the text field that shows [hint].
Future<void> type(WidgetTester tester, String hint, String text) async {
  final field = find.widgetWithText(TextField, hint);
  await tester.ensureVisible(field);
  await tester.enterText(field, text);
}

/// Scrolls to and taps the button labelled [label].
Future<void> press(WidgetTester tester, String label) async {
  final button = find.widgetWithText(FilledButton, label);
  await tester.ensureVisible(button);
  await tester.tap(button);
  await tester.pumpAndSettle();
}
