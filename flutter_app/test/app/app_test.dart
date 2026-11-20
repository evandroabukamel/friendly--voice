import 'package:flutter_test/flutter_test.dart';
import 'package:friendly_voice/app/app.dart';
import 'package:material_ui/material_ui.dart';

void main() {
  testWidgets('opens with the app title on a light Material 3 theme', (
    tester,
  ) async {
    await tester.pumpWidget(const FriendlyVoiceApp());

    expect(find.text('Friendly Voice'), findsOneWidget);
    final context = tester.element(find.byType(Scaffold));
    final theme = Theme.of(context);
    expect(theme.useMaterial3, isTrue);
    expect(theme.brightness, Brightness.light);
  });
}
