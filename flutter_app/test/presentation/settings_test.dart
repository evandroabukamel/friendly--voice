import 'package:flutter_test/flutter_test.dart';
import 'package:friendly_voice/domain/app_settings.dart';
import 'package:friendly_voice/l10n/app_localizations_de.dart';
import 'package:friendly_voice/l10n/app_localizations_ja.dart';
import 'package:friendly_voice/l10n/app_localizations_pt.dart';
import 'package:material_ui/material_ui.dart';

import '../support/app_harness.dart';
import '../support/fakes.dart';

Finder tab(String label) =>
    find.descendant(of: find.byType(NavigationBar), matching: find.text(label));

void main() {
  testWidgets('your name saves itself; the server is shown, not typed', (
    tester,
  ) async {
    final store = FakeSettingsStore(AppSettings(listener: 'Evan'));
    await pumpApp(tester, store: store, engineUrl: 'http://10.0.0.2:8000');
    await openTab(tester, 'Settings');

    await type(tester, 'Your name', 'Evandro');
    expect(store.saved.listener, 'Evandro');

    final server = tester.widget<TextField>(
      find.widgetWithText(TextField, 'Voice server URL'),
    );
    expect(server.enabled, isFalse);
    expect(server.controller!.text, 'http://10.0.0.2:8000');
  });

  testWidgets('choosing the app language translates the app and is kept', (
    tester,
  ) async {
    final store = FakeSettingsStore(AppSettings(listener: 'Evan'));
    final scheduler = FakeAlarmScheduler();
    await pumpApp(tester, store: store, scheduler: scheduler);
    await openTab(tester, 'Settings');

    await choose(tester, 'App language', 'Deutsch');
    final de = AppLocalizationsDe();
    expect(store.saved.language, 'de');
    expect(store.saved.listener, 'Evan');
    expect(tab(de.tabReminders), findsOneWidget);
    expect(find.text(de.settingsSaved), findsOneWidget);

    await choose(tester, de.settingsLanguageHint, de.languageSystemDefault);
    expect(store.saved.language, isNull);
    expect(tab('Reminders'), findsOneWidget);
  });

  testWidgets('without a choice the app follows the phone language', (
    tester,
  ) async {
    tester.platformDispatcher.localesTestValue = const [Locale('ja', 'JP')];
    addTearDown(tester.platformDispatcher.clearLocalesTestValue);
    await pumpApp(tester);
    expect(tab(AppLocalizationsJa().tabReminders), findsOneWidget);
  });

  testWidgets('a kept choice wins over the phone, and notifications follow', (
    tester,
  ) async {
    tester.platformDispatcher.localesTestValue = const [Locale('ja', 'JP')];
    addTearDown(tester.platformDispatcher.clearLocalesTestValue);
    final scheduler = FakeAlarmScheduler();
    await pumpApp(
      tester,
      core: FakeCoreWithPeople.make(),
      settings: AppSettings(language: 'pt-BR'),
      scheduler: scheduler,
    );
    final pt = AppLocalizationsPt();
    expect(tab(pt.tabReminders), findsOneWidget);
    expect(scheduler.texts.last.title, pt.notificationTitle);
    expect(scheduler.texts.last.channelName, pt.channelName);
  });

  testWidgets('Arabic lays the app out right to left', (tester) async {
    await pumpApp(tester, settings: AppSettings(language: 'ar'));
    expect(
      Directionality.of(tester.element(find.byType(NavigationBar))),
      TextDirection.rtl,
    );
  });
}
