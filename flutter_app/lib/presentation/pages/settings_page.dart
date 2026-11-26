import 'dart:async';

import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:friendly_voice/domain/app_settings.dart';
import 'package:friendly_voice/presentation/atoms/atoms.dart';
import 'package:friendly_voice/presentation/state/providers.dart';
import 'package:friendly_voice/presentation/templates/templates.dart';
import 'package:material_ui/material_ui.dart';

/// "Settings" tab: the listener's name, saved as it is typed. The voice
/// server is part of the build and only shown.
class SettingsPage extends ConsumerWidget {
  const new({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final connection = ref.watch(connectionProvider).value;
    return SkyPage(
      header: const ScreenTitle('Settings'),
      children: [
        if (connection != null) ...[
          SoftCard(
            children: [
              LiveField(
                label: 'Your name',
                helper:
                    'Used in the messages when a voice has no nickname for '
                    'you.',
                initial: connection.settings.listener ?? '',
                capitalization: TextCapitalization.words,
                onChanged: (name) => unawaited(
                  ref
                      .read(connectionProvider.notifier)
                      .save(AppSettings(listener: name)),
                ),
              ),
            ],
          ),
          SoftCard(
            children: [
              const CardTitle('Voice engine'),
              MutedText(connection.core.engineName),
            ],
          ),
        ],
      ],
    );
  }
}
