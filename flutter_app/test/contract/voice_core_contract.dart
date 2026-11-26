import 'package:flutter_test/flutter_test.dart';
import 'package:friendly_voice/domain/voice_core.dart';

import '../support/wav.dart';

/// Behaviour of [VoiceCore] that the UI relies on. Run against the fake
/// and against the real Rust core, so the fake cannot drift from it.
void voiceCoreContract(
  String name,
  Future<VoiceCore> Function() open, {
  String? skip,
}) {
  group('$name honours the VoiceCore contract', () {
    late VoiceCore core;
    setUp(() async {
      if (skip == null) core = await open();
    });

    test('registration needs consent; the default language is pt-BR', () {
      expect(() => core.registerPerson(name: 'Vó'), throwsA(isA<AppFailure>()));
      expect(
        () => core.registerPerson(name: 'Vó', consentBy: '  '),
        throwsA(isA<AppFailure>()),
      );
      final p = core.registerPerson(name: 'Vó', consentBy: 'Vó');
      expect((p.language, p.voiceReady), ('pt-BR', false));
      expect(core.listPeople().map((x) => x.id), [p.id]);
    }, skip: skip);

    test('the voice needs enough material, then the person is ready', () async {
      final p = core.registerPerson(name: 'Vó', consentBy: 'Vó');
      await core.addSample(personId: p.id, audio: voiceLikeWav(6, 210));
      await expectLater(core.buildVoice(p.id), throwsA(isA<AppFailure>()));
      await core.addSample(personId: p.id, audio: voiceLikeWav(6, 205));
      final v = await core.buildVoice(p.id);
      expect((v.sampleCount, v.gender), (2, Gender.female));
      expect(core.listPeople().single.voiceReady, isTrue);
    }, skip: skip);

    test('blank free text means the theme speaks', () async {
      final p = core.registerPerson(name: 'Vó', consentBy: 'Vó');
      for (final f0 in [210.0, 205.0]) {
        await core.addSample(personId: p.id, audio: voiceLikeWav(6, f0));
      }
      await core.buildVoice(p.id);
      final m = await core.speak(personId: p.id, theme: 'calm', text: ' ');
      expect(m.text.trim(), isNotEmpty);
      expect(m.audioPath, isNotEmpty);
    }, skip: skip);

    test('schedules: a blank time is refused; removal of unknown fails', () {
      final p = core.registerPerson(name: 'Vó', consentBy: 'Vó');
      ScheduleRequest daily(String at) => ScheduleRequest(
        speakerId: p.id,
        kind: ScheduleKind.daily,
        at: at,
        theme: 'calm',
        pitchShift: 0,
      );
      expect(() => core.createSchedule(daily(' ')), throwsA(isA<AppFailure>()));
      final s = core.createSchedule(daily('08:00'));
      expect(s.nextDue, endsWith('08:00'));
      expect(core.listSchedules().map((x) => x.id), [s.id]);
      core.removeSchedule(s.id);
      expect(core.listSchedules(), isEmpty);
      expect(() => core.removeSchedule(s.id), throwsA(isA<AppFailure>()));
    }, skip: skip);

    test('a person carries consent, listener name and recordings', () async {
      final p = core.registerPerson(
        name: 'Vó',
        consentBy: 'Ana',
        listenerName: ' Filhote ',
      );
      expect(
        (p.consentBy, p.listenerName, p.sampleCount),
        ('Ana', 'Filhote', 0),
      );
      await core.addSample(personId: p.id, audio: voiceLikeWav(6, 210));
      expect(core.listSamples(p.id).single.durationSecs, closeTo(6, 0.01));
      expect(core.listPeople().single.sampleCount, 1);
      expect(core.setListenerName(p.id, 'Campeão').listenerName, 'Campeão');
      expect(core.setListenerName(p.id, '  ').listenerName, isNull);
    }, skip: skip);

    test('a reminder round-trips, keeps its id when edited, and switches '
        'off', () {
      final p = core.registerPerson(name: 'Vó', consentBy: 'Vó');
      final sent = ScheduleRequest(
        speakerId: p.id,
        kind: ScheduleKind.weekly,
        at: '07:30',
        days: 'mon,fri',
        theme: 'calm',
        pitchShift: 0,
      );
      final s = core.createSchedule(sent);
      expect(
        (s.enabled, s.time, s.speakerName, s.voiceAvailable),
        (true, '07:30', 'Vó', true),
      );
      expect(s.request, sent);
      expect(core.updateSchedule(s.id, s.request), s);
      final off = core.setScheduleEnabled(s.id, false);
      expect((off.enabled, off.nextDue), (false, null));
      final edited = core.updateSchedule(
        s.id,
        ScheduleRequest(
          speakerId: p.id,
          kind: ScheduleKind.daily,
          at: '09:15',
          theme: 'calm',
          pitchShift: 0,
        ),
      );
      expect((edited.id, edited.enabled, edited.time), (s.id, false, '09:15'));
    }, skip: skip);

    test('deleting a person keeps their reminders, switched off', () async {
      final p = core.registerPerson(name: 'Vó', consentBy: 'Vó');
      core.createSchedule(
        ScheduleRequest(
          speakerId: p.id,
          kind: ScheduleKind.daily,
          at: '08:00',
          theme: 'calm',
          pitchShift: 0,
        ),
      );
      await core.deletePerson(p.id);
      expect(core.listPeople(), isEmpty);
      final kept = core.listSchedules().single;
      expect(
        (kept.enabled, kept.voiceAvailable, kept.speakerName),
        (false, false, null),
      );
      await core.deletePerson(p.id);
    }, skip: skip);
  });
}
