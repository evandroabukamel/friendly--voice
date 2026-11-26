import 'dart:async';

import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:friendly_voice/domain/voice_core.dart';
import 'package:friendly_voice/presentation/atoms/atoms.dart';
import 'package:friendly_voice/presentation/design/tokens.dart';
import 'package:friendly_voice/presentation/molecules/molecules.dart';
import 'package:friendly_voice/presentation/state/labels.dart';
import 'package:friendly_voice/presentation/state/people.dart';
import 'package:friendly_voice/presentation/state/voice_editor.dart';
import 'package:friendly_voice/presentation/templates/templates.dart';
import 'package:material_ui/material_ui.dart';

/// Register a voice (with consent), add its recordings and create it.
/// Nothing needs saving: see [VoiceEditor].
class VoiceEditorPage extends ConsumerWidget {
  const new({this.id, super.key});

  /// `null` = a new voice.
  final String? id;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final provider = voiceEditorProvider(id);
    final state = ref.watch(provider);
    final editor = ref.read(provider.notifier);
    final catalog = ref.watch(catalogProvider);
    final codes = [
      for (final l in catalog?.languages ?? const <LanguageOption>[]) l.code,
    ];
    final language =
        state.language ??
        matchLanguage(
          codes,
          View.of(context).platformDispatcher.locale.toLanguageTag(),
        );
    final person = state.person;
    final enough = state.recordedSecs >= minVoiceSecs;
    return PopScope(
      onPopInvokedWithResult: (didPop, _) {
        if (didPop) editor.leave();
      },
      child: SkyScreen(
        header: ScreenHeader(person == null ? 'New voice' : person.name),
        children: [
          SoftCard(
            children: [
              if (person == null) ...[
                LiveField(
                  label: 'Name (e.g. Grandma Lucia)',
                  initial: state.name,
                  error: state.nameError,
                  capitalization: TextCapitalization.words,
                  onChanged: editor.editName,
                ),
                LiveField(
                  label: 'Who authorized the use of this voice',
                  helper:
                      "Required: only use a voice with the person's "
                      'permission.',
                  initial: state.consentBy,
                  error: state.consentError,
                  capitalization: TextCapitalization.words,
                  onChanged: editor.editConsent,
                ),
                LabeledDropdown<String>(
                  label: 'Language',
                  value: language,
                  options: [
                    for (final l
                        in catalog?.languages ?? const <LanguageOption>[])
                      (l.code, l.name),
                  ],
                  onChanged: (code) {
                    if (code != null) editor.editLanguage(code);
                  },
                ),
              ] else ...[
                MutedText(languageName(catalog, person.language)),
                MutedText('Authorized by ${person.consentBy}'),
                const SizedBox(height: Space.m),
              ],
              LiveField(
                label: 'How this voice calls you',
                helper:
                    'Optional, e.g. a nickname. Empty: your name from '
                    'Settings.',
                initial: state.listenerName,
                capitalization: TextCapitalization.words,
                onChanged: editor.editListenerName,
              ),
            ],
          ),
          SoftCard(
            children: [
              const CardTitle('Recordings'),
              const SizedBox(height: Space.s),
              MutedText(
                person == null
                    ? 'Fill in the name and who authorized it, then add '
                          'recordings.'
                    : recordingsLabel(state.samples.length, state.recordedSecs),
              ),
              const SizedBox(height: Space.m),
              OutlinedButton.icon(
                onPressed: state.busy
                    ? null
                    : () {
                        if (language != null && state.language == null) {
                          editor.editLanguage(language);
                        }
                        unawaited(editor.addRecording());
                      },
                icon: const Icon(Icons.audio_file_outlined),
                label: const Text('Add audio files'),
              ),
              const SizedBox(height: Space.s),
              for (final (i, s) in state.samples.indexed)
                SampleRow(number: i + 1, sample: s),
              const SizedBox(height: Space.m),
              MutedText(
                person?.voiceReady ?? false
                    ? 'The voice is ready. Added more recordings? Create it '
                          'again to improve it.'
                    : 'Add at least 10 seconds of recordings (WAV or FLAC, '
                          '2 s to 3 min each), then create the voice.',
              ),
              const SizedBox(height: Space.m),
              FilledButton(
                onPressed: state.busy || !enough ? null : editor.createVoice,
                child: Text(state.busy ? 'Working on it…' : 'Create voice'),
              ),
              if (state.busy)
                const Padding(
                  padding: EdgeInsets.only(top: Space.m),
                  child: LinearProgressIndicator(),
                ),
            ],
          ),
          if (person != null)
            OutlinedButton.icon(
              onPressed: () => _confirmDelete(context, editor),
              icon: const Icon(Icons.delete_outline),
              label: const Text('Delete voice'),
            ),
        ],
      ),
    );
  }

  Future<void> _confirmDelete(BuildContext context, VoiceEditor editor) async {
    final yes = await showDialog<bool>(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('Delete this voice?'),
        content: const Text(
          'Its recordings and the created voice are deleted from this phone '
          'and the voice server. Reminders that use it are turned off and '
          'kept, so you can choose another voice.',
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.of(context).pop(false),
            child: const Text('Cancel'),
          ),
          TextButton(
            onPressed: () => Navigator.of(context).pop(true),
            child: const Text('Delete'),
          ),
        ],
      ),
    );
    if (!(yes ?? false)) return;
    if (await editor.delete() && context.mounted) Navigator.of(context).pop();
  }
}
