import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:friendly_voice/app/app.dart';
import 'package:friendly_voice/src/rust/frb_generated.dart';
import 'package:material_ui/material_ui.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await RustLib.init();
  runApp(const ProviderScope(child: FriendlyVoiceApp()));
}
