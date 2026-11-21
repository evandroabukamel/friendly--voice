import 'package:flutter_test/flutter_test.dart';
import 'package:friendly_voice/presentation/molecules/molecules.dart';
import 'package:material_ui/material_ui.dart';

Widget host(Widget child) => MaterialApp(home: Scaffold(body: child));

void main() {
  test('pitch shift labels match the Kotlin app', () {
    expect(pitchShiftLabel(2), 'Pitch shift: +2 semitones (higher)');
    expect(pitchShiftLabel(-3), 'Pitch shift: -3 semitones (lower)');
    expect(pitchShiftLabel(0), "Pitch shift: 0 (the person's natural pitch)");
  });

  testWidgets('PitchShiftControl moves in whole semitones within ±max', (
    tester,
  ) async {
    var value = 0;
    await tester.pumpWidget(
      host(PitchShiftControl(value: 0, max: 6, onChanged: (v) => value = v)),
    );
    expect(
      find.text("Pitch shift: 0 (the person's natural pitch)"),
      findsOneWidget,
    );
    final slider = tester.widget<Slider>(find.byType(Slider));
    expect((slider.min, slider.max, slider.divisions), (-6.0, 6.0, 12));
    await tester.drag(find.byType(Slider), const Offset(1000, 0));
    expect(value, 6);
  });

  testWidgets('LabeledDropdown shows its label above the options', (
    tester,
  ) async {
    await tester.pumpWidget(
      host(
        LabeledDropdown<String>(
          label: 'Tone of voice',
          options: const [('warm', 'warm — soft and caring')],
          value: 'warm',
          onChanged: (_) {},
        ),
      ),
    );
    expect(find.text('Tone of voice'), findsOneWidget);
    expect(find.text('warm — soft and caring'), findsOneWidget);
  });
}
