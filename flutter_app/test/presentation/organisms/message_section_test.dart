import 'package:flutter_test/flutter_test.dart';
import 'package:friendly_voice/domain/app_settings.dart';
import 'package:friendly_voice/presentation/state/message.dart';
import 'package:material_ui/material_ui.dart';

import '../../support/app_harness.dart';
import '../../support/fakes.dart';

Future<void> choose(WidgetTester tester, String current, String option) async {
  final dropdown = find.text(current);
  await tester.ensureVisible(dropdown);
  await tester.tap(dropdown);
  await tester.pumpAndSettle();
  await tester.tap(find.text(option).last);
  await tester.pumpAndSettle();
}

void main() {
  testWidgets('speaks the theme with its suggested tone and plays it', (
    tester,
  ) async {
    final player = FakeAudioPlayer();
    final core = await pumpHome(
      tester,
      core: FakeCoreWithPeople.make(),
      settings: AppSettings(listener: 'Evandro'),
      overrides: [audioPlayerProvider.overrideWithValue(player)],
    );
    expect(find.text('3. Message'), findsOneWidget);
    expect(
      find.text('Initial tone suggested for this theme: cheerful'),
      findsOneWidget,
    );
    await choose(tester, 'motivation', 'calm');
    expect(
      find.text('Initial tone suggested for this theme: calm'),
      findsOneWidget,
    );

    await press(tester, 'Speak now');

    expect(core.spoken.single.theme, 'calm');
    expect(core.spoken.single.tone, isNull, reason: 'suggested by the theme');
    expect(core.spoken.single.listener, 'Evandro');
    expect(
      find.text('💬 Bom dia, Evandro!\nTone: calm · pt-BR'),
      findsOneWidget,
    );
    expect(player.played, ['/outbox/1.wav']);
  });

  testWidgets('free text, a chosen tone and a pitch shift are sent', (
    tester,
  ) async {
    final core = await pumpHome(
      tester,
      core: FakeCoreWithPeople.make(),
      overrides: [audioPlayerProvider.overrideWithValue(FakeAudioPlayer())],
    );
    await type(
      tester,
      "Or your own text, in the person's language (optional)",
      'Boa noite.',
    );
    await choose(tester, 'Suggested by the theme', 'warm — soft and caring');
    final slider = find.byType(Slider);
    await tester.ensureVisible(slider);
    await tester.drag(slider, const Offset(-1000, 0));
    await tester.pumpAndSettle();
    expect(find.text('Pitch shift: -6 semitones (lower)'), findsOneWidget);

    await press(tester, 'Speak now');

    final s = core.spoken.single;
    expect((s.text, s.tone, s.pitchShift), ('Boa noite.', 'warm', -6));
  });

  testWidgets('without a person it asks to register first', (tester) async {
    await pumpHome(
      tester,
      overrides: [audioPlayerProvider.overrideWithValue(FakeAudioPlayer())],
    );
    await press(tester, 'Speak now');
    expect(find.text('Register and select a person first.'), findsOneWidget);
  });
}
