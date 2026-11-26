import 'package:flutter_test/flutter_test.dart';
import 'package:friendly_voice/domain/app_settings.dart';
import 'package:friendly_voice/domain/voice_core.dart';
import 'package:friendly_voice/presentation/design/daylight.dart';
import 'package:friendly_voice/presentation/templates/templates.dart';
import 'package:material_ui/material_ui.dart';

import '../support/app_harness.dart';
import '../support/fakes.dart';

void main() {
  testWidgets('three tabs: Reminders, Voices and Settings', (tester) async {
    await pumpApp(tester);

    final bar = find.byType(NavigationBar);
    for (final tab in ['Reminders', 'Voices', 'Settings']) {
      expect(
        find.descendant(of: bar, matching: find.text(tab)),
        findsOneWidget,
      );
    }
    expect(
      find.text(
        'No reminders yet.\n'
        'Tap + to hear someone you love at the right time.',
      ),
      findsOneWidget,
    );

    await openTab(tester, 'Voices');
    expect(find.textContaining('No voices yet.'), findsOneWidget);
    expect(find.byTooltip('Add voice'), findsOneWidget);

    await openTab(tester, 'Settings');
    expect(find.byType(FloatingActionButton), findsNothing);
  });

  testWidgets('the sky follows the time of day', (tester) async {
    Future<Daylight> skyAt(int hour) async {
      await pumpApp(tester, now: DateTime(2026, 11, 25, hour));
      final box = tester.widget<AnimatedContainer>(
        find
            .descendant(
              of: find.byType(DaylightBackground),
              matching: find.byType(AnimatedContainer),
            )
            .first,
      );
      final colors =
          ((box.decoration! as BoxDecoration).gradient! as LinearGradient)
              .colors;
      return Daylight.values.firstWhere(
        (d) => d.gradient.first == colors.first,
      );
    }

    expect(await skyAt(6), Daylight.dawn);
    expect(await skyAt(12), Daylight.day);
    expect(await skyAt(18), Daylight.dusk);
    expect(await skyAt(22), Daylight.night);
  });

  testWidgets('no screen has a Save button', (tester) async {
    final core = FakeCoreWithPeople.make();
    await pumpApp(tester, core: core);
    Finder save() => find.textContaining(RegExp('^Save'));
    expect(save(), findsNothing);
    await tester.tap(find.byTooltip('Add reminder'));
    await tester.pumpAndSettle();
    expect(save(), findsNothing);
    await tester.pageBack();
    await tester.pumpAndSettle();
    await openTab(tester, 'Voices');
    await tester.tap(find.byTooltip('Add voice'));
    await tester.pumpAndSettle();
    expect(save(), findsNothing);
    await tester.pageBack();
    await tester.pumpAndSettle();
    await openTab(tester, 'Settings');
    expect(save(), findsNothing);
  });

  testWidgets('on start: asks permissions, delivers due messages, schedules', (
    tester,
  ) async {
    final core = FakeCoreWithPeople.make()
      ..due = const [
        DueMessage(
          scheduleId: 'a',
          text: 'Bom dia!',
          tone: 'warm',
          audioPath: '/1.wav',
        ),
      ]
      ..alarms = const [Alarm(scheduleId: 'a', atEpochMs: 42)];
    final player = FakeAudioPlayer();
    final scheduler = FakeAlarmScheduler();
    await pumpApp(
      tester,
      core: core,
      settings: AppSettings(listener: 'Evandro'),
      scheduler: scheduler,
      player: player,
    );

    expect(scheduler.permissionsRequested, 1);
    expect(player.played, ['/1.wav']);
    expect(find.text('💬 Bom dia!\nTone: warm'), findsOneWidget);
    expect(scheduler.replaced.last.single.atEpochMs, 42);

    core.due = const [
      DueMessage(
        scheduleId: 'a',
        text: 'Oi',
        tone: 'calm',
        audioPath: '/2.wav',
      ),
    ];
    scheduler.tap();
    await tester.pumpAndSettle();
    expect(player.played, ['/1.wav', '/2.wav']);
  });
}
