import 'package:friendly_voice/presentation/atoms/atoms.dart';
import 'package:material_ui/material_ui.dart';

/// A caption followed by a dropdown (e.g. "Theme" + the themes).
class LabeledDropdown<T> extends StatelessWidget {
  const new({
    required this.label,
    required this.options,
    required this.value,
    required this.onChanged,
    super.key,
  });

  final String label;
  final List<(T, String)> options;
  final T? value;
  final ValueChanged<T?> onChanged;

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        FieldLabel(label),
        OptionDropdown<T>(options: options, value: value, onChanged: onChanged),
      ],
    );
  }
}
