import 'package:flutter_test/flutter_test.dart';
import 'package:material_ui/material_ui.dart';

import '../support/app_harness.dart';
import '../support/fakes.dart';

/// Opens Ana's voice (registered, no recordings yet).
Future<FakeVoiceCore> openAna(
  WidgetTester tester,
  FakeVoiceRecorder recorder,
) async {
  final core = await pumpApp(
    tester,
    core: FakeCoreWithPeople.make(),
    recorder: recorder,
  );
  await openTab(tester, 'Voices');
  await tapText(tester, 'Ana');
  return core;
}

/// Taps Record without settling: the running timer never settles.
Future<void> startRecording(WidgetTester tester) async {
  await reveal(tester, find.text('Record'));
  await tester.tap(find.text('Record'));
  await tester.pump();
}

/// Taps Stop recording (visible right below Record), then settles.
Future<void> stopRecording(WidgetTester tester) async {
  await tester.tap(find.text('Stop recording'));
  await tester.pumpAndSettle();
}

int anaSamples(FakeVoiceCore core) =>
    core.samples.where((s) => s.personId == 'spk-2').length;

void main() {
  testWidgets('recording from the microphone, with a running timer', (
    tester,
  ) async {
    final recorder = FakeVoiceRecorder();
    final core = await openAna(tester, recorder);

    await startRecording(tester);
    expect(recorder.started, 1);
    expect(find.text('Recording… 0:00 of 3:00'), findsOneWidget);

    await tester.pump(const Duration(seconds: 72));
    expect(find.text('Recording… 1:12 of 3:00'), findsOneWidget);

    await stopRecording(tester);
    expect(recorder.stopped, 1);
    expect(anaSamples(core), 1);
    expect(find.text('Recordings added: 1'), findsOneWidget);
    expect(find.text('Record'), findsOneWidget);
  });

  testWidgets('a recording stops by itself at the 3-minute limit', (
    tester,
  ) async {
    final recorder = FakeVoiceRecorder();
    final core = await openAna(tester, recorder);

    await startRecording(tester);
    await tester.pump(const Duration(minutes: 3));
    await tester.pumpAndSettle();
    expect(recorder.stopped, 1);
    expect(anaSamples(core), 1);
  });

  testWidgets('a refused microphone is explained', (tester) async {
    final recorder = FakeVoiceRecorder()..allowed = false;
    await openAna(tester, recorder);

    await startRecording(tester);
    expect(recorder.started, 0);
    expect(
      find.text('Allow the microphone to record a voice.'),
      findsOneWidget,
    );
  });

  testWidgets('a missing microphone or a failed recording is explained', (
    tester,
  ) async {
    final recorder = FakeVoiceRecorder()..available = false;
    final core = await openAna(tester, recorder);

    await startRecording(tester);
    expect(find.text('The microphone is not available.'), findsOneWidget);

    recorder
      ..available = true
      ..fails = true;
    await startRecording(tester);
    await stopRecording(tester);
    expect(
      find.text('The recording failed. Please try again.'),
      findsOneWidget,
    );
    expect(anaSamples(core), 0);
  });

  testWidgets('leaving the screen while recording discards it', (tester) async {
    final recorder = FakeVoiceRecorder();
    final core = await openAna(tester, recorder);

    await startRecording(tester);
    await tester.state<NavigatorState>(find.byType(Navigator)).maybePop();
    await tester.pumpAndSettle();
    expect(recorder.cancelled, 1);
    expect(anaSamples(core), 0);
  });
}
