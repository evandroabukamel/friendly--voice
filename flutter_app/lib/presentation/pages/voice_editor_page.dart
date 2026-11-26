import 'dart:async';

import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:friendly_voice/domain/voice_core.dart';
import 'package:friendly_voice/l10n/app_localizations.dart';
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
    final l = AppLocalizations.of(context);
    final provider = voiceEditorProvider(id);
    final state = ref.watch(provider);
    final editor = ref.read(provider.notifier);
    final catalog = ref.watch(catalogProvider);
    final languages = catalog?.languages ?? const <LanguageOption>[];
    final language =
        state.language ??
        matchLanguage([
          for (final lang in languages) lang.code,
        ], View.of(context).platformDispatcher.locale.toLanguageTag());
    final person = state.person;
    final registered = person != null;
    final enough = state.recordedSecs >= minVoiceSecs;
    return PopScope(
      onPopInvokedWithResult: (didPop, _) {
        if (didPop) editor.leave();
      },
      child: SkyScreen(
        header: ScreenHeader(
          registered ? l.importVoiceTitleEdit : l.importVoiceTitleNew,
        ),
        children: [
          SoftCard(
            children: [
              // Who the voice is and who allowed it are fixed once the
              // voice is registered (consent is recorded with it).
              LiveField(
                label: l.voiceNameHint,
                initial: state.name,
                enabled: !registered,
                error: state.missingName ? l.voiceNameRequired : null,
                capitalization: TextCapitalization.words,
                onChanged: editor.editName,
              ),
              LiveField(
                label: l.voiceConsentHint,
                helper: l.voiceConsentHelp,
                initial: state.consentBy,
                enabled: !registered,
                error: state.missingConsent ? l.voiceConsentRequired : null,
                capitalization: TextCapitalization.words,
                onChanged: editor.editConsent,
              ),
              LabeledDropdown<String>(
                label: l.voiceLanguageHint,
                value: language,
                options: [for (final lang in languages) (lang.code, lang.name)],
                onChanged: registered
                    ? null
                    : (code) {
                        if (code != null) editor.editLanguage(code);
                      },
              ),
              LiveField(
                label: l.voiceListenerHint,
                helper: l.voiceListenerHelp,
                initial: state.listenerName,
                capitalization: TextCapitalization.words,
                onChanged: editor.editListenerName,
              ),
            ],
          ),
          SoftCard(
            children: [
              CardTitle(l.samplesTitle),
              const SizedBox(height: Space.s),
              MutedText(
                registered
                    ? recordingsLabel(
                        l,
                        state.samples.length,
                        state.recordedSecs,
                      )
                    : l.samplesFillFirst,
              ),
              const SizedBox(height: Space.m),
              OutlinedButton.icon(
                onPressed: state.busy
                    ? null
                    : () {
                        if (language != null && state.language == null) {
                          editor.editLanguage(language);
                        }
                        unawaited(editor.addRecordings());
                      },
                icon: const Icon(Icons.audio_file_outlined),
                label: Text(l.samplesAddFiles),
              ),
              const SizedBox(height: Space.s),
              OutlinedButton.icon(
                onPressed: state.work != VoiceWork.idle
                    ? null
                    : () {
                        if (language != null && state.language == null) {
                          editor.editLanguage(language);
                        }
                        unawaited(editor.toggleRecording());
                      },
                icon: Icon(
                  state.recording ? Icons.stop_rounded : Icons.mic_none,
                ),
                label: Text(state.recording ? l.samplesStop : l.samplesRecord),
              ),
              if (state.recordingSecs case final secs?)
                Padding(
                  padding: const EdgeInsets.only(top: Space.s),
                  child: Text(
                    l.recordingStatus(
                      clock(l, secs),
                      clock(l, maxRecordingSecs),
                    ),
                    textAlign: TextAlign.center,
                  ),
                ),
              const SizedBox(height: Space.s),
              for (final (i, s) in state.samples.indexed)
                SampleRow(number: i + 1, sample: s),
              const SizedBox(height: Space.m),
              MutedText(
                person?.voiceReady ?? false
                    ? l.createVoiceHintReady
                    : l.createVoiceHint,
              ),
              const SizedBox(height: Space.m),
              FilledButton(
                onPressed: state.busy || !enough ? null : editor.createVoice,
                child: Text(
                  state.work == VoiceWork.creating
                      ? l.createVoiceRunning
                      : l.createVoice,
                ),
              ),
              if (state.busy)
                const Padding(
                  padding: EdgeInsets.only(top: Space.m),
                  child: LinearProgressIndicator(),
                ),
            ],
          ),
          if (registered)
            OutlinedButton.icon(
              onPressed: () => _confirmDelete(context, editor),
              icon: const Icon(Icons.delete_outline),
              label: Text(l.deleteVoice),
            ),
        ],
      ),
    );
  }

  Future<void> _confirmDelete(BuildContext context, VoiceEditor editor) async {
    final yes = await showDialog<bool>(
      context: context,
      builder: (context) {
        final l = AppLocalizations.of(context);
        return AlertDialog(
          title: Text(l.deleteVoiceTitle),
          content: Text(l.deleteVoiceMessage),
          actions: [
            TextButton(
              onPressed: () => Navigator.of(context).pop(false),
              child: Text(l.actionCancel),
            ),
            TextButton(
              onPressed: () => Navigator.of(context).pop(true),
              child: Text(l.deleteVoiceConfirm),
            ),
          ],
        );
      },
    );
    if (!(yes ?? false)) return;
    if (await editor.delete() && context.mounted) Navigator.of(context).pop();
  }
}

/// "1:05" in the language's clock format.
String clock(AppLocalizations l, int secs) =>
    l.clockFormat(secs ~/ 60, (secs % 60).toString().padLeft(2, '0'));
