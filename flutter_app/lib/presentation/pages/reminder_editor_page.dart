import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:friendly_voice/presentation/atoms/atoms.dart';
import 'package:friendly_voice/presentation/design/tokens.dart';
import 'package:friendly_voice/presentation/molecules/molecules.dart';
import 'package:friendly_voice/presentation/organisms/reminder_cards.dart';
import 'package:friendly_voice/presentation/state/labels.dart';
import 'package:friendly_voice/presentation/state/people.dart';
import 'package:friendly_voice/presentation/state/providers.dart';
import 'package:friendly_voice/presentation/state/reminder_editor.dart';
import 'package:friendly_voice/presentation/templates/templates.dart';
import 'package:material_ui/material_ui.dart';

/// Create or edit a reminder. Every change is saved as soon as the choices
/// make a valid reminder: there is no Save button.
class ReminderEditorPage extends ConsumerWidget {
  const new({this.id, super.key});

  /// `null` = a new reminder.
  final String? id;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final provider = reminderEditorProvider(id);
    final state = ref.watch(provider);
    final editor = ref.read(provider.notifier);
    final catalog = ref.watch(catalogProvider);
    final form = state.form;
    final problem = state.failure ?? state.error.map(formErrorText);
    return SkyScreen(
      header: ScreenHeader(
        state.savedId == null ? 'New reminder' : 'Edit reminder',
      ),
      children: [
        MessageCard(
          form: form,
          voices: ref.watch(readyVoicesProvider),
          catalog: catalog,
          onEdit: editor.change,
          onGoToVoices: () {
            ref.read(tabProvider.notifier).current = AppTab.voices;
            Navigator.of(context).pop();
          },
        ),
        TonePitchCard(form: form, catalog: catalog, onEdit: editor.change),
        RepeatCard(form: form, onEdit: editor.change),
        QuietHoursCard(form: form, onEdit: editor.change),
        if (problem != null) SoftCard(children: [ProblemText(problem)]),
        OutlinedButton.icon(
          onPressed: state.playing ? null : editor.playNow,
          icon: state.playing
              ? const SizedBox.square(
                  dimension: Space.icon,
                  child: CircularProgressIndicator(strokeWidth: 2),
                )
              : const Icon(Icons.play_arrow_rounded),
          label: Text(state.playing ? 'Preparing the message…' : 'Play now'),
        ),
        if (state.savedId != null) ...[
          const SizedBox(height: Space.m),
          TextButton.icon(
            onPressed: () async {
              await editor.delete();
              if (context.mounted) Navigator.of(context).pop();
            },
            icon: const Icon(Icons.delete_outline),
            label: const Text('Delete reminder'),
          ),
        ],
      ],
    );
  }
}

extension<T extends Object> on T? {
  R? map<R>(R Function(T) f) {
    final self = this;
    return self == null ? null : f(self);
  }
}
