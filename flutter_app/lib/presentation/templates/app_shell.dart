import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:friendly_voice/presentation/pages/reminders_page.dart';
import 'package:friendly_voice/presentation/pages/settings_page.dart';
import 'package:friendly_voice/presentation/pages/voices_page.dart';
import 'package:friendly_voice/presentation/state/alarms.dart';
import 'package:friendly_voice/presentation/state/providers.dart';
import 'package:friendly_voice/presentation/templates/templates.dart';
import 'package:material_ui/material_ui.dart';

/// The three tabs — Reminders, Voices, Settings — in a bottom navigation
/// bar over the sky of the time of day.
class AppShell extends ConsumerWidget {
  const new({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    ref.watch(alarmsProvider); // keeps the alarms and deliveries running
    final tab = ref.watch(tabProvider);
    final connected = ref.watch(connectionProvider).hasValue;
    return Scaffold(
      extendBody: true,
      body: DaylightBackground(
        child: IndexedStack(
          index: tab.index,
          children: const [RemindersPage(), VoicesPage(), SettingsPage()],
        ),
      ),
      floatingActionButton: switch (tab) {
        _ when !connected => null,
        AppTab.reminders => FloatingActionButton(
          tooltip: 'Add reminder',
          onPressed: () => openReminder(context, null),
          child: const Icon(Icons.add),
        ),
        AppTab.voices => FloatingActionButton(
          tooltip: 'Add voice',
          onPressed: () => openVoice(context, null),
          child: const Icon(Icons.add),
        ),
        AppTab.settings => null,
      },
      bottomNavigationBar: NavigationBar(
        selectedIndex: tab.index,
        onDestinationSelected: (i) =>
            ref.read(tabProvider.notifier).current = AppTab.values[i],
        destinations: const [
          NavigationDestination(
            icon: Icon(Icons.alarm_outlined),
            selectedIcon: Icon(Icons.alarm),
            label: 'Reminders',
          ),
          NavigationDestination(
            icon: Icon(Icons.mic_none),
            selectedIcon: Icon(Icons.mic),
            label: 'Voices',
          ),
          NavigationDestination(
            icon: Icon(Icons.settings_outlined),
            selectedIcon: Icon(Icons.settings),
            label: 'Settings',
          ),
        ],
      ),
    );
  }
}
