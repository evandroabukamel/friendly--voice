import 'package:material_ui/material_ui.dart';

/// Full-width button; disabled when [onPressed] is null.
class ActionButton extends StatelessWidget {
  const new({required this.label, this.onPressed, super.key});

  final String label;
  final VoidCallback? onPressed;

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 4),
      child: SizedBox(
        width: double.infinity,
        child: FilledButton(onPressed: onPressed, child: Text(label)),
      ),
    );
  }
}
