import 'package:material_ui/material_ui.dart';

/// Text input that explains itself with a hint, like the Kotlin EditTexts.
class HintTextField extends StatelessWidget {
  const new({
    required this.hint,
    required this.controller,
    this.keyboardType,
    super.key,
  });

  final String hint;
  final TextEditingController controller;
  final TextInputType? keyboardType;

  @override
  Widget build(BuildContext context) {
    return TextField(
      controller: controller,
      keyboardType: keyboardType,
      decoration: InputDecoration(hintText: hint),
    );
  }
}
