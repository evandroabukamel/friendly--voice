import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:friendly_voice/presentation/atoms/atoms.dart';
import 'package:friendly_voice/presentation/molecules/molecules.dart';
import 'package:friendly_voice/presentation/state/people.dart';
import 'package:friendly_voice/presentation/state/providers.dart';
import 'package:material_ui/material_ui.dart';

/// "1. Person of the voice": registration (with consent) and selection.
class PersonSection extends ConsumerStatefulWidget {
  const new({super.key});

  @override
  ConsumerState<PersonSection> createState() => _PersonSectionState();
}

class _PersonSectionState extends ConsumerState<PersonSection> {
  final _name = TextEditingController();
  final _relationship = TextEditingController();
  final _consent = TextEditingController();
  final _language = TextEditingController();

  @override
  void dispose() {
    for (final c in [_name, _relationship, _consent, _language]) {
      c.dispose();
    }
    super.dispose();
  }

  Future<void> _register() => ref
      .read(statusProvider.notifier)
      .run(
        () async => ref
            .read(peopleProvider.notifier)
            .register(
              name: _name.text,
              relationship: _relationship.text,
              consentBy: _consent.text,
              language: _language.text,
            ),
      );

  @override
  Widget build(BuildContext context) {
    final people = ref.watch(peopleProvider);
    final selected = ref.watch(selectedPersonProvider);
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        const SectionTitle('1. Person of the voice'),
        HintTextField(hint: 'Name (e.g. Grandma Lucia)', controller: _name),
        HintTextField(
          hint: 'Relationship (e.g. grandmother)',
          controller: _relationship,
        ),
        HintTextField(
          hint: 'Who authorized the use of the voice (required)',
          controller: _consent,
        ),
        HintTextField(
          hint:
              'Language of the recordings (e.g. pt-BR, en, es; default pt-BR)',
          controller: _language,
        ),
        ActionButton(label: 'Register person', onPressed: _register),
        LabeledDropdown<String>(
          label: 'Selected person',
          options: [for (final p in people) (p.id, personLabel(p))],
          value: selected?.id,
          onChanged: (id) =>
              ref.read(selectedPersonIdProvider.notifier).selected = id,
        ),
      ],
    );
  }
}
