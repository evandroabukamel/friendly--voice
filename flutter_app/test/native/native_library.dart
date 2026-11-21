import 'dart:io';

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated_io.dart';
import 'package:friendly_voice/src/rust/frb_generated.dart';

export '../support/wav.dart';

/// Host build of the Rust bridge (`cargo build -p friendly_voice_bridge`),
/// used to test the real Rust core from Dart without a device.
final File hostLibrary = File(
  '../target/debug/libfriendly_voice_bridge'
  '${Platform.isMacOS ? '.dylib' : '.so'}',
);

/// Reason to skip native tests, or `null` when the library was built.
String? get skipNative => hostLibrary.existsSync()
    ? null
    : 'build it first: cargo build -p friendly_voice_bridge';

Future<void> initNative() async {
  if (!RustLib.instance.initialized) {
    await RustLib.init(
      externalLibrary: ExternalLibrary.open(hostLibrary.absolute.path),
    );
  }
}

/// A fresh data folder for one test.
Directory tempDataDir() => Directory.systemTemp.createTempSync('fv-native-');
