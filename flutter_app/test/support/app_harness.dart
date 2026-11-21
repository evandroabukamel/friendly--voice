import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_riverpod/misc.dart' show Override;
import 'package:flutter_test/flutter_test.dart';
import 'package:friendly_voice/domain/app_settings.dart';
import 'package:friendly_voice/presentation/pages/home_page.dart';
import 'package:friendly_voice/presentation/state/providers.dart';
import 'package:material_ui/material_ui.dart';

import 'fakes.dart';

/// Pumps the home page over fakes and returns the fake core.
/// Overrides that replace every platform adapter with a fake (tests never
/// touch the real plugins).
List<Override> fakeAdapters({
  FakeVoiceCoreFactory? factory,
  AppSettings? settings,
  FakeSettingsStore? store,
  FakeAudioPicker? picker,
  FakeAudioPlayer? player,
  FakeAlarmScheduler? scheduler,
}) => [
  settingsStoreProvider.overrideWithValue(store ?? FakeSettingsStore(settings)),
  voiceCoreFactoryProvider.overrideWithValue(factory ?? FakeVoiceCoreFactory()),
  audioPickerProvider.overrideWithValue(picker ?? FakeAudioPicker()),
  audioPlayerProvider.overrideWithValue(player ?? FakeAudioPlayer()),
  alarmSchedulerProvider.overrideWithValue(scheduler ?? FakeAlarmScheduler()),
  deliveryIntervalProvider.overrideWithValue(null),
];

/// Pumps the home page over fakes and returns the fake core.
Future<FakeVoiceCore> pumpHome(
  WidgetTester tester, {
  FakeVoiceCore? core,
  AppSettings? settings,
  FakeAudioPicker? picker,
  FakeAudioPlayer? player,
  FakeAlarmScheduler? scheduler,
}) async {
  final fake = core ?? FakeVoiceCore();
  await tester.pumpWidget(
    ProviderScope(
      overrides: fakeAdapters(
        factory: FakeVoiceCoreFactory(fake),
        settings: settings,
        picker: picker,
        player: player,
        scheduler: scheduler,
      ),
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
