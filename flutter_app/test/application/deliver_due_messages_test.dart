import 'package:flutter_test/flutter_test.dart';
import 'package:friendly_voice/application/deliver_due_messages.dart';
import 'package:friendly_voice/domain/alarms.dart';
import 'package:friendly_voice/domain/voice_core.dart';

import '../support/fakes.dart';

const texts = NotificationTexts(channelName: 'Mensagens', title: 'Para você');

void main() {
  late FakeVoiceCore core;
  late FakeAudioPlayer player;
  late FakeAlarmScheduler alarms;
  late DeliverDueMessages deliver;

  setUp(() {
    core = FakeVoiceCore();
    player = FakeAudioPlayer();
    alarms = FakeAlarmScheduler();
    deliver = DeliverDueMessages(
      core: core,
      player: player,
      alarms: alarms,
      texts: () => texts,
    );
  });

  test('plays every due message in order and reschedules the alarms', () async {
    core
      ..due = const [
        DueMessage(
          scheduleId: 'a',
          text: 'Bom dia!',
          tone: 'warm',
          audioPath: '/1.wav',
        ),
        DueMessage(scheduleId: 'b', error: 'the voice server is unreachable'),
        DueMessage(
          scheduleId: 'c',
          text: 'Respira fundo.',
          tone: 'calm',
          audioPath: '/2.wav',
        ),
      ]
      ..alarms = const [
        Alarm(scheduleId: 'a', atEpochMs: 1),
        Alarm(scheduleId: 'c', atEpochMs: 2),
      ];

    final delivered = await deliver(listener: 'Evandro');

    expect(player.played, ['/1.wav', '/2.wav']);
    expect(alarms.replaced.single.map((a) => a.atEpochMs), [1, 2]);
    expect(alarms.texts.single, texts);
    expect(delivered.map((m) => m.scheduleId), ['a', 'b', 'c']);
  });

  test('with nothing due it only reschedules', () async {
    expect(await deliver(), isEmpty);
    expect(player.played, isEmpty);
    expect(alarms.replaced, hasLength(1));
  });

  test('never asks for more alarms than iOS can keep pending', () async {
    core.alarms = [
      for (var i = 0; i < 100; i++) Alarm(scheduleId: 's', atEpochMs: i),
    ];
    await deliver.reschedule();
    expect(alarms.replaced.single, hasLength(DeliverDueMessages.maxPending));
    expect(DeliverDueMessages.maxPending, lessThan(64));
  });
}
