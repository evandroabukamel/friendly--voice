import 'package:flutter_test/flutter_test.dart';
import 'package:friendly_voice/domain/voice_core.dart';
import 'package:material_ui/material_ui.dart';

import '../../support/app_harness.dart';
import '../../support/fakes.dart';

Future<void> choose(WidgetTester tester, String current, String option) async {
  final dropdown = find.text(current);
  await tester.ensureVisible(dropdown);
  await tester.tap(dropdown);
  await tester.pumpAndSettle();
  await tester.tap(find.text(option).last);
  await tester.pumpAndSettle();
}

void main() {
  testWidgets('creates a daily schedule from the message above and lists it', (
    tester,
  ) async {
    final core = await pumpHome(tester, core: FakeCoreWithPeople.make());
    expect(
      find.text('4. Schedule (uses the theme, text and tone above)'),
      findsOneWidget,
    );
    expect(find.text('No schedules.'), findsOneWidget);

    await choose(tester, 'motivation', 'calm');
    await type(tester, 'Time: HH:MM (or YYYY-MM-DD HH:MM for once)', '08:00');
    await type(tester, 'Do not disturb: 22:00-07:00', '22:00-07:00');
    await press(tester, 'Schedule');

    final r = core.scheduleRequests.single;
    expect(
      (r.speakerId, r.kind, r.at, r.quiet, r.theme, r.tone, r.pitchShift),
      ('spk-1', ScheduleKind.daily, '08:00', '22:00-07:00', 'calm', null, 0.0),
    );
    expect(find.text('Scheduled! Next: 21/11 08:00'), findsOneWidget);
    expect(
      find.text('• sch-1 — tone: suggested by the theme — next: 21/11 08:00'),
      findsOneWidget,
    );
  });

  testWidgets('weekly and interval rules send their own fields', (
    tester,
  ) async {
    final core = await pumpHome(tester, core: FakeCoreWithPeople.make());
    await choose(tester, 'daily', 'weekly');
    await type(tester, 'Time: HH:MM (or YYYY-MM-DD HH:MM for once)', '07:30');
    await type(tester, 'Days (weekly): mon,wed,fri', 'mon,fri');
    await press(tester, 'Schedule');
    await choose(tester, 'weekly', 'interval');
    await type(tester, 'Every how many minutes (interval)', '120');
    await type(tester, 'Only between (interval): 09:00-18:00', '09:00-18:00');
    await press(tester, 'Schedule');

    final [weekly, interval] = core.scheduleRequests;
    expect((weekly.kind, weekly.days), (ScheduleKind.weekly, 'mon,fri'));
    expect(
      (interval.kind, interval.everyMinutes, interval.within),
      (ScheduleKind.interval, 120, '09:00-18:00'),
    );
  });

  testWidgets('removes a schedule by id and reports failures', (tester) async {
    final core = await pumpHome(tester, core: FakeCoreWithPeople.make());
    await type(tester, 'Time: HH:MM (or YYYY-MM-DD HH:MM for once)', '08:00');
    await press(tester, 'Schedule');

    await type(tester, 'Id of the schedule to remove', ' sch-1 ');
    await press(tester, 'Remove schedule');
    expect(find.text('Removed.'), findsOneWidget);
    expect(core.schedules, isEmpty);
    expect(find.text('No schedules.'), findsOneWidget);

    await press(tester, 'Remove schedule');
    expect(find.text("⚠️ schedule 'sch-1' not found"), findsOneWidget);
  });

  testWidgets('a rule without its time shows the core message', (tester) async {
    await pumpHome(tester, core: FakeCoreWithPeople.make());
    await press(tester, 'Schedule');
    expect(find.text('⚠️ provide the time (HH:MM)'), findsOneWidget);
    expect(find.byType(Slider), findsOneWidget);
  });
}
