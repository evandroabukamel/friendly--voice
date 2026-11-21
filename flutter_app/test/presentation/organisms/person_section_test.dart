import 'package:flutter_test/flutter_test.dart';

import '../../support/app_harness.dart';
import '../../support/fakes.dart';

void main() {
  testWidgets('registers a person with consent and selects them', (
    tester,
  ) async {
    final core = await pumpHome(tester);
    expect(find.text('1. Person of the voice'), findsOneWidget);

    await type(tester, 'Name (e.g. Grandma Lucia)', 'Vó');
    await type(tester, 'Relationship (e.g. grandmother)', 'grandmother');
    await type(tester, 'Who authorized the use of the voice (required)', 'Vó');
    await type(
      tester,
      'Language of the recordings (e.g. pt-BR, en, es; default pt-BR)',
      'es',
    );
    await press(tester, 'Register person');

    expect(find.text('Person registered: Vó'), findsOneWidget);
    expect(core.people.single.language, 'es');
    expect(find.text('Selected person'), findsOneWidget);
    expect(find.text('Vó · es (voice not generated)'), findsOneWidget);
  });

  testWidgets('without consent the failure is shown and nobody is added', (
    tester,
  ) async {
    final core = await pumpHome(tester);
    await type(tester, 'Name (e.g. Grandma Lucia)', 'Vó');
    await press(tester, 'Register person');

    expect(
      find.text(
        "⚠️ the consent of 'Vó' must be recorded before cloning the voice",
      ),
      findsOneWidget,
    );
    expect(core.people, isEmpty);
  });

  testWidgets('people with a voice show it and the first one is selected', (
    tester,
  ) async {
    final core = FakeCoreWithPeople.make();
    await pumpHome(tester, core: core);
    expect(find.text('Vó · pt-BR (voice ready · female)'), findsOneWidget);
  });
}
