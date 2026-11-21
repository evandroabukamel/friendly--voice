@Tags(['native'])
library;

import 'package:flutter_test/flutter_test.dart';
import 'package:friendly_voice/src/rust/api/catalog.dart';

import 'native_library.dart';

void main() {
  setUpAll(() async {
    if (skipNative == null) await initNative();
  });

  test('the catalog comes from the Rust domain', () {
    final c = catalog();
    expect(c.tones.map((t) => t.slug), [
      'natural',
      'warm',
      'cheerful',
      'calm',
      'firm',
      'whisper',
    ]);
    expect(
      c.themes.singleWhere((t) => t.slug == 'good-morning').defaultTone,
      'warm',
    );
    expect(c.languages.map((l) => l.code), contains('pt'));
    expect(c.maxPitchShift, 6);
  }, skip: skipNative);
}
