import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:friendly_voice/domain/app_settings.dart';
import 'package:friendly_voice/presentation/atoms/atoms.dart';
import 'package:friendly_voice/presentation/state/providers.dart';
import 'package:material_ui/material_ui.dart';

/// "Settings": the listener's name and the optional neural voice server.
class SettingsSection extends ConsumerStatefulWidget {
  const new({super.key});

  @override
  ConsumerState<SettingsSection> createState() => _SettingsSectionState();
}

class _SettingsSectionState extends ConsumerState<SettingsSection> {
  final _listener = TextEditingController();
  final _engineUrl = TextEditingController();
  var _filled = false;

  @override
  void dispose() {
    _listener.dispose();
    _engineUrl.dispose();
    super.dispose();
  }

  Future<void> _save() => ref.read(statusProvider.notifier).run(() async {
    final connection = await ref
        .read(connectionProvider.notifier)
        .save(
          AppSettings(listener: _listener.text, engineUrl: _engineUrl.text),
        );
    return 'Voice engine: ${connection.core.engineName}';
  });

  @override
  Widget build(BuildContext context) {
    final connection = ref.watch(connectionProvider);
    if (connection case AsyncData(:final value) when !_filled) {
      _filled = true;
      _listener.text = value.settings.listener ?? '';
      _engineUrl.text = value.settings.engineUrl ?? '';
    }
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        const SectionTitle('Settings'),
        HintTextField(
          hint: 'Your name (to personalize the messages)',
          controller: _listener,
        ),
        HintTextField(
          hint: 'Neural voice server URL (optional)',
          controller: _engineUrl,
          keyboardType: TextInputType.url,
        ),
        ActionButton(label: 'Save settings', onPressed: _save),
      ],
    );
  }
}
