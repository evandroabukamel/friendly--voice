@Tags(['native'])
library;

import 'package:flutter_test/flutter_test.dart';
import 'package:friendly_voice/src/rust/api/failure.dart';
import 'package:friendly_voice/src/rust/api/session.dart';
import 'package:friendly_voice/src/rust/api/speakers.dart';

import 'native_library.dart';

void main() {
  setUpAll(() async {
    if (skipNative == null) await initNative();
  });

  test('samples are analyzed and the voice is generated', () async {
    final session = Session.open(dataDir: tempDataDir().path);
    final person = session.registerSpeaker(name: 'Vó', consentBy: 'Vó');

    final sample = await session.addSample(
      speakerId: person.id,
      audio: voiceLikeWav(6, 210),
    );
    expect(sample.durationSecs, closeTo(6, 0.01));
    await expectLater(
      session.buildVoice(speakerId: person.id),
      throwsA(isA<AppFailure>()),
    );

    await session.addSample(speakerId: person.id, audio: voiceLikeWav(6, 205));
    final voice = await session.buildVoice(speakerId: person.id);
    expect(voice.sampleCount, 2);
    expect(voice.gender, Gender.female);
  }, skip: skipNative);
}
