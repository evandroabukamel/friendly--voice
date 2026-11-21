import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:friendly_voice/presentation/atoms/atoms.dart';
import 'package:friendly_voice/presentation/state/providers.dart';
import 'package:friendly_voice/presentation/state/samples.dart';
import 'package:material_ui/material_ui.dart';

/// "2. Audio samples": choose a recording, add it, generate the voice.
class SamplesSection extends ConsumerStatefulWidget {
  const new({super.key});

  @override
  ConsumerState<SamplesSection> createState() => _SamplesSectionState();
}

class _SamplesSectionState extends ConsumerState<SamplesSection> {
  final _transcript = TextEditingController();

  @override
  void dispose() {
    _transcript.dispose();
    super.dispose();
  }

  Future<void> _run(Future<String?> Function(SamplesNotifier) action) => ref
      .read(statusProvider.notifier)
      .run(() => action(ref.read(samplesProvider.notifier)));

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        const SectionTitle('2. Audio samples'),
        ActionButton(
          label: 'Choose a .wav or .flac file',
          onPressed: () => _run((s) => s.choose()),
        ),
        HintTextField(
          hint: 'What is said in the recording (optional)',
          controller: _transcript,
        ),
        ActionButton(
          label: 'Add sample',
          onPressed: () => _run((s) => s.add(_transcript.text)),
        ),
        ActionButton(
          label: 'Generate voice',
          onPressed: () => _run((s) => s.generateVoice()),
        ),
      ],
    );
  }
}
