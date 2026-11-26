import 'dart:async';

import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:friendly_voice/l10n/app_localizations.dart';
import 'package:friendly_voice/presentation/atoms/atoms.dart';
import 'package:friendly_voice/presentation/molecules/molecules.dart';
import 'package:friendly_voice/presentation/state/app_language.dart';
import 'package:friendly_voice/presentation/state/providers.dart';
import 'package:friendly_voice/presentation/templates/templates.dart';
import 'package:material_ui/material_ui.dart';

/// "Settings" tab: the listener's name (saved as it is typed) and the app
/// language. The voice server is part of the build and only shown.
class SettingsPage extends ConsumerWidget {
  const new({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final l = AppLocalizations.of(context);
    final connection = ref.watch(connectionProvider).value;
    final settings = connection?.settings;
    final save = ref.read(connectionProvider.notifier).save;
    return SkyPage(
      header: ScreenTitle(l.settingsTitle),
      children: [
        if (connection != null && settings != null)
          SoftCard(
            children: [
              LiveField(
                label: l.settingsListenerHint,
                helper: l.settingsListenerHelp,
                initial: settings.listener ?? '',
                capitalization: TextCapitalization.words,
                onChanged: (name) =>
                    unawaited(save(settings.copyWith(listener: () => name))),
              ),
              LabeledDropdown<String>(
                label: l.settingsLanguageHint,
                value: settings.language ?? '',
                options: [
                  ('', l.languageSystemDefault),
                  for (final (:tag, :name) in appLanguages) (tag, name),
                ],
                onChanged: (tag) async {
                  final chosen = (tag?.isEmpty ?? true) ? null : tag;
                  await save(settings.copyWith(language: () => chosen));
                  ref
                      .read(messagesProvider.notifier)
                      .show((l) => l.settingsSaved);
                },
              ),
              LiveField(
                label: l.settingsEngineHint,
                initial:
                    ref.watch(engineUrlProvider) ?? connection.core.engineName,
                enabled: false,
                onChanged: (_) {},
              ),
            ],
          ),
      ],
    );
  }
}
