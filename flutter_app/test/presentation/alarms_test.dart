import 'package:flutter_test/flutter_test.dart';
import 'package:friendly_voice/domain/app_settings.dart';
import 'package:friendly_voice/domain/voice_core.dart';

import '../support/app_harness.dart';
import '../support/fakes.dart';

void main() {
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
    await pumpHome(
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
  });

  testWidgets('a tapped alarm delivers; creating a schedule reschedules', (
    tester,
  ) async {
    final core = FakeCoreWithPeople.make();
    final player = FakeAudioPlayer();
    final scheduler = FakeAlarmScheduler();
    await pumpHome(tester, core: core, scheduler: scheduler, player: player);
    final before = scheduler.replaced.length;

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
    expect(player.played, ['/2.wav']);

    await type(tester, 'Time: HH:MM (or YYYY-MM-DD HH:MM for once)', '08:00');
    await press(tester, 'Schedule');
    expect(scheduler.replaced.length, greaterThan(before + 1));
  });
}
