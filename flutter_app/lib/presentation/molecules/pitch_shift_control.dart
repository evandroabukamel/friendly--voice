import 'package:friendly_voice/presentation/atoms/atoms.dart';
import 'package:material_ui/material_ui.dart';

/// Caption of the pitch shift, in whole semitones.
String pitchShiftLabel(int semitones) => switch (semitones) {
  > 0 => 'Pitch shift: +$semitones semitones (higher)',
  < 0 => 'Pitch shift: $semitones semitones (lower)',
  _ => "Pitch shift: 0 (the person's natural pitch)",
};

/// Slider from -[max] to +[max] semitones with its caption.
class PitchShiftControl extends StatelessWidget {
  const new({
    required this.value,
    required this.max,
    required this.onChanged,
    super.key,
  });

  final int value;
  final int max;
  final ValueChanged<int> onChanged;

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        FieldLabel(pitchShiftLabel(value)),
        Slider(
          value: value.toDouble(),
          min: -max.toDouble(),
          max: max.toDouble(),
          divisions: max * 2,
          onChanged: (v) => onChanged(v.round()),
        ),
      ],
    );
  }
}
