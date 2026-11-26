import 'dart:typed_data';

import 'package:flutter_test/flutter_test.dart';
import 'package:friendly_voice/domain/audio_files.dart';
import 'package:friendly_voice/domain/voice_core.dart';
import 'package:material_ui/material_ui.dart';

import '../support/app_harness.dart';
import '../support/fakes.dart';

PickedAudio wav(String name) =>
    PickedAudio(name: name, bytes: Uint8List.fromList([1, 2, 3]));

Future<void> back(WidgetTester tester) async {
  await tester.state<NavigatorState>(find.byType(Navigator)).maybePop();
  await tester.pumpAndSettle();
}

Future<void> openVoices(WidgetTester tester) => openTab(tester, 'Voices');

Future<void> openNewVoice(WidgetTester tester) async {
  await openVoices(tester);
  await tester.tap(find.byTooltip('Add voice'));
  await tester.pumpAndSettle();
}

void main() {
  testWidgets('a voice card shows its language, state and recordings', (
    tester,
  ) async {
    await pumpApp(tester, core: FakeCoreWithPeople.make());
    await openVoices(tester);

    expect(find.text('Vó'), findsOneWidget);
    expect(find.text('Portuguese (Brazil)'), findsOneWidget);
    expect(find.text('Voice ready'), findsOneWidget);
    expect(find.text('2 recordings · 12 seconds'), findsOneWidget);
    expect(find.text('Ana'), findsOneWidget);
    expect(find.text('Voice not created yet'), findsOneWidget);
    expect(find.text('0 recordings · 0 seconds'), findsOneWidget);
  });

  testWidgets('adding recordings first asks for the name and the consent', (
    tester,
  ) async {
    final core = await pumpApp(tester);
    await openNewVoice(tester);

    await tapText(tester, 'Add audio files');
    expect(find.text('Enter the name.'), findsOneWidget);
    expect(
      find.text('Enter who authorized the use of this voice.'),
      findsOneWidget,
    );
    expect(core.people, isEmpty);
  });

  testWidgets('a new voice: register with consent, add recordings, create', (
    tester,
  ) async {
    final picker = FakeAudioPicker([
      [wav('a.wav')],
      [wav('b.wav')],
    ]);
    final core = await pumpApp(tester, picker: picker);
    await openNewVoice(tester);

    await type(tester, 'Name (e.g. Grandma Lucia)', 'Lucia');
    await type(tester, 'Who authorized the use of this voice', 'Lucia');
    await choose(tester, 'Language', 'Spanish');
    await tapText(tester, 'Add audio files');
    expect(core.people.single.name, 'Lucia');
    expect(core.people.single.consentBy, 'Lucia');
    expect(core.people.single.language, 'es');
    expect(find.text('Recording 1'), findsOneWidget);
    expect(
      tester.widget<FilledButton>(find.byType(FilledButton)).onPressed,
      isNull,
      reason: '6 s is not enough to create the voice',
    );

    await tapText(tester, 'Add audio files');
    expect(find.text('2 recordings · 12 seconds'), findsOneWidget);
    await tapText(tester, 'Create voice');
    expect(core.people.single.voiceReady, isTrue);
    expect(
      find.text('Voice created! It is ready for your reminders.'),
      findsOneWidget,
    );

    await back(tester);
    expect(find.text('Voice ready'), findsOneWidget);
  });

  testWidgets('leaving a filled-in new voice keeps it', (tester) async {
    final core = await pumpApp(tester);
    await openNewVoice(tester);
    await type(tester, 'Name (e.g. Grandma Lucia)', 'Lucia');
    await type(tester, 'Who authorized the use of this voice', 'Lucia');
    await back(tester);

    expect(core.people.single.name, 'Lucia');
    expect(find.text('Voice saved. Now add recordings.'), findsOneWidget);
    expect(find.text('Lucia'), findsOneWidget);
  });

  testWidgets('leaving an untouched new voice keeps nothing', (tester) async {
    final core = await pumpApp(tester);
    await openNewVoice(tester);
    await back(tester);
    expect(core.people, isEmpty);
  });

  testWidgets('several files chosen at once are all added', (tester) async {
    final picker = FakeAudioPicker([
      [wav('a.wav'), wav('b.wav')],
    ]);
    final core = await pumpApp(
      tester,
      core: FakeCoreWithPeople.make(),
      picker: picker,
    );
    await openVoices(tester);
    await tapText(tester, 'Ana');

    await tapText(tester, 'Add audio files');
    expect(core.samples.where((s) => s.personId == 'spk-2'), hasLength(2));
    expect(find.text('Recordings added: 2'), findsOneWidget);
    expect(find.text('Recording 2'), findsOneWidget);
  });

  testWidgets('how the voice calls you saves as it is typed', (tester) async {
    final core = await pumpApp(tester, core: FakeCoreWithPeople.make());
    await openVoices(tester);
    await tapText(tester, 'Vó');

    await type(tester, 'How this voice calls you', 'Netinho');
    expect(core.people.first.listenerName, 'Netinho');
    await back(tester);
    expect(find.text('Saved.'), findsOneWidget);
  });

  testWidgets('deleting a voice asks first and keeps its reminders off', (
    tester,
  ) async {
    final core = FakeCoreWithPeople.make()
      ..createSchedule(
        const ScheduleRequest(
          speakerId: 'spk-1',
          kind: ScheduleKind.daily,
          at: '07:30',
          theme: 'motivation',
          pitchShift: 0,
        ),
      );
    await pumpApp(tester, core: core);
    await openVoices(tester);
    await tapText(tester, 'Vó');

    await tapText(tester, 'Delete voice');
    expect(find.text('Delete this voice?'), findsOneWidget);
    await tapText(tester, 'Cancel');
    expect(core.people, hasLength(2));

    await tapText(tester, 'Delete voice');
    await tapText(tester, 'Delete');
    expect(core.people.map((p) => p.name), ['Ana']);
    expect(core.schedules.single.enabled, isFalse);
    expect(find.text('Voice deleted.'), findsOneWidget);
    expect(find.text('Vó'), findsNothing);
  });

  testWidgets('a refused recording is explained in the app language', (
    tester,
  ) async {
    final core = FakeCoreWithPeople.make()
      ..rejectSamples = const AppFailure(
        kind: FailureKind.audioTooLong,
        message: 'audio sample rejected: audio too long (200.0s)',
      );
    await pumpApp(
      tester,
      core: core,
      picker: FakeAudioPicker([
        [wav('long.mp3')],
        [wav('long.mp3')],
      ]),
    );
    await openVoices(tester);
    await tapText(tester, 'Ana');
    await tapText(tester, 'Add audio files');
    expect(
      find.text(
        'This recording is longer than 3 minutes: split it into pieces.',
      ),
      findsOneWidget,
    );
  });

  for (final (kind, text) in const [
    (
      FailureKind.audioFormat,
      'This audio format is not supported. Convert the file to WAV.',
    ),
    (
      FailureKind.noAudioTrack,
      'This file has no audio. Use a WAV, MP3 or FLAC file.',
    ),
  ]) {
    testWidgets('a $kind failure shows the Android text', (tester) async {
      final core = FakeCoreWithPeople.make()
        ..rejectSamples = AppFailure(kind: kind, message: 'core words');
      await pumpApp(
        tester,
        core: core,
        picker: FakeAudioPicker([
          [wav('x.bin')],
        ]),
      );
      await openVoices(tester);
      await tapText(tester, 'Ana');
      await tapText(tester, 'Add audio files');
      expect(find.text(text), findsOneWidget);
      expect(find.text('core words'), findsNothing);
    });
  }
}
