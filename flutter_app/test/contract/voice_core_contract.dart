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
  });
}
