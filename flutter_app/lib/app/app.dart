import 'package:friendly_voice/presentation/pages/home_page.dart';
import 'package:material_ui/material_ui.dart';

/// Root widget: light Material 3 theme, like the Kotlin app's light theme.
class FriendlyVoiceApp extends StatelessWidget {
  const new({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Friendly Voice',
      debugShowCheckedModeBanner: false,
      theme: ThemeData(colorSchemeSeed: Colors.blue),
      home: const HomePage(),
    );
  }
}
