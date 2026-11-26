import 'package:material_ui/material_ui.dart';

/// Design tokens of the warm design system, ported from the Kotlin app's
/// `colors.xml` and `dimens.xml`. Widgets use these names instead of raw
/// values.
abstract final class Palette {
  // Sunrise (the day gradient).
  static const honey = Color(0xFFFFB547);
  static const apricot = Color(0xFFF38A4A);
  static const terracotta = Color(0xFFD2553F);

  // Text: cocoa on cards and light skies, cream on dark skies.
  static const cocoa = Color(0xFF3B2416);
  static const cocoaMuted = Color(0xFF6E4B36);
  static const cream = Color(0xFFFFF8EE);
  static const creamMuted = Color(0xFFFFEBD6);

  // Surfaces.
  static const card = Color(0xE6FFF4E6);
  static const cardSolid = Color(0xFFFFF4E6);
  static const navBar = Color(0xF2FFF4E6);
  static const divider = Color(0x333B2416);
  static const error = Color(0xFF8C1D18);
}

/// Spacing, shapes and sizes (dp).
abstract final class Space {
  static const xs = 4.0;
  static const s = 8.0;
  static const m = 16.0;
  static const l = 24.0;
  static const xl = 32.0;
  static const screen = 20.0;

  /// Room under a list for the floating button and the navigation bar.
  static const listBottom = 112.0;

  static const cardRadius = 28.0;
  static const fieldRadius = 20.0;
  static const buttonHeight = 64.0;
  static const touch = 48.0;
  static const chip = 48.0;
  static const icon = 28.0;
}

/// Type scale: large and light. Body text is at least 18, buttons 20.
abstract final class TypeScale {
  static const caption = 16.0;
  static const body = 18.0;
  static const button = 20.0;
  static const cardTitle = 24.0;
  static const picker = 28.0;
  static const screenTitle = 40.0;
  static const time = 64.0;
}
