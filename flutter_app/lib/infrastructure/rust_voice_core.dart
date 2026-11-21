import 'dart:io';
import 'dart:typed_data';

import 'package:friendly_voice/domain/voice_core.dart';
import 'package:friendly_voice/src/rust/api/catalog.dart' as rust_catalog;
import 'package:friendly_voice/src/rust/api/session.dart';

/// [VoiceCore] backed by the Rust core through flutter_rust_bridge.
final class RustVoiceCore implements VoiceCore {
  new(this._session);

  final Session _session;

  @override
  String get engineName => _session.engineName;

  @override
  Catalog catalog() => rust_catalog.catalog();

  @override
  List<SpeakerSummary> listPeople() => _session.listSpeakers();

  @override
  SpeakerSummary registerPerson({
    required String name,
    String? relationship,
    String? consentBy,
    String? language,
  }) => _session.registerSpeaker(
    name: name,
    relationship: relationship,
    consentBy: consentBy,
    language: language,
  );

  @override
  Future<SampleSummary> addSample({
    required String personId,
    required Uint8List audio,
    String? transcript,
  }) => _session.addSample(
    speakerId: personId,
    audio: audio,
    transcript: transcript,
  );

  @override
  Future<VoiceSummary> buildVoice(String personId) =>
      _session.buildVoice(speakerId: personId);

  @override
  Future<SpokenMessage> speak({
    required String personId,
    String? theme,
    String? text,
    String? tone,
    int pitchShift = 0,
    String? listener,
  }) => _session.speak(
    speakerId: personId,
    theme: theme,
    text: text,
    toneStyle: tone,
    pitchShift: pitchShift.toDouble(),
    listener: listener,
  );

  @override
  ScheduleSummary createSchedule(ScheduleRequest request) =>
      _session.createSchedule(request: request);

  @override
  List<ScheduleSummary> listSchedules() => _session.listSchedules();

  @override
  void removeSchedule(String id) => _session.removeSchedule(id: id);

  @override
  Future<List<DueMessage>> runDueSchedules({String? listener}) =>
      _session.runDueSchedules(listener: listener);

  @override
  List<Alarm> upcomingAlarms(int limit) =>
      _session.upcomingAlarms(limit: limit);
}

/// Opens the Rust core on the folder returned by [dataDir] (in the app,
/// the platform's application support directory).
final class RustVoiceCoreFactory implements VoiceCoreFactory {
  new({required this.dataDir});

  final Future<Directory> Function() dataDir;

  @override
  Future<VoiceCore> open({String? engineUrl}) async {
    final dir = await dataDir();
    return RustVoiceCore(Session.open(dataDir: dir.path, engineUrl: engineUrl));
  }
}
