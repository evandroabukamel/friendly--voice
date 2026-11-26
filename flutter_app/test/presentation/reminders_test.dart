import 'package:flutter_test/flutter_test.dart';
import 'package:friendly_voice/domain/voice_core.dart';
import 'package:material_ui/material_ui.dart';

import '../support/app_harness.dart';
import '../support/fakes.dart';

ScheduleRequest daily(String at, {String speaker = 'spk-1'}) => ScheduleRequest(
  speakerId: speaker,
  kind: ScheduleKind.daily,
  at: at,
  theme: 'motivation',
  pitchShift: 0,
);

Future<void> openNewReminder(WidgetTester tester) async {
  await tester.tap(find.byTooltip('Add reminder'));
  await tester.pumpAndSettle();
}

Future<void> back(WidgetTester tester) async {
  await tester.state<NavigatorState>(find.byType(Navigator)).maybePop();
  await tester.pumpAndSettle();
}

void main() {
  testWidgets('a reminder card shows its time, message, repeat and voice', (
    tester,
  ) async {
    final core = FakeCoreWithPeople.make()..createSchedule(daily('07:30'));
    await pumpApp(tester, core: core);

    expect(find.text('07:30'), findsOneWidget);
    expect(find.textContaining('Motivation'), findsOneWidget);
    expect(find.text('Daily · Vó'), findsOneWidget);
    expect(find.textContaining('No reminders yet'), findsNothing);
  });

  testWidgets('the switch turns a reminder off and on', (tester) async {
    final core = FakeCoreWithPeople.make()..createSchedule(daily('07:30'));
    await pumpApp(tester, core: core);

    await tester.tap(find.byType(Switch));
    await tester.pumpAndSettle();
    expect(core.schedules.single.enabled, isFalse);

    await tester.tap(find.byType(Switch));
    await tester.pumpAndSettle();
    expect(core.schedules.single.enabled, isTrue);
  });

  testWidgets('a reminder whose voice was deleted asks for another voice', (
    tester,
  ) async {
    final core = FakeCoreWithPeople.make()
      ..createSchedule(daily('07:30', speaker: 'gone'));
    await pumpApp(tester, core: core);

    expect(
      find.text('Voice unavailable — tap to choose another'),
      findsOneWidget,
    );
  });

  testWidgets('swiping a reminder deletes it', (tester) async {
    final core = FakeCoreWithPeople.make()..createSchedule(daily('07:30'));
    await pumpApp(tester, core: core);

    await tester.drag(find.text('07:30'), const Offset(-600, 0));
    await tester.pumpAndSettle();
    expect(core.schedules, isEmpty);
    expect(find.text('Reminder deleted.'), findsOneWidget);
  });

  testWidgets('a new reminder saves itself on the first change and then '
      'updates in place', (tester) async {
    final core = FakeCoreWithPeople.make();
    await pumpApp(tester, core: core);

    await openNewReminder(tester);
    expect(find.text('New reminder'), findsOneWidget);
    expect(core.schedules, isEmpty, reason: 'opening alone saves nothing');

    await tapText(tester, 'Chosen days');
    expect(core.schedules, hasLength(1));
    final id = core.schedules.single.id;
    expect(core.schedules.single.request.kind, ScheduleKind.weekly);
    expect(core.schedules.single.request.speakerId, 'spk-1');
    await tester.scrollUntilVisible(find.text('Edit reminder'), -300);

    await tapText(tester, 'Daily');
    expect(core.schedules.single.id, id);
    expect(core.schedules.single.request.kind, ScheduleKind.daily);

    await back(tester);
    expect(find.text('08:00'), findsOneWidget);
    expect(find.text('Daily · Vó'), findsOneWidget);
  });

  testWidgets('leaving a new reminder untouched creates nothing', (
    tester,
  ) async {
    final core = FakeCoreWithPeople.make();
    await pumpApp(tester, core: core);
    await openNewReminder(tester);
    await back(tester);
    expect(core.schedules, isEmpty);
    expect(find.textContaining('No reminders yet'), findsOneWidget);
  });

  testWidgets('incomplete choices are explained instead of saved', (
    tester,
  ) async {
    final core = FakeCoreWithPeople.make()
      ..createSchedule(
        const ScheduleRequest(
          speakerId: 'spk-1',
          kind: ScheduleKind.weekly,
          at: '08:00',
          days: 'mon',
          theme: 'motivation',
          pitchShift: 0,
        ),
      );
    await pumpApp(tester, core: core);
    await tester.tap(find.text('08:00'));
    await tester.pumpAndSettle();

    await reveal(tester, find.byTooltip('Monday'));
    await tester.tap(find.byTooltip('Monday'));
    await tester.pumpAndSettle();
    expect(find.text('Choose at least one day.'), findsOneWidget);
    expect(core.schedules.single.request.days, 'mon');

    await reveal(tester, find.byTooltip('Friday'));
    await tester.tap(find.byTooltip('Friday'));
    await tester.pumpAndSettle();
    expect(find.text('Choose at least one day.'), findsNothing);
    expect(core.schedules.single.request.days, 'fri');
  });

  testWidgets('own words replace the theme', (tester) async {
    final core = FakeCoreWithPeople.make();
    await pumpApp(tester, core: core);
    await openNewReminder(tester);

    await choose(tester, 'Theme', 'Your own text');
    await type(tester, 'What should the voice say?', 'Drink water!');
    expect(core.schedules.single.request.text, 'Drink water!');
    expect(core.schedules.single.request.theme, isNull);
  });

  testWidgets('Play now speaks the message in the chosen voice', (
    tester,
  ) async {
    final player = FakeAudioPlayer();
    final core = FakeCoreWithPeople.make();
    await pumpApp(tester, core: core, player: player);
    await openNewReminder(tester);

    await tapText(tester, 'Play now');
    expect(core.spoken.single.personId, 'spk-1');
    expect(player.played, hasLength(1));
  });

  testWidgets('without a ready voice, the editor leads to Voices', (
    tester,
  ) async {
    await pumpApp(tester);
    await openNewReminder(tester);

    expect(
      find.text('No voice is ready yet. Create one in Voices first.'),
      findsOneWidget,
    );
    await tapText(tester, 'Go to Voices');
    expect(find.textContaining('No voices yet.'), findsOneWidget);
  });

  testWidgets('deleting from the editor removes the reminder', (tester) async {
    final core = FakeCoreWithPeople.make()..createSchedule(daily('07:30'));
    await pumpApp(tester, core: core);
    await tester.tap(find.text('07:30'));
    await tester.pumpAndSettle();

    await tapText(tester, 'Delete reminder');
    expect(core.schedules, isEmpty);
    expect(find.text('Reminders'), findsWidgets);
  });
}
