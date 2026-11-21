import 'dart:typed_data';

import 'package:flutter_test/flutter_test.dart';
import 'package:friendly_voice/domain/audio_files.dart';

import '../../support/app_harness.dart';
import '../../support/fakes.dart';

PickedAudio wav(int kb) =>
    PickedAudio(name: 'vo.wav', bytes: Uint8List(kb * 1024));

void main() {
  testWidgets('chooses a file, adds it as a sample and generates the voice', (
    tester,
  ) async {
    final core = FakeCoreWithPeople.make();
    final picker = FakeAudioPicker([wav(10), wav(20)]);
    await pumpHome(tester, core: core, picker: picker);
    expect(find.text('2. Audio samples'), findsOneWidget);

    await press(tester, 'Choose a .wav or .flac file');
    expect(
      find.text('File chosen (10 KB). Fill in the text and add it.'),
      findsOneWidget,
    );
    await type(tester, 'What is said in the recording (optional)', 'Bom dia');
    await press(tester, 'Add sample');
    expect(find.text('Sample added (6.0s, mean pitch 210 Hz)'), findsOneWidget);
    expect(core.samples.single, (
      personId: 'spk-1',
      bytes: 10240,
      transcript: 'Bom dia',
    ));

    await press(tester, 'Add sample');
    expect(find.text('Choose a .wav or .flac file first.'), findsOneWidget);

    await press(tester, 'Choose a .wav or .flac file');
    await press(tester, 'Add sample');
    await press(tester, 'Generate voice');
    expect(
      find.text('Voice generated! pt-BR · female voice · 2 sample(s), 12s'),
      findsOneWidget,
    );
  });

  testWidgets(
    'a cancelled choice keeps the status and generating needs material',
    (tester) async {
      final core = FakeCoreWithPeople.make();
      await pumpHome(
        tester,
        core: core,
        picker: FakeAudioPicker([wav(10), null]),
      );
      await press(tester, 'Choose a .wav or .flac file');
      await press(tester, 'Choose a .wav or .flac file'); // cancelled
      expect(
        find.text('File chosen (10 KB). Fill in the text and add it.'),
        findsOneWidget,
      );
      await press(tester, 'Generate voice');
      expect(
        find.text('⚠️ not enough voice material: 0 sample(s)'),
        findsOneWidget,
      );
    },
  );

  testWidgets('without a person the samples ask to register first', (
    tester,
  ) async {
    await pumpHome(tester, picker: FakeAudioPicker([wav(1)]));
    await press(tester, 'Choose a .wav or .flac file');
    await press(tester, 'Add sample');
    expect(find.text('Register and select a person first.'), findsOneWidget);
  });
}
