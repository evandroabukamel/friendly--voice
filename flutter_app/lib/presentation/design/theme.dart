import 'package:friendly_voice/presentation/design/tokens.dart';
import 'package:material_ui/material_ui.dart';

/// The app's Material 3 theme: warm, light surfaces; large, light type;
/// cocoa pill buttons; cream cards. The window behind every screen is the
/// daylight gradient (see `GradientPage`), so the scaffold is transparent.
ThemeData warmTheme() {
  const scheme = ColorScheme(
    brightness: Brightness.light,
    primary: Palette.cocoa,
    onPrimary: Palette.cream,
    primaryContainer: Palette.honey,
    onPrimaryContainer: Palette.cocoa,
    secondary: Palette.terracotta,
    onSecondary: Palette.cream,
    secondaryContainer: Palette.honey,
    onSecondaryContainer: Palette.cocoa,
    tertiary: Palette.apricot,
    onTertiary: Palette.cocoa,
    error: Palette.error,
    onError: Palette.cream,
    surface: Palette.cardSolid,
    onSurface: Palette.cocoa,
    onSurfaceVariant: Palette.cocoaMuted,
    outline: Palette.cocoaMuted,
    outlineVariant: Palette.divider,
  );
  const light = FontWeight.w300;
  const thin = FontWeight.w200;
  final text = Typography.material2021().black
      .apply(bodyColor: Palette.cocoa, displayColor: Palette.cocoa)
      .copyWith(
        displayLarge: const TextStyle(
          fontSize: TypeScale.time,
          fontWeight: thin,
        ),
        headlineLarge: const TextStyle(
          fontSize: TypeScale.screenTitle,
          fontWeight: thin,
        ),
        titleLarge: const TextStyle(
          fontSize: TypeScale.cardTitle,
          fontWeight: light,
        ),
        bodyLarge: const TextStyle(fontSize: TypeScale.body, height: 1.3),
        bodyMedium: const TextStyle(fontSize: TypeScale.body, height: 1.3),
        labelLarge: const TextStyle(
          fontSize: TypeScale.button,
          fontWeight: FontWeight.w400,
        ),
        labelMedium: const TextStyle(
          fontSize: TypeScale.caption,
          fontWeight: FontWeight.w500,
          color: Palette.cocoaMuted,
        ),
      );
  const pill = StadiumBorder();
  const buttonSize = Size(Space.touch, Space.buttonHeight);
  final buttonText = text.labelLarge;
  final fieldBorder = OutlineInputBorder(
    borderRadius: BorderRadius.circular(Space.fieldRadius),
    borderSide: const BorderSide(color: Palette.cocoaMuted),
  );
  return ThemeData(
    useMaterial3: true,
    colorScheme: scheme,
    textTheme: text,
    scaffoldBackgroundColor: Colors.transparent,
    canvasColor: Palette.cardSolid,
    cardTheme: const CardThemeData(
      color: Palette.card,
      elevation: 0,
      margin: EdgeInsets.only(bottom: Space.m),
      shape: RoundedRectangleBorder(
        borderRadius: BorderRadius.all(Radius.circular(Space.cardRadius)),
      ),
    ),
    filledButtonTheme: FilledButtonThemeData(
      style: FilledButton.styleFrom(
        minimumSize: buttonSize,
        shape: pill,
        textStyle: buttonText,
        iconSize: Space.icon,
      ),
    ),
    outlinedButtonTheme: OutlinedButtonThemeData(
      style: OutlinedButton.styleFrom(
        minimumSize: buttonSize,
        shape: pill,
        textStyle: buttonText,
        foregroundColor: Palette.cocoa,
        side: const BorderSide(color: Palette.cocoa, width: 2),
        iconSize: Space.icon,
      ),
    ),
    textButtonTheme: TextButtonThemeData(
      style: TextButton.styleFrom(
        minimumSize: const Size(Space.touch, Space.touch),
        textStyle: buttonText,
        foregroundColor: Palette.cocoa,
      ),
    ),
    floatingActionButtonTheme: const FloatingActionButtonThemeData(
      backgroundColor: Palette.cocoa,
      foregroundColor: Palette.cream,
      shape: CircleBorder(),
      iconSize: Space.icon,
    ),
    chipTheme: ChipThemeData(
      showCheckmark: false,
      labelStyle: text.bodyLarge,
      backgroundColor: Palette.cardSolid,
      selectedColor: Palette.cocoa,
      secondaryLabelStyle: text.bodyLarge?.copyWith(color: Palette.cream),
      side: const BorderSide(color: Palette.cocoa),
      shape: const StadiumBorder(),
      padding: const EdgeInsets.symmetric(horizontal: Space.s, vertical: 10),
    ),
    switchTheme: SwitchThemeData(
      thumbColor: WidgetStateProperty.resolveWith(
        (s) => s.contains(WidgetState.selected) ? Palette.cream : null,
      ),
      trackColor: WidgetStateProperty.resolveWith(
        (s) => s.contains(WidgetState.selected) ? Palette.cocoa : null,
      ),
    ),
    sliderTheme: const SliderThemeData(
      activeTrackColor: Palette.cocoa,
      thumbColor: Palette.cocoa,
      inactiveTrackColor: Palette.divider,
    ),
    inputDecorationTheme: InputDecorationTheme(
      border: fieldBorder,
      enabledBorder: fieldBorder,
      focusedBorder: fieldBorder.copyWith(
        borderSide: const BorderSide(color: Palette.cocoa, width: 2),
      ),
      labelStyle: const TextStyle(color: Palette.cocoaMuted),
      floatingLabelStyle: const TextStyle(color: Palette.cocoa),
      helperMaxLines: 3,
      errorMaxLines: 3,
      contentPadding: const EdgeInsets.symmetric(
        horizontal: Space.m,
        vertical: 18,
      ),
    ),
    navigationBarTheme: NavigationBarThemeData(
      backgroundColor: Palette.navBar,
      indicatorColor: Palette.honey,
      height: 80,
      labelBehavior: NavigationDestinationLabelBehavior.alwaysShow,
      iconTheme: WidgetStateProperty.resolveWith(
        (s) => IconThemeData(
          size: Space.icon,
          color: s.contains(WidgetState.selected)
              ? Palette.cocoa
              : Palette.cocoaMuted,
        ),
      ),
      labelTextStyle: WidgetStateProperty.resolveWith(
        (s) => TextStyle(
          fontSize: TypeScale.caption,
          color: s.contains(WidgetState.selected)
              ? Palette.cocoa
              : Palette.cocoaMuted,
        ),
      ),
    ),
    snackBarTheme: const SnackBarThemeData(
      backgroundColor: Palette.cocoa,
      contentTextStyle: TextStyle(
        color: Palette.cream,
        fontSize: TypeScale.body,
      ),
      behavior: SnackBarBehavior.floating,
    ),
    dividerTheme: const DividerThemeData(color: Palette.divider),
  );
}
