import 'package:friendly_voice/presentation/atoms/atoms.dart';
import 'package:material_ui/material_ui.dart';

/// One scrolling screen: the status line, then the sections, with the
/// Kotlin app's padding (16 around, 32 at the bottom).
class SectionedPage extends StatelessWidget {
  const new({required this.status, required this.sections, super.key});

  final String status;
  final List<Widget> sections;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: SafeArea(
        child: SingleChildScrollView(
          padding: const EdgeInsets.fromLTRB(16, 16, 16, 32),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: [StatusMessage(status), ...sections],
          ),
        ),
      ),
    );
  }
}
