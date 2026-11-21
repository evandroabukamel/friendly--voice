import 'package:material_ui/material_ui.dart';

/// One choice among labelled options (the Kotlin Spinner).
class OptionDropdown<T> extends StatelessWidget {
  const new({
    required this.options,
    required this.value,
    required this.onChanged,
    super.key,
  });

  /// `(value, label)` pairs, in display order.
  final List<(T, String)> options;
  final T? value;
  final ValueChanged<T?> onChanged;

  @override
  Widget build(BuildContext context) {
    return DropdownButton<T>(
      isExpanded: true,
      value: options.any((o) => o.$1 == value) ? value : null,
      items: [
        for (final (v, label) in options)
          DropdownMenuItem<T>(value: v, child: Text(label)),
      ],
      onChanged: onChanged,
    );
  }
}
