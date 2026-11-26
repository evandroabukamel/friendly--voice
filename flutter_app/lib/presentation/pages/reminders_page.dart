import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:friendly_voice/l10n/app_localizations.dart';
import 'package:friendly_voice/presentation/atoms/atoms.dart';
import 'package:friendly_voice/presentation/design/tokens.dart';
import 'package:friendly_voice/presentation/molecules/molecules.dart';
import 'package:friendly_voice/presentation/pages/reminder_editor_page.dart';
import 'package:friendly_voice/presentation/state/people.dart';
import 'package:friendly_voice/presentation/state/reminders.dart';
import 'package:friendly_voice/presentation/templates/templates.dart';
import 'package:material_ui/material_ui.dart';

/// "Reminders" tab: every reminder with its switch; tap to edit, swipe to
/// delete.
class RemindersPage extends ConsumerWidget {
  const new({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final l = AppLocalizations.of(context);
    final reminders = ref.watch(remindersProvider);
    final catalog = ref.watch(catalogProvider);
    final notifier = ref.read(remindersProvider.notifier);
    return SkyPage(
      header: ScreenTitle(l.remindersTitle),
      children: [
        if (reminders.isEmpty)
          Padding(
            padding: const EdgeInsets.only(top: Space.xl),
            child: SkyText(l.remindersEmpty, textAlign: TextAlign.center),
          ),
        for (final r in reminders)
          Dismissible(
            key: ValueKey(r.id),
            direction: DismissDirection.endToStart,
            background: const Align(
              alignment: AlignmentDirectional.centerEnd,
              child: Padding(
                padding: EdgeInsetsDirectional.only(end: Space.l),
                child: Icon(Icons.delete_outline, size: Space.icon),
              ),
            ),
            onDismissed: (_) => notifier.remove(r.id),
            child: ReminderCard(
              reminder: r,
              catalog: catalog,
              onTap: () => openReminder(context, r.id),
              onToggle: (on) => notifier.setEnabled(r.id, enabled: on),
            ),
          ),
      ],
    );
  }
}

/// Opens the reminder screen (`null` = a new reminder).
Future<void> openReminder(BuildContext context, String? id) => Navigator.of(
  context,
).push(MaterialPageRoute<void>(builder: (_) => ReminderEditorPage(id: id)));
