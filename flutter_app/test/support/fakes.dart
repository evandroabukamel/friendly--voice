import 'dart:typed_data';

import 'package:friendly_voice/domain/app_settings.dart';
import 'package:friendly_voice/domain/voice_core.dart';

/// In-memory [SettingsStore].
final class FakeSettingsStore implements SettingsStore {
  new([AppSettings? initial]) : saved = initial ?? AppSettings();

  AppSettings saved;

  @override
  Future<AppSettings> load() async => saved;

  @override
  Future<void> save(AppSettings settings) async => saved = settings;
}

/// Opens [FakeVoiceCore]s and remembers the engine URLs it was given.
final class FakeVoiceCoreFactory implements VoiceCoreFactory {
  new([FakeVoiceCore? core]) : core = core ?? FakeVoiceCore();

  final FakeVoiceCore core;
  final List<String?> openedWith = [];

  @override
  Future<VoiceCore> open({String? engineUrl}) async {
    openedWith.add(engineUrl);
    core.engineName = engineUrl == null
        ? 'local (prosody sketch)'
        : 'neural server (HTTP)';
    return core;
  }
}

/// In-memory [VoiceCore] with the same observable rules as the Rust core
/// for the cases the UI needs (consent required, voice required to speak).
final class FakeVoiceCore implements VoiceCore {
  @override
  String engineName = 'local (prosody sketch)';

  final List<SpeakerSummary> people = [];
  final List<ScheduleSummary> schedules = [];
  final List<ScheduleRequest> scheduleRequests = [];
  final List<({String personId, int bytes, String? transcript})> samples = [];
  final List<
    ({
      String personId,
      String? theme,
      String? text,
      String? tone,
      int pitchShift,
      String? listener,
    })
  >
  spoken = [];
  List<DueMessage> due = [];
  List<Alarm> alarms = [];

  @override
  Catalog catalog() => const Catalog(
    themes: [
      ThemeOption(slug: 'motivation', defaultTone: 'cheerful'),
      ThemeOption(slug: 'calm', defaultTone: 'calm'),
    ],
    tones: [
      ToneOption(
        slug: 'natural',
        description: 'the way the person normally speaks',
      ),
      ToneOption(slug: 'warm', description: 'soft and caring'),
      ToneOption(slug: 'calm', description: 'slow, lower and steady'),
    ],
    languages: [LanguageOption(code: 'pt', name: 'Portuguese')],
    maxPitchShift: 6,
  );

  @override
  List<SpeakerSummary> listPeople() => List.of(people);

  @override
  SpeakerSummary registerPerson({
    required String name,
    String? relationship,
    String? consentBy,
    String? language,
  }) {
    if (consentBy == null || consentBy.trim().isEmpty) {
      throw AppFailure(
        message:
            "the consent of '$name' must be recorded before cloning the voice",
      );
    }
    final person = SpeakerSummary(
      id: 'spk-${people.length + 1}',
      name: name,
      relationship: relationship,
      language: (language == null || language.isEmpty) ? 'pt-BR' : language,
      voiceReady: false,
    );
    people.add(person);
    return person;
  }

  @override
  Future<SampleSummary> addSample({
    required String personId,
    required Uint8List audio,
    String? transcript,
  }) async {
    samples.add((
      personId: personId,
      bytes: audio.length,
      transcript: transcript,
    ));
    return SampleSummary(
      id: 'smp-${samples.length}',
      durationSecs: 6,
      meanPitchHz: 210,
      transcript: transcript,
    );
  }

  @override
  Future<VoiceSummary> buildVoice(String personId) async {
    final i = people.indexWhere((p) => p.id == personId);
    final count = samples.where((s) => s.personId == personId).length;
    if (count < 2) {
      throw AppFailure(message: 'not enough voice material: $count sample(s)');
    }
    final p = people[i];
    people[i] = SpeakerSummary(
      id: p.id,
      name: p.name,
      relationship: p.relationship,
      language: p.language,
      voiceReady: true,
      gender: Gender.female,
    );
    return VoiceSummary(
      language: p.language,
      gender: Gender.female,
      sampleCount: count,
      totalAudioSecs: count * 6,
    );
  }

  @override
  Future<SpokenMessage> speak({
    required String personId,
    String? theme,
    String? text,
    String? tone,
    int pitchShift = 0,
    String? listener,
  }) async {
    spoken.add((
      personId: personId,
      theme: theme,
      text: text,
      tone: tone,
      pitchShift: pitchShift,
      listener: listener,
    ));
    return SpokenMessage(
      text: text ?? 'Bom dia, ${listener ?? 'querido'}!',
      tone: tone ?? 'calm',
      language: 'pt-BR',
      audioPath: '/outbox/${spoken.length}.wav',
    );
  }

  @override
  ScheduleSummary createSchedule(ScheduleRequest request) {
    if (request.at == null && request.kind != ScheduleKind.interval) {
      throw const AppFailure(message: 'provide the time (HH:MM)');
    }
    scheduleRequests.add(request);
    final s = ScheduleSummary(
      id: 'sch-${scheduleRequests.length}',
      speakerId: request.speakerId,
      toneLabel: request.tone ?? 'suggested by the theme',
      nextDue: '21/11 ${request.at ?? '09:00'}',
    );
    schedules.add(s);
    return s;
  }

  @override
  List<ScheduleSummary> listSchedules() => List.of(schedules);

  @override
  void removeSchedule(String id) {
    if (!schedules.any((s) => s.id == id)) {
      throw AppFailure(message: "schedule '$id' not found");
    }
    schedules.removeWhere((s) => s.id == id);
  }

  @override
  Future<List<DueMessage>> runDueSchedules({String? listener}) async {
    final out = due;
    due = [];
    return out;
  }

  @override
  List<Alarm> upcomingAlarms(int limit) => alarms.take(limit).toList();
}

/// A core that already has people, the first one with a generated voice.
abstract final class FakeCoreWithPeople {
  static FakeVoiceCore make() => FakeVoiceCore()
    ..people.addAll(const [
      SpeakerSummary(
        id: 'spk-1',
        name: 'Vó',
        language: 'pt-BR',
        voiceReady: true,
        gender: Gender.female,
      ),
      SpeakerSummary(
        id: 'spk-2',
        name: 'Ana',
        language: 'es',
        voiceReady: false,
      ),
    ]);
}
