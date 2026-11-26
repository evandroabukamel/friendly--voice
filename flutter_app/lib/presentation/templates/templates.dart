/// Templates: the sky behind every screen and the page layout on it.
library;

import 'package:flutter/services.dart' show SystemUiOverlayStyle;
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:friendly_voice/presentation/design/tokens.dart';
import 'package:friendly_voice/presentation/state/providers.dart';
import 'package:material_ui/material_ui.dart';

/// The warm gradient of the current time of day, edge to edge, with
/// status-bar icons that stay readable on it.
class DaylightBackground extends ConsumerWidget {
  const new({required this.child, super.key});

  final Widget child;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final daylight = ref.watch(daylightProvider);
    return AnnotatedRegion<SystemUiOverlayStyle>(
      value: daylight.isDark
          ? SystemUiOverlayStyle.light
          : SystemUiOverlayStyle.dark,
      child: AnimatedContainer(
        duration: const Duration(seconds: 2),
        decoration: BoxDecoration(gradient: daylight.linear),
        child: child,
      ),
    );
  }
}

/// A scrolling page on the sky: a header (title or back + title) and its
/// cards, with room left for the floating button and the navigation bar.
class SkyPage extends StatelessWidget {
  const new({required this.header, required this.children, super.key});

  final Widget header;
  final List<Widget> children;

  @override
  Widget build(BuildContext context) => SafeArea(
    bottom: false,
    child: ListView(
      padding: const EdgeInsets.fromLTRB(
        Space.screen,
        0,
        Space.screen,
        Space.listBottom,
      ),
      children: [header, ...children],
    ),
  );
}

/// A full screen pushed over the tabs (reminder or voice editor).
class SkyScreen extends StatelessWidget {
  const new({required this.header, required this.children, super.key});

  final Widget header;
  final List<Widget> children;

  @override
  Widget build(BuildContext context) => Scaffold(
    body: DaylightBackground(
      child: SkyPage(header: header, children: children),
    ),
  );
}
