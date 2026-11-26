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

  /// [listenerName]: how this person calls the listener (optional).
  SpeakerSummary registerPerson({
    required String name,
    String? relationship,
    String? consentBy,
    String? language,
    String? listenerName,
  });

  /// Sets how the person calls the listener; blank clears it.
  SpeakerSummary setListenerName(String personId, String? listenerName);

  /// Deletes the person and everything made from their voice (also on the
  /// voice server); their reminders are switched off and kept. Idempotent.
  Future<void> deletePerson(String personId);

  Future<SampleSummary> addSample({
    required String personId,
    required Uint8List audio,
    String? transcript,
  });

  List<SampleSummary> listSamples(String personId);

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

  /// Replaces the rule, keeping the id and the on/off state. Sending the
  /// same request again changes nothing.
  ScheduleSummary updateSchedule(String id, ScheduleRequest request);

  /// The on/off switch; switching on only fires future occurrences.
  ScheduleSummary setScheduleEnabled(String id, bool enabled);

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
