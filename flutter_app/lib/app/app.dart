import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:friendly_voice/presentation/design/theme.dart';
import 'package:friendly_voice/presentation/design/tokens.dart';
import 'package:friendly_voice/presentation/state/providers.dart';
import 'package:friendly_voice/presentation/templates/app_shell.dart';
import 'package:material_ui/material_ui.dart';

/// Root widget: the warm theme, the tabs, and the messages for the user
/// shown as snackbars over whichever screen is open.
class FriendlyVoiceApp extends ConsumerStatefulWidget {
  const new({super.key});

  @override
  ConsumerState<FriendlyVoiceApp> createState() => _FriendlyVoiceAppState();
}

class _FriendlyVoiceAppState extends ConsumerState<FriendlyVoiceApp> {
  final _messenger = GlobalKey<ScaffoldMessengerState>();

  @override
  Widget build(BuildContext context) {
    ref.listen(messagesProvider, (_, message) {
      if (message == null) return;
      _messenger.currentState
        ?..hideCurrentSnackBar()
        ..showSnackBar(
          SnackBar(
            content: Text(message.text),
            backgroundColor: message.isError ? Palette.error : null,
          ),
        );
    });
    return MaterialApp(
      title: 'Friendly Voice',
      debugShowCheckedModeBanner: false,
      scaffoldMessengerKey: _messenger,
      theme: warmTheme(),
      home: const AppShell(),
    );
  }
}
