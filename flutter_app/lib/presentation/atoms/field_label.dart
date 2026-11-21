import 'package:material_ui/material_ui.dart';

/// Plain caption above a control (e.g. "Selected person").
class FieldLabel extends StatelessWidget {
  const new(this.text, {super.key});

  final String text;

  @override
  Widget build(BuildContext context) => Text(text);
}
