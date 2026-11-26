@Tags(['native'])
library;

import 'package:flutter_test/flutter_test.dart';
import 'package:friendly_voice/src/rust/api/failure.dart';
import 'package:friendly_voice/src/rust/api/session.dart';

import 'native_library.dart';

void main() {
  setUpAll(() async {
    if (skipNative == null) await initNative();
  });

  test('registering without consent throws an actionable AppFailure', () {
    final session = Session.open(dataDir: tempDataDir().path);
    expect(
      () => session.registerSpeaker(name: 'Vó'),
      throwsA(
        isA<AppFailure>().having(
          (f) => f.message,
          'message',
          contains('consent'),
        ),
      ),
    );
  }, skip: skipNative);

  test('registered people are listed', () {
    final session = Session.open(dataDir: tempDataDir().path)
      ..registerSpeaker(name: 'Vó', consentBy: 'Vó');
    final people = session.listSpeakers();
    expect(people.single.name, 'Vó');
    expect(people.single.language, 'pt-BR');
    expect(people.single.voiceReady, isFalse);
  }, skip: skipNative);
}
