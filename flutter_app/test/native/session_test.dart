@Tags(['native'])
library;

import 'package:flutter_test/flutter_test.dart';
import 'package:friendly_voice/src/rust/api/session.dart';

import 'native_library.dart';

void main() {
  setUpAll(() async {
    if (skipNative == null) await initNative();
  });

  test('opens a session on the local engine', () {
    final session = Session.open(dataDir: tempDataDir().path);
    expect(session.engineName, 'local (prosody sketch)');
  }, skip: skipNative);

  test('a blank engine URL means the local engine', () {
    final session = Session.open(dataDir: tempDataDir().path, engineUrl: ' ');
    expect(session.engineName, 'local (prosody sketch)');
  }, skip: skipNative);
}
