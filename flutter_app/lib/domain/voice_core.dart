import 'dart:typed_data';

import 'package:friendly_voice/src/rust/api/catalog.dart';
import 'package:friendly_voice/src/rust/api/messages.dart';
import 'package:friendly_voice/src/rust/api/schedules.dart';
import 'package:friendly_voice/src/rust/api/speakers.dart';
import 'package:friendly_voice/src/rust/api/voice.dart';

export 'package:friendly_voice/src/rust/api/catalog.dart';
export 'package:friendly_voice/src/rust/api/failure.dart';
export 'package:friendly_voice/src/rust/api/messages.dart';
export 'package:friendly_voice/src/rust/api/schedules.dart';
export 'package:friendly_voice/src/rust/api/speakers.dart';
export 'package:friendly_voice/src/rust/api/voice.dart';

/// Port to the business rules (the Rust core). The data types are the ones
/// generated from the Rust API: the core is their single source of truth.
/// Every operation throws `AppFailure` with an actionable message.
abstract interface class VoiceCore {
  String get engineName;

  Catalog catalog();

  List<SpeakerSummary> listPeople();

  SpeakerSummary registerPerson({
    required String name,
    String? relationship,
    String? consentBy,
    String? language,
  });

  Future<SampleSummary> addSample({
    required String personId,
    required Uint8List audio,
    String? transcript,
  });

  Future<VoiceSummary> buildVoice(String personId);

  Future<SpokenMessage> speak({
    required String personId,
    String? theme,
    String? text,
    String? tone,
    int pitchShift = 0,
    String? listener,
  });

  ScheduleSummary createSchedule(ScheduleRequest request);

  List<ScheduleSummary> listSchedules();

  void removeSchedule(String id);

  /// Speaks every schedule that is due, exactly once.
  Future<List<DueMessage>> runDueSchedules({String? listener});

  /// The next firings of all schedules, soonest first.
  List<Alarm> upcomingAlarms(int limit);
}

/// Opens the core on the app's data folder, with the neural server at
/// `engineUrl` or the local sketch engine when it is null.
abstract interface class VoiceCoreFactory {
  Future<VoiceCore> open({String? engineUrl});
}
