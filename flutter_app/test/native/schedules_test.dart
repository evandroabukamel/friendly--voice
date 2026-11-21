@Tags(['native'])
library;

import 'package:flutter_test/flutter_test.dart';
import 'package:friendly_voice/src/rust/api/failure.dart';
import 'package:friendly_voice/src/rust/api/schedules.dart';
import 'package:friendly_voice/src/rust/api/session.dart';

import 'native_library.dart';

ScheduleRequest daily(String speakerId, String? at) => ScheduleRequest(
  speakerId: speakerId,
  kind: ScheduleKind.daily,
  at: at,
  theme: 'good-morning',
  pitchShift: 0,
);

void main() {
  setUpAll(() async {
    if (skipNative == null) await initNative();
  });

  test('schedules are created, listed, alarmed and removed', () {
    final session = Session.open(dataDir: tempDataDir().path);
    final person = session.registerSpeaker(name: 'Vó', consentBy: 'Vó');

    final created = session.createSchedule(request: daily(person.id, '08:00'));
    expect(created.nextDue, endsWith('08:00'));
    expect(session.listSchedules(), hasLength(1));

    final alarms = session.upcomingAlarms(limit: 3);
    expect(alarms, hasLength(3));
    expect(
      DateTime.fromMillisecondsSinceEpoch(alarms.first.atEpochMs)
          .isAfter(DateTime.now()),
      isTrue,
    );

    session.removeSchedule(id: created.id);
    expect(session.listSchedules(), isEmpty);
  }, skip: skipNative);

  test('a rule without its time is refused', () {
    final session = Session.open(dataDir: tempDataDir().path);
    final person = session.registerSpeaker(name: 'Vó', consentBy: 'Vó');
    expect(
      () => session.createSchedule(request: daily(person.id, null)),
      throwsA(isA<AppFailure>()),
    );
  }, skip: skipNative);
}
