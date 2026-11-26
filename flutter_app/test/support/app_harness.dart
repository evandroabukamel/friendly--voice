import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_riverpod/misc.dart' show Override;
import 'package:flutter_test/flutter_test.dart';
import 'package:friendly_voice/app/app.dart';
import 'package:friendly_voice/domain/app_settings.dart';
import 'package:friendly_voice/presentation/state/providers.dart';
import 'package:material_ui/material_ui.dart';

import 'fakes.dart';

/// Overrides that replace every platform adapter with a fake (tests never
/// touch the real plugins) and pin the clock.
List<Override> fakeAdapters({
  FakeVoiceCoreFactory? factory,
  AppSettings? settings,
  FakeSettingsStore? store,
  FakeAudioPicker? picker,
  FakeAudioPlayer? player,
  FakeAlarmScheduler? scheduler,
  String? engineUrl,
  DateTime? now,
}) => [
  engineUrlProvider.overrideWithValue(engineUrl),
  settingsStoreProvider.overrideWithValue(store ?? FakeSettingsStore(settings)),
  voiceCoreFactoryProvider.overrideWithValue(factory ?? FakeVoiceCoreFactory()),
  audioPickerProvider.overrideWithValue(picker ?? FakeAudioPicker()),
  audioPlayerProvider.overrideWithValue(player ?? FakeAudioPlayer()),
  alarmSchedulerProvider.overrideWithValue(scheduler ?? FakeAlarmScheduler()),
  deliveryIntervalProvider.overrideWithValue(null),
  daylightTickProvider.overrideWithValue(null),
  clockProvider.overrideWithValue(() => now ?? DateTime(2026, 11, 25, 12)),
];

/// Pumps the whole app over fakes and returns the fake core.
Future<FakeVoiceCore> pumpApp(
  WidgetTester tester, {
  FakeVoiceCore? core,
  AppSettings? settings,
  FakeSettingsStore? store,
  FakeVoiceCoreFactory? factory,
  FakeAudioPicker? picker,
  FakeAudioPlayer? player,
  FakeAlarmScheduler? scheduler,
  String? engineUrl,
  DateTime? now,
}) async {
  final fake = core ?? factory?.core ?? FakeVoiceCore();
  tester.view.physicalSize = const Size(1080, 2400);
  tester.view.devicePixelRatio = 2.5;
  addTearDown(tester.view.reset);
  await tester.pumpWidget(
    ProviderScope(
      overrides: fakeAdapters(
        factory: factory ?? FakeVoiceCoreFactory(fake),
        settings: settings,
        store: store,
        picker: picker,
        player: player,
        scheduler: scheduler,
        engineUrl: engineUrl,
        now: now,
      ),
      child: const FriendlyVoiceApp(),
    ),
  );
  await tester.pumpAndSettle();
  return fake;
}

/// Taps the tab labelled [label] in the navigation bar.
Future<void> openTab(WidgetTester tester, String label) async {
  await tester.tap(
    find.descendant(of: find.byType(NavigationBar), matching: find.text(label)),
  );
  await tester.pumpAndSettle();
}

/// Types into the text field labelled [label].
Future<void> type(WidgetTester tester, String label, String text) async {
  final field = find.widgetWithText(TextField, label);
  await reveal(tester, field);
  await tester.enterText(field, text);
  await tester.pumpAndSettle();
}

/// Scrolls the screen on top until [finder] is built and visible (the
/// screens are lazy lists).
Future<void> reveal(WidgetTester tester, Finder finder) async {
  if (finder.evaluate().isEmpty) {
    await tester.scrollUntilVisible(
      finder,
      200,
      scrollable: find.byType(Scrollable).last,
    );
  }
  await tester.ensureVisible(finder.last);
  await tester.pumpAndSettle();
}

/// Scrolls to and taps the widget showing [text].
Future<void> tapText(WidgetTester tester, String text) async {
  await reveal(tester, find.text(text));
  await tester.tap(find.text(text).last);
  await tester.pumpAndSettle();
}

/// Picks [option] in the dropdown labelled [label].
Future<void> choose(WidgetTester tester, String label, String option) async {
  final dropdown = find.ancestor(
    of: find.text(label),
    matching: find.byWidgetPredicate((w) => w is DropdownButtonFormField),
  );
  await reveal(tester, find.text(label));
  await tester.tap(dropdown.first);
  await tester.pumpAndSettle();
  await tester.tap(find.text(option).last);
  await tester.pumpAndSettle();
}
