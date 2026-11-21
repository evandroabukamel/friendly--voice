import 'package:flutter_test/flutter_test.dart';
import 'package:friendly_voice/presentation/templates/sectioned_page.dart';
import 'package:material_ui/material_ui.dart';

void main() {
  testWidgets('shows the status first, then the sections, and scrolls', (
    tester,
  ) async {
    await tester.pumpWidget(
      MaterialApp(
        home: SectionedPage(
          status: 'Voice engine: local (prosody sketch)',
          sections: [for (var i = 0; i < 30; i++) Text('row $i')],
        ),
      ),
    );
    final status = tester.getTopLeft(
      find.text('Voice engine: local (prosody sketch)'),
    );
    expect(status.dy, lessThan(tester.getTopLeft(find.text('row 0')).dy));
    expect(status.dx, 16);
    await tester.scrollUntilVisible(find.text('row 29'), 300);
    expect(find.text('row 29'), findsOneWidget);
  });
}
