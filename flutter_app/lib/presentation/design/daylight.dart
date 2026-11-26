import 'package:friendly_voice/presentation/design/tokens.dart';
import 'package:material_ui/material_ui.dart';

/// The part of the day the screen is painted in. Every phase is warm: a
/// peach dawn, the honey-to-terracotta sunrise of the day, an ember dusk
/// and a plum night. Reminder cards use the phase of their own time, so a
/// glance tells morning from evening.
enum Daylight {
  dawn(
    gradient: [Color(0xFFFFD9B0), Color(0xFFFBB28A), Color(0xFFE88468)],
    onGradient: Palette.cocoa,
    onGradientMuted: Palette.cocoaMuted,
  ),
  day(
    gradient: [Palette.honey, Palette.apricot, Palette.terracotta],
    onGradient: Palette.cocoa,
    onGradientMuted: Palette.cocoaMuted,
  ),
  dusk(
    gradient: [Color(0xFFC9573F), Color(0xFF9C3A47), Color(0xFF5A2440)],
    onGradient: Palette.cream,
    onGradientMuted: Palette.creamMuted,
  ),
  night(
    gradient: [Color(0xFF3A2340), Color(0xFF2C1B33), Color(0xFF1E1426)],
    onGradient: Palette.cream,
    onGradientMuted: Palette.creamMuted,
  );

  new({
    required this.gradient,
    required this.onGradient,
    required this.onGradientMuted,
  });

  /// Top to bottom.
  final List<Color> gradient;

  /// Titles and text drawn straight on the gradient.
  final Color onGradient;
  final Color onGradientMuted;

  /// Dark skies get light text and light status-bar icons.
  bool get isDark => onGradient == Palette.cream;

  /// Phase of an hour of the day (any integer; wraps around the clock):
  /// dawn 5–8, day 8–17, dusk 17–20, night 20–5.
  static Daylight at(int hour) => switch (hour % 24) {
    >= 5 && < 8 => dawn,
    >= 8 && < 17 => day,
    >= 17 && < 20 => dusk,
    _ => night,
  };

  LinearGradient get linear => LinearGradient(
    begin: Alignment.topCenter,
    end: Alignment.bottomCenter,
    colors: gradient,
  );
}
