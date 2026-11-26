@Tags(['native'])
library;

import 'package:flutter_test/flutter_test.dart';
import 'package:friendly_voice/infrastructure/rust_voice_core.dart';

import '../contract/voice_core_contract.dart';
import 'native_library.dart';

void main() {
  setUpAll(() async {
    if (skipNative == null) await initNative();
  });

  voiceCoreContract(
    'RustVoiceCore',
    () => RustVoiceCoreFactory(dataDir: () async => tempDataDir()).open(),
    skip: skipNative,
  );
}
