import 'package:material_ui/material_ui.dart';

/// The status line at the top of the screen (result or error of the last
/// action).
class StatusMessage extends StatelessWidget {
  const new(this.text, {super.key});

  final String text;

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.only(bottom: 8),
      child: Text(text),
    );
  }
}
