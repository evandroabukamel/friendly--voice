@Tags(['native'])
library;

import 'package:flutter_test/flutter_test.dart';
import 'package:friendly_voice/infrastructure/rust_voice_core.dart';
import 'package:friendly_voice/src/rust/api/failure.dart';

import 'native_library.dart';

void main() {
  setUpAll(() async {
    if (skipNative == null) await initNative();
  });

  test('the factory opens the Rust core in the app folder', () async {
    final dir = tempDataDir();
    final core = await RustVoiceCoreFactory(dataDir: () async => dir).open();
    expect(core.engineName, 'local (prosody sketch)');
    expect(core.catalog().tones, hasLength(6));

    final person = core.registerPerson(name: 'Vó', consentBy: 'Vó');
    expect(core.listPeople().single.id, person.id);
    expect(() => core.registerPerson(name: 'X'), throwsA(isA<AppFailure>()));

    final again = await RustVoiceCoreFactory(dataDir: () async => dir)
        .open(engineUrl: 'http://127.0.0.1:8020');
    expect(again.engineName, 'neural server (HTTP)');
    expect(again.listPeople(), hasLength(1), reason: 'same data folder');
  }, skip: skipNative);
}
