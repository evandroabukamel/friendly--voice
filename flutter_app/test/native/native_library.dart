import 'dart:io';
import 'dart:math' as math;

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated_io.dart';
import 'package:friendly_voice/src/rust/frb_generated.dart';

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

/// Voiced WAV (harmonics at [f0] Hz) with syllable-like pauses, 16 kHz mono.
Uint8List voiceLikeWav(double seconds, double f0) {
  const rate = 16000;
  final n = (seconds * rate).round();
  final data = ByteData(44 + n * 2);
  void ascii(int at, String s) {
    for (var i = 0; i < s.length; i++) {
      data.setUint8(at + i, s.codeUnitAt(i));
    }
  }

  ascii(0, 'RIFF');
  data.setUint32(4, 36 + n * 2, Endian.little);
  ascii(8, 'WAVEfmt ');
  data
    ..setUint32(16, 16, Endian.little)
    ..setUint16(20, 1, Endian.little)
    ..setUint16(22, 1, Endian.little)
    ..setUint32(24, rate, Endian.little)
    ..setUint32(28, rate * 2, Endian.little)
    ..setUint16(32, 2, Endian.little)
    ..setUint16(34, 16, Endian.little);
  ascii(36, 'data');
  data.setUint32(40, n * 2, Endian.little);
  for (var i = 0; i < n; i++) {
    final t = i / rate;
    var v = 0.0;
    if (t % 0.5 < 0.35) {
      for (var k = 1; k < 6; k++) {
        v += math.sin(2 * math.pi * f0 * k * t) / k;
      }
      v *= 0.15;
    }
    data.setInt16(44 + i * 2, (v * 32767).round(), Endian.little);
  }
  return data.buffer.asUint8List();
}
