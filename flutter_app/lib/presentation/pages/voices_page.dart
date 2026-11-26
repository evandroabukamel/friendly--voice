import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:friendly_voice/l10n/app_localizations.dart';
import 'package:friendly_voice/presentation/atoms/atoms.dart';
import 'package:friendly_voice/presentation/design/tokens.dart';
import 'package:friendly_voice/presentation/molecules/molecules.dart';
import 'package:friendly_voice/presentation/pages/voice_editor_page.dart';
import 'package:friendly_voice/presentation/state/people.dart';
import 'package:friendly_voice/presentation/templates/templates.dart';
import 'package:material_ui/material_ui.dart';

/// "Voices" tab: the people whose voice can speak the reminders.
class VoicesPage extends ConsumerWidget {
  const new({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final l = AppLocalizations.of(context);
    final people = ref.watch(peopleProvider);
    final catalog = ref.watch(catalogProvider);
    return SkyPage(
      header: ScreenTitle(l.voicesTitle),
      children: [
        if (people.isEmpty)
          Padding(
            padding: const EdgeInsets.only(top: Space.xl),
            child: SkyText(l.voicesEmpty, textAlign: TextAlign.center),
          ),
        for (final p in people)
          VoiceCard(
            person: p,
            languageName: languageName(catalog, p.language),
            onTap: () => openVoice(context, p.id),
          ),
      ],
    );
  }
}

/// Opens the voice screen (`null` = a new voice).
Future<void> openVoice(BuildContext context, String? id) =>
    Navigator.of(context)
        .push(MaterialPageRoute<void>(builder: (_) => VoiceEditorPage(id: id)));
