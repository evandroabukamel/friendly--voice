import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:friendly_voice/presentation/atoms/atoms.dart';
import 'package:friendly_voice/presentation/design/tokens.dart';
import 'package:friendly_voice/presentation/molecules/molecules.dart';
import 'package:friendly_voice/presentation/pages/reminder_editor_page.dart';
import 'package:friendly_voice/presentation/state/reminders.dart';
import 'package:friendly_voice/presentation/templates/templates.dart';
import 'package:material_ui/material_ui.dart';

/// "Reminders" tab: every reminder with its switch; tap to edit, swipe to
/// delete.
class RemindersPage extends ConsumerWidget {
  const new({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final reminders = ref.watch(remindersProvider);
    final notifier = ref.read(remindersProvider.notifier);
    return SkyPage(
      header: const ScreenTitle('Reminders'),
      children: [
        if (reminders.isEmpty)
          const Padding(
            padding: EdgeInsets.only(top: Space.xl),
            child: SkyText(
              'No reminders yet.\n'
              'Tap + to hear someone you love at the right time.',
              textAlign: TextAlign.center,
            ),
          ),
        for (final r in reminders)
          Dismissible(
            key: ValueKey(r.id),
            direction: DismissDirection.endToStart,
            background: const Align(
              alignment: Alignment.centerRight,
              child: Padding(
                padding: EdgeInsets.only(right: Space.l),
                child: Icon(Icons.delete_outline, size: Space.icon),
              ),
            ),
            onDismissed: (_) => notifier.remove(r.id),
            child: ReminderCard(
              reminder: r,
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
