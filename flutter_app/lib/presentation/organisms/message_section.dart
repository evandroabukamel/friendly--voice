import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:friendly_voice/domain/voice_core.dart';
import 'package:friendly_voice/presentation/atoms/atoms.dart';
import 'package:friendly_voice/presentation/molecules/molecules.dart';
import 'package:friendly_voice/presentation/state/message.dart';
import 'package:friendly_voice/presentation/state/providers.dart';
import 'package:material_ui/material_ui.dart';

/// "3. Message": theme or free text, tone, pitch shift, "Speak now".
class MessageSection extends ConsumerStatefulWidget {
  const new({super.key});

  @override
  ConsumerState<MessageSection> createState() => _MessageSectionState();
}

class _MessageSectionState extends ConsumerState<MessageSection> {
  final _text = TextEditingController();

  @override
  void initState() {
    super.initState();
    _text.addListener(
      () => ref
          .read(messageDraftProvider.notifier)
          .update((d) => d.copyWith(text: _text.text)),
    );
  }

  @override
  void dispose() {
    _text.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final catalog = ref.watch(catalogProvider);
    final draft = ref.watch(messageDraftProvider);
    final draftNotifier = ref.read(messageDraftProvider.notifier);
    final theme = draft.theme ?? catalog?.themes.firstOrNull?.slug;
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        const SectionTitle('3. Message'),
        LabeledDropdown<String>(
          label: 'Theme',
          options: [
            for (final t in catalog?.themes ?? const <ThemeOption>[])
              (t.slug, t.slug),
          ],
          value: theme,
          onChanged: (v) => draftNotifier.update((d) => d.copyWith(theme: v)),
        ),
        HintTextField(
          hint: "Or your own text, in the person's language (optional)",
          controller: _text,
        ),
        LabeledDropdown<String?>(
          label: 'Tone of voice',
          options: [
            (null, 'Suggested by the theme'),
            for (final t in catalog?.tones ?? const <ToneOption>[])
              (t.slug, '${t.slug} — ${t.description}'),
          ],
          value: draft.tone,
          onChanged: (v) =>
              draftNotifier.update((d) => d.copyWith(tone: () => v)),
        ),
        FieldLabel(
          'Initial tone suggested for this theme: '
          '${suggestedTone(catalog, theme)}',
        ),
        PitchShiftControl(
          value: draft.pitchShift,
          max: (catalog?.maxPitchShift ?? 6).round(),
          onChanged: (v) =>
              draftNotifier.update((d) => d.copyWith(pitchShift: v)),
        ),
        ActionButton(
          label: 'Speak now',
          onPressed: () =>
              ref.read(statusProvider.notifier).run(draftNotifier.speakNow),
        ),
      ],
    );
  }
}
