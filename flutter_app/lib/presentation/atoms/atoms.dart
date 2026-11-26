/// Atoms: the smallest pieces of the warm design system. Text drawn on the
/// sky takes its colour from the time of day; text on cards is cocoa.
library;

import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:friendly_voice/presentation/design/tokens.dart';
import 'package:friendly_voice/presentation/state/providers.dart';
import 'package:material_ui/material_ui.dart';

/// Big, light screen title on the sky ("Reminders").
class ScreenTitle extends ConsumerWidget {
  const new(this.text, {super.key});

  final String text;

  @override
  Widget build(BuildContext context, WidgetRef ref) => Padding(
    padding: const EdgeInsets.only(top: Space.l, bottom: Space.m),
    child: Text(
      text,
      style: Theme.of(context).textTheme.headlineLarge
          ?.copyWith(color: ref.watch(daylightProvider).onGradient),
    ),
  );
}

/// Body text on the sky (empty states, hints).
class SkyText extends ConsumerWidget {
  const new(this.text, {super.key, this.textAlign});

  final String text;
  final TextAlign? textAlign;

  @override
  Widget build(BuildContext context, WidgetRef ref) => Text(
    text,
    textAlign: textAlign,
    style: TextStyle(
      fontSize: TypeScale.button,
      height: 1.4,
      fontWeight: FontWeight.w300,
      color: ref.watch(daylightProvider).onGradient,
    ),
  );
}

/// Large, thin digits ("08:00").
class TimeText extends StatelessWidget {
  const new(this.text, {super.key});

  final String text;

  @override
  Widget build(BuildContext context) =>
      Text(text, style: Theme.of(context).textTheme.displayLarge);
}

/// Small section label inside a card ("At", "Between").
class CardLabel extends StatelessWidget {
  const new(this.text, {super.key});

  final String text;

  @override
  Widget build(BuildContext context) => Padding(
    padding: const EdgeInsets.only(top: Space.m, bottom: Space.s),
    child: Text(text, style: Theme.of(context).textTheme.labelMedium),
  );
}

/// Title of a card ("Repeat", "Recordings").
class CardTitle extends StatelessWidget {
  const new(this.text, {super.key});

  final String text;

  @override
  Widget build(BuildContext context) =>
      Text(text, style: Theme.of(context).textTheme.titleLarge);
}

/// Secondary line on a card.
class MutedText extends StatelessWidget {
  const new(this.text, {super.key});

  final String text;

  @override
  Widget build(BuildContext context) =>
      Text(text, style: const TextStyle(color: Palette.cocoaMuted));
}

/// Why something cannot be done yet, in the core's or the form's words.
class ProblemText extends StatelessWidget {
  const new(this.text, {super.key});

  final String text;

  @override
  Widget build(BuildContext context) => Padding(
    padding: const EdgeInsets.symmetric(vertical: Space.s),
    child: Text(text, style: const TextStyle(color: Palette.error)),
  );
}

/// A text field that reports every change (screens save as the user
/// types; there is no Save button). It starts from [initial] and keeps its
/// own text afterwards.
class LiveField extends StatefulWidget {
  const new({
    required this.label,
    required this.initial,
    required this.onChanged,
    this.helper,
    this.error,
    this.suffix,
    this.keyboardType,
    this.capitalization = TextCapitalization.none,
    this.maxLines = 1,
    this.enabled = true,
    super.key,
  });

  final String label;
  final String initial;
  final ValueChanged<String> onChanged;
  final String? helper;
  final String? error;

  /// Unit after the value ("minutes").
  final String? suffix;
  final TextInputType? keyboardType;
  final TextCapitalization capitalization;
  final int maxLines;
  final bool enabled;

  @override
  State<LiveField> createState() => _LiveFieldState();
}

class _LiveFieldState extends State<LiveField> {
  late final _text = TextEditingController(text: widget.initial);

  @override
  void dispose() {
    _text.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) => Padding(
    padding: const EdgeInsets.only(bottom: Space.m),
    child: TextField(
      controller: _text,
      onChanged: widget.onChanged,
      keyboardType: widget.keyboardType,
      textCapitalization: widget.capitalization,
      maxLines: widget.maxLines,
      enabled: widget.enabled,
      style: const TextStyle(fontSize: TypeScale.body),
      decoration: InputDecoration(
        labelText: widget.label,
        helperText: widget.helper,
        errorText: widget.error,
        suffixText: widget.suffix,
      ),
    ),
  );
}

/// A cream card with the standard inner padding.
class SoftCard extends StatelessWidget {
  const new({required this.children, super.key});

  final List<Widget> children;

  @override
  Widget build(BuildContext context) => Card(
    child: Padding(
      padding: const EdgeInsets.all(Space.l),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: children,
      ),
    ),
  );
}
