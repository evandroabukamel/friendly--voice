import 'package:flutter_test/flutter_test.dart';
import 'package:friendly_voice/presentation/design/daylight.dart';
import 'package:material_ui/material_ui.dart';

void main() {
  test('the time of day picks dawn, day, dusk or night', () {
    Daylight at(int hour) => Daylight.at(hour);
    expect([
      for (final h in [5, 7]) at(h),
    ], everyElement(Daylight.dawn));
    expect([
      for (final h in [8, 12, 16]) at(h),
    ], everyElement(Daylight.day));
    expect([
      for (final h in [17, 19]) at(h),
    ], everyElement(Daylight.dusk));
    expect([
      for (final h in [20, 23, 0, 4]) at(h),
    ], everyElement(Daylight.night));
  });

  test('hours wrap around the clock', () {
    expect(Daylight.at(24), Daylight.night);
    expect(Daylight.at(-1), Daylight.night);
    expect(Daylight.at(29), Daylight.dawn);
  });

  test('each phase is a warm three-colour gradient with readable text', () {
    for (final d in Daylight.values) {
      expect(d.gradient, hasLength(3), reason: d.name);
      final top = d.gradient.first;
      final contrast = _contrast(d.onGradient, top);
      expect(contrast, greaterThanOrEqualTo(3), reason: '${d.name}: $contrast');
    }
  });

  test('light skies get dark text and dark skies light text', () {
    expect(Daylight.dawn.isDark, isFalse);
    expect(Daylight.day.isDark, isFalse);
    expect(Daylight.dusk.isDark, isTrue);
    expect(Daylight.night.isDark, isTrue);
  });
}

/// WCAG contrast ratio; 3:1 is the bar for large text.
double _contrast(Color a, Color b) {
  final la = a.computeLuminance();
  final lb = b.computeLuminance();
  final (hi, lo) = la > lb ? (la, lb) : (lb, la);
  return (hi + 0.05) / (lo + 0.05);
}
