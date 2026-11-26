import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_riverpod/misc.dart' show NotifierProviderFamily;
import 'package:friendly_voice/domain/voice_core.dart';
import 'package:friendly_voice/presentation/state/people.dart';
import 'package:friendly_voice/presentation/state/providers.dart';
import 'package:friendly_voice/presentation/state/reminder_form.dart';
import 'package:friendly_voice/presentation/state/reminders.dart';

/// What the reminder screen shows.
final class ReminderEditorState {
  const new({
    required this.form,
    this.savedId,
    this.error,
    this.failure,
    this.playing = false,
  });

  final ReminderForm form;

  /// The stored reminder (`null` until the first valid change of a new one).
  final String? savedId;

  /// Why the current choices do not make a reminder yet.
  final FormError? error;

  /// What the core refused, in its own words.
  final String? failure;
  final bool playing;

  ReminderEditorState copyWith({
    ReminderForm? form,
    String? savedId,
    FormError? Function()? error,
    String? Function()? failure,
    bool? playing,
  }) => ReminderEditorState(
    form: form ?? this.form,
    savedId: savedId ?? this.savedId,
    error: error == null ? this.error : error(),
    failure: failure == null ? this.failure : failure(),
    playing: playing ?? this.playing,
  );
}

/// The reminder screen for one reminder (`null` = a new one). There is no
/// Save button: every change is saved as soon as the choices make a valid
/// reminder. A new reminder is stored on its first change, then updated in
/// place (same id, same on/off state).
final NotifierProviderFamily<ReminderEditor, ReminderEditorState, String?>
reminderEditorProvider = NotifierProvider.autoDispose
    .family<ReminderEditor, ReminderEditorState, String?>(ReminderEditor.new);

class ReminderEditor extends Notifier<ReminderEditorState> {
  new(this.scheduleId);

  final String? scheduleId;

  @override
  ReminderEditorState build() {
    final voices = [for (final v in ref.read(readyVoicesProvider)) v.id];
    final saved = ref
        .read(remindersProvider)
        .where((s) => s.id == scheduleId)
        .firstOrNull;
    final firstTheme = ref.read(catalogProvider)?.themes.firstOrNull?.slug;
    final form = saved == null
        ? ReminderForm(theme: firstTheme)
        : ReminderForm.from(saved.request);
    return ReminderEditorState(
      form: form.withSelectableVoice(voices, editing: saved != null),
      savedId: saved?.id,
    );
  }

  /// Applies one change and saves it when the choices are complete.
  void change(ReminderForm Function(ReminderForm) edit) {
    final form = edit(state.form);
    if (form == state.form) return;
    state = state.copyWith(form: form);
    _save();
  }

  void _save() {
    switch (state.form.toRequest()) {
      case FormInvalid(:final error):
        state = state.copyWith(error: () => error, failure: () => null);
      case FormOk(:final request):
        try {
          final core = requireCore(ref);
          final id = state.savedId;
          final saved = id == null
              ? core.createSchedule(request)
              : core.updateSchedule(id, request);
          state = state.copyWith(
            savedId: saved.id,
            error: () => null,
            failure: () => null,
          );
          ref.read(remindersProvider.notifier).changed();
        } on AppFailure catch (f) {
          state = state.copyWith(error: () => null, failure: () => f.message);
        }
    }
  }

  /// Speaks the reminder's message now, in its voice, and plays it.
  Future<void> playNow() async {
    final form = state.form;
    state = state.copyWith(playing: true);
    await ref.read(messagesProvider.notifier).run(() async {
      final voice =
          form.speakerId ?? (throw const Notice('Choose a voice first.'));
      final listener = ref.read(connectionProvider).value?.settings.listener;
      final m = await requireCore(ref).speak(
        personId: voice,
        theme: form.hasOwnText ? null : form.theme,
        text: form.hasOwnText ? form.text : null,
        tone: form.tone,
        pitchShift: form.pitch,
        listener: listener,
      );
      await ref.read(audioPlayerProvider).play(m.audioPath);
      return '💬 ${m.text}';
    });
    if (ref.mounted) state = state.copyWith(playing: false);
  }

  /// Deletes the stored reminder (nothing to do for an unsaved one).
  Future<void> delete() async {
    final id = state.savedId;
    if (id != null) await ref.read(remindersProvider.notifier).remove(id);
  }
}
