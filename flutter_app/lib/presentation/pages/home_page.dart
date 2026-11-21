import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:friendly_voice/presentation/organisms/message_section.dart';
import 'package:friendly_voice/presentation/organisms/person_section.dart';
import 'package:friendly_voice/presentation/organisms/samples_section.dart';
import 'package:friendly_voice/presentation/organisms/schedule_section.dart';
import 'package:friendly_voice/presentation/organisms/settings_section.dart';
import 'package:friendly_voice/presentation/state/providers.dart';
import 'package:friendly_voice/presentation/templates/sectioned_page.dart';
import 'package:material_ui/material_ui.dart';

/// Single screen: settings → person → samples → message → schedule.
class HomePage extends ConsumerWidget {
  const new({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return SectionedPage(
      status: ref.watch(statusLineProvider),
      sections: const [
        SettingsSection(),
        PersonSection(),
        SamplesSection(),
        MessageSection(),
        ScheduleSection(),
      ],
    );
  }
}
