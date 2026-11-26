import 'dart:async';
import 'dart:typed_data';

import 'package:friendly_voice/domain/alarms.dart';
import 'package:friendly_voice/domain/app_settings.dart';
import 'package:friendly_voice/domain/audio_files.dart';
import 'package:friendly_voice/domain/audio_player.dart';
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
/// for the cases the UI needs (consent required, enough material for a
/// voice, reminders that round-trip and keep their id and switch). The
/// shared contract in `test/contract` keeps it honest.
final class FakeVoiceCore implements VoiceCore {
  @override
  String engineName = 'local (prosody sketch)';

  final List<SpeakerSummary> people = [];

  /// Every request sent to [createSchedule], in order.
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

  final Map<String, ({ScheduleRequest request, bool enabled})> _schedules = {};
  int _nextSchedule = 0;

  List<ScheduleSummary> get schedules => [
    for (final id in _schedules.keys) _summary(id),
  ];

  static String? _clean(String? v) {
    final t = v?.trim();
    return (t == null || t.isEmpty) ? null : t;
  }

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
    languages: [
      LanguageOption(code: 'pt-BR', name: 'Portuguese (Brazil)'),
      LanguageOption(code: 'es', name: 'Spanish'),
    ],
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
    String? listenerName,
  }) {
    final consent = _clean(consentBy);
    if (consent == null) {
      throw AppFailure(
        message:
            "the consent of '$name' must be recorded before cloning the voice",
      );
    }
    final person = SpeakerSummary(
      id: 'spk-${people.length + 1}',
      name: name,
      relationship: _clean(relationship),
      language: _clean(language) ?? 'pt-BR',
      voiceReady: false,
      consentBy: consent,
      listenerName: _clean(listenerName),
      sampleCount: 0,
      totalSampleSecs: 0,
    );
    people.add(person);
    return person;
  }

  SpeakerSummary _replace(
    String personId,
    SpeakerSummary Function(SpeakerSummary) change,
  ) {
    final i = people.indexWhere((p) => p.id == personId);
    if (i < 0) throw AppFailure(message: "person '$personId' not found");
    return people[i] = change(people[i]);
  }

  static SpeakerSummary _copy(
    SpeakerSummary p, {
    bool? voiceReady,
    Gender? gender,
    String? Function()? listenerName,
    int? sampleCount,
    double? totalSampleSecs,
  }) => SpeakerSummary(
    id: p.id,
    name: p.name,
    relationship: p.relationship,
    language: p.language,
    voiceReady: voiceReady ?? p.voiceReady,
    gender: gender ?? p.gender,
    consentBy: p.consentBy,
    listenerName: listenerName == null ? p.listenerName : listenerName(),
    sampleCount: sampleCount ?? p.sampleCount,
    totalSampleSecs: totalSampleSecs ?? p.totalSampleSecs,
  );

  @override
  SpeakerSummary setListenerName(String personId, String? listenerName) =>
      _replace(
        personId,
        (p) => _copy(p, listenerName: () => _clean(listenerName)),
      );

  @override
  Future<void> deletePerson(String personId) async {
    people.removeWhere((p) => p.id == personId);
    samples.removeWhere((s) => s.personId == personId);
    for (final MapEntry(:key, :value) in _schedules.entries.toList()) {
      if (value.request.speakerId == personId) {
        _schedules[key] = (request: value.request, enabled: false);
      }
    }
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
    final count = listSamples(personId).length;
    _replace(
      personId,
      (p) => _copy(p, sampleCount: count, totalSampleSecs: count * 6),
    );
    return listSamples(personId).last;
  }

  @override
  List<SampleSummary> listSamples(String personId) => [
    for (final (i, s) in samples.where((s) => s.personId == personId).indexed)
      SampleSummary(
        id: 'smp-$personId-${i + 1}',
        durationSecs: 6,
        meanPitchHz: 210,
        transcript: s.transcript,
      ),
  ];

  @override
  Future<VoiceSummary> buildVoice(String personId) async {
    final count = listSamples(personId).length;
    if (count < 2) {
      throw AppFailure(message: 'not enough voice material: $count sample(s)');
    }
    final p = _replace(
      personId,
      (p) => _copy(p, voiceReady: true, gender: Gender.female),
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
    final blank = text == null || text.trim().isEmpty; // like the core
    return SpokenMessage(
      text: blank ? 'Bom dia, ${listener ?? 'querido'}!' : text,
      tone: tone ?? 'calm',
      language: 'pt-BR',
      audioPath: '/outbox/${spoken.length}.wav',
    );
  }

  static void _validate(ScheduleRequest r) {
    if (_clean(r.at) == null && r.kind != ScheduleKind.interval) {
      throw const AppFailure(message: 'provide the time (HH:MM)');
    }
  }

  ScheduleSummary _summary(String id) {
    final (:request, :enabled) = _schedules[id]!;
    final person = people.where((p) => p.id == request.speakerId).firstOrNull;
    final at = request.at;
    return ScheduleSummary(
      id: id,
      speakerId: request.speakerId,
      speakerName: person?.name,
      voiceAvailable: person != null,
      enabled: enabled,
      time: (request.kind == ScheduleKind.interval || at == null)
          ? null
          : at.substring(at.length - 5),
      toneLabel: request.tone ?? 'suggested by the theme',
      nextDue: enabled ? '21/11 ${request.at ?? '09:00'}' : null,
      request: request,
    );
  }

  ({ScheduleRequest request, bool enabled}) _schedule(String id) =>
      _schedules[id] ?? (throw AppFailure(message: "schedule '$id' not found"));

  @override
  ScheduleSummary createSchedule(ScheduleRequest request) {
    _validate(request);
    scheduleRequests.add(request);
    final id = 'sch-${++_nextSchedule}';
    _schedules[id] = (request: request, enabled: true);
    return _summary(id);
  }

  @override
  ScheduleSummary updateSchedule(String id, ScheduleRequest request) {
    final current = _schedule(id);
    _validate(request);
    _schedules[id] = (request: request, enabled: current.enabled);
    return _summary(id);
  }

  @override
  ScheduleSummary setScheduleEnabled(String id, bool enabled) {
    final current = _schedule(id);
    _schedules[id] = (request: current.request, enabled: enabled);
    return _summary(id);
  }

  @override
  List<ScheduleSummary> listSchedules() => schedules;

  @override
  void removeSchedule(String id) {
    _schedule(id);
    _schedules.remove(id);
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
        consentBy: 'Vó',
        sampleCount: 2,
        totalSampleSecs: 12,
      ),
      SpeakerSummary(
        id: 'spk-2',
        name: 'Ana',
        language: 'es',
        voiceReady: false,
        consentBy: 'Ana',
        sampleCount: 0,
        totalSampleSecs: 0,
      ),
    ]);
}

/// Picker that returns the queued files, in order (`null` = cancelled).
final class FakeAudioPicker implements AudioFilePicker {
  new([List<PickedAudio?>? queue]) : queue = queue ?? [];

  final List<PickedAudio?> queue;

  @override
  Future<PickedAudio?> pick() async => queue.isEmpty ? null : queue.removeAt(0);
}

/// Records what was played.
final class FakeAudioPlayer implements AudioPlayer {
  final List<String> played = [];

  @override
  Future<void> play(String path) async => played.add(path);
}

/// Records the alarms it was asked to keep, and lets tests emit taps.
final class FakeAlarmScheduler implements AlarmScheduler {
  final List<List<Alarm>> replaced = [];
  int permissionsRequested = 0;
  final _taps = StreamController<void>.broadcast();

  void tap() => _taps.add(null);

  @override
  Stream<void> get taps => _taps.stream;

  @override
  Future<void> requestPermissions() async => permissionsRequested++;

  @override
  Future<void> replaceAll(List<Alarm> alarms) async => replaced.add(alarms);
}
