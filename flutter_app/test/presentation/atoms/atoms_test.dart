import 'package:flutter_test/flutter_test.dart';
import 'package:friendly_voice/presentation/atoms/atoms.dart';
import 'package:material_ui/material_ui.dart';

Widget host(Widget child) => MaterialApp(home: Scaffold(body: child));

void main() {
  testWidgets(
    'SectionTitle is bold and 18 logical pixels, like the Kotlin app',
    (tester) async {
      await tester.pumpWidget(
        host(const SectionTitle('1. Person of the voice')),
      );
      final text = tester.widget<Text>(find.text('1. Person of the voice'));
      expect(text.style?.fontSize, 18);
      expect(text.style?.fontWeight, FontWeight.bold);
    },
  );

  testWidgets('HintTextField shows its hint and edits its controller', (
    tester,
  ) async {
    final controller = TextEditingController();
    await tester.pumpWidget(
      host(
        HintTextField(
          hint: 'Name (e.g. Grandma Lucia)',
          controller: controller,
        ),
      ),
    );
    expect(find.text('Name (e.g. Grandma Lucia)'), findsOneWidget);
    await tester.enterText(find.byType(TextField), 'Vó');
    expect(controller.text, 'Vó');
  });

  testWidgets('ActionButton spans the width and reports taps', (tester) async {
    var taps = 0;
    await tester.pumpWidget(
      host(ActionButton(label: 'Register person', onPressed: () => taps++)),
    );
    await tester.tap(find.text('Register person'));
    expect(taps, 1);
    final width = tester.getSize(find.byType(ActionButton)).width;
    expect(width, tester.getSize(find.byType(Scaffold)).width);
  });

  testWidgets('a disabled ActionButton ignores taps', (tester) async {
    await tester.pumpWidget(host(const ActionButton(label: 'Speak now')));
    expect(
      tester.widget<FilledButton>(find.byType(FilledButton)).onPressed,
      isNull,
    );
  });

  testWidgets('OptionDropdown shows the labels and reports the chosen value', (
    tester,
  ) async {
    String? chosen;
    await tester.pumpWidget(
      host(
        OptionDropdown<String>(
          options: const [('calm', 'calm'), ('warm', 'warm')],
          value: 'calm',
          onChanged: (v) => chosen = v,
        ),
      ),
    );
    await tester.tap(find.text('calm'));
    await tester.pumpAndSettle();
    await tester.tap(find.text('warm').last);
    await tester.pumpAndSettle();
    expect(chosen, 'warm');
  });

  testWidgets('StatusMessage shows the latest status', (tester) async {
    await tester.pumpWidget(host(const StatusMessage('Processing…')));
    expect(find.text('Processing…'), findsOneWidget);
  });
}
