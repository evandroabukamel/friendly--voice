@Tags(['native'])
library;

import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:friendly_voice/src/rust/api/session.dart';

import 'native_library.dart';

void main() {
  setUpAll(() async {
    if (skipNative == null) await initNative();
  });

  test('speaks a theme in the person voice and returns a WAV', () async {
    final session = Session.open(dataDir: tempDataDir().path);
    final person = session.registerSpeaker(name: 'Vó', consentBy: 'Vó');
    for (final f0 in [210.0, 205.0]) {
      await session.addSample(speakerId: person.id, audio: voiceLikeWav(6, f0));
    }
    await session.buildVoice(speakerId: person.id);

    final spoken = await session.speak(
      speakerId: person.id,
      theme: 'calm',
      pitchShift: 0,
      listener: 'Evandro',
    );
    expect(spoken.tone, 'calm');
    expect(File(spoken.audioPath).existsSync(), isTrue);
  }, skip: skipNative);
}
