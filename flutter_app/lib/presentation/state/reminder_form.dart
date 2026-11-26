import 'package:friendly_voice/domain/voice_core.dart';
import 'package:material_ui/material_ui.dart' show immutable;

/// A time of day on the clock (no date, no zone).
@immutable
final class ClockTime {
  const new(this.hour, this.minute)
    : assert(hour >= 0 && hour < 24, 'hour 0-23'),
      assert(minute >= 0 && minute < 60, 'minute 0-59');

  /// "HH:MM"; anything unreadable becomes 08:00.
  factory parse(String text) {
    final parts = text.trim().split(':');
    final h = parts.isEmpty ? null : int.tryParse(parts.first);
    final m = parts.length < 2 ? null : int.tryParse(parts[1]);
    if (h == null || m == null || h < 0 || h > 23 || m < 0 || m > 59) {
      return const ClockTime(8, 0);
    }
    return ClockTime(h, m);
  }

  final int hour;
  final int minute;

  @override
  String toString() =>
      '${hour.toString().padLeft(2, '0')}:${minute.toString().padLeft(2, '0')}';

  @override
  bool operator ==(Object other) =>
      other is ClockTime && other.hour == hour && other.minute == minute;

  @override
  int get hashCode => Object.hash(hour, minute);
}

enum Weekday {
  mon,
  tue,
  wed,
  thu,
  fri,
  sat,
  sun;

  static const Set<Weekday> workdays = {mon, tue, wed, thu, fri};
  static const Set<Weekday> weekend = {sat, sun};

  /// "mon,wed,fri" (also "Monday", any case) → days in calendar order;
  /// unknown tokens are skipped.
  static List<Weekday> parseList(String? text) {
    final found = <Weekday>{};
    for (final token in (text ?? '').split(',')) {
      final key = token.trim().toLowerCase();
      if (key.length < 3) {
        continue;
      }
      for (final d in values) {
        if (d.name == key.substring(0, 3)) found.add(d);
      }
    }
    return sorted(found);
  }

  static List<Weekday> sorted(Iterable<Weekday> days) =>
      days.toSet().toList()..sort((a, b) => a.index.compareTo(b.index));
}

/// A window between two clock times ("09:00-18:00"; may cross midnight).
@immutable
final class TimeRange {
  const new(this.from, this.to);

  final ClockTime from;
  final ClockTime to;

  static TimeRange? parse(String? text) {
    final parts = text?.split('-');
    if (parts == null || parts.length != 2) return null;
    return TimeRange(ClockTime.parse(parts[0]), ClockTime.parse(parts[1]));
  }

  @override
  String toString() => '$from-$to';

  @override
  bool operator ==(Object other) =>
      other is TimeRange && other.from == from && other.to == to;

  @override
  int get hashCode => Object.hash(from, to);
}

/// Why the form does not make a reminder yet. The UI words each one.
enum FormError { noVoice, noTheme, noDays, badInterval, noDate }

sealed class FormResult {
  const new();
}

final class FormOk extends FormResult {
  const new(this.request);
  final ScheduleRequest request;
}

final class FormInvalid extends FormResult {
  const new(this.error);
  final FormError error;
}

/// Every choice on the reminder screen. Pure: turned into the core's
/// request by [toRequest] and rebuilt from a saved one by [ReminderForm.from].
@immutable
final class ReminderForm {
  const new({
    this.speakerId,
    this.theme,
    this.text,
    this.tone,
    this.pitch = 0,
    this.kind = ScheduleKind.daily,
    this.time = const ClockTime(8, 0),
    this.days = Weekday.workdays,
    this.everyMinutes = 60,
    this.window = const TimeRange(ClockTime(9, 0), ClockTime(18, 0)),
    this.date,
    this.quietEnabled = false,
    this.quiet = const TimeRange(ClockTime(22, 0), ClockTime(7, 0)),
  });

  /// The form of a saved reminder (tap on a reminder card).
  factory from(ScheduleRequest r) {
    final quiet = TimeRange.parse(r.quiet);
    final base = ReminderForm(
      speakerId: r.speakerId,
      theme: r.theme,
      text: r.text,
      tone: r.tone,
      pitch: r.pitchShift.round(),
      kind: r.kind,
      quietEnabled: quiet != null,
      quiet: quiet ?? const ReminderForm().quiet,
    );
    final at = r.at ?? '';
    return switch (r.kind) {
      ScheduleKind.daily => base.copyWith(time: ClockTime.parse(at)),
      ScheduleKind.weekly => base.copyWith(
        time: ClockTime.parse(at),
        days: Weekday.parseList(r.days).toSet(),
      ),
      ScheduleKind.interval => base.copyWith(
        everyMinutes: r.everyMinutes,
        window: TimeRange.parse(r.within) ?? base.window,
      ),
      ScheduleKind.once => base.copyWith(
        date: DateTime.tryParse(at.split(' ').first),
        time: ClockTime.parse(at.split(' ').last),
      ),
    };
  }

  final String? speakerId;
  final String? theme;

  /// Own words; when not blank they replace the theme.
  final String? text;

  /// `null` = the tone the theme suggests.
  final String? tone;
  final int pitch;
  final ScheduleKind kind;
  final ClockTime time;
  final Set<Weekday> days;
  final int? everyMinutes;
  final TimeRange window;
  final DateTime? date;
  final bool quietEnabled;
  final TimeRange quiet;

  bool get hasOwnText => (text ?? '').trim().isNotEmpty;

  FormResult toRequest() {
    final speaker = speakerId;
    if (speaker == null) return const FormInvalid(FormError.noVoice);
    if (!hasOwnText && theme == null) {
      return const FormInvalid(FormError.noTheme);
    }
    String? at;
    String? weekdays;
    int? every;
    String? within;
    switch (kind) {
      case ScheduleKind.daily:
        at = '$time';
      case ScheduleKind.weekly:
        if (days.isEmpty) return const FormInvalid(FormError.noDays);
        at = '$time';
        weekdays = Weekday.sorted(days).map((d) => d.name).join(',');
      case ScheduleKind.interval:
        final minutes = everyMinutes;
        if (minutes == null || minutes < 1) {
          return const FormInvalid(FormError.badInterval);
        }
        every = minutes;
        within = '$window';
      case ScheduleKind.once:
        final d = date;
        if (d == null) return const FormInvalid(FormError.noDate);
        at = '${_isoDate(d)} $time';
    }
    return FormOk(
      ScheduleRequest(
        speakerId: speaker,
        kind: kind,
        at: at,
        days: weekdays,
        everyMinutes: every,
        within: within,
        quiet: quietEnabled ? '$quiet' : null,
        theme: hasOwnText ? null : theme,
        text: hasOwnText ? text!.trim() : null,
        tone: tone,
        pitchShift: pitch.toDouble(),
      ),
    );
  }

  /// Matches the voice to the ready ones: a new reminder starts with the
  /// first; an edited one whose voice is gone must choose another.
  ReminderForm withSelectableVoice(
    List<String> voiceIds, {
    required bool editing,
  }) {
    if (speakerId != null && voiceIds.contains(speakerId)) return this;
    return copyWith(speakerId: () => editing ? null : voiceIds.firstOrNull);
  }

  /// Nullable fields take a function so they can be cleared.
  ReminderForm copyWith({
    String? Function()? speakerId,
    String? Function()? theme,
    String? Function()? text,
    String? Function()? tone,
    int? pitch,
    ScheduleKind? kind,
    ClockTime? time,
    Set<Weekday>? days,
    int? everyMinutes,
    TimeRange? window,
    DateTime? date,
    bool? quietEnabled,
    TimeRange? quiet,
  }) => ReminderForm(
    speakerId: speakerId == null ? this.speakerId : speakerId(),
    theme: theme == null ? this.theme : theme(),
    text: text == null ? this.text : text(),
    tone: tone == null ? this.tone : tone(),
    pitch: pitch ?? this.pitch,
    kind: kind ?? this.kind,
    time: time ?? this.time,
    days: days ?? this.days,
    everyMinutes: everyMinutes ?? this.everyMinutes,
    window: window ?? this.window,
    date: date ?? this.date,
    quietEnabled: quietEnabled ?? this.quietEnabled,
    quiet: quiet ?? this.quiet,
  );

  @override
  bool operator ==(Object other) =>
      other is ReminderForm &&
      other.speakerId == speakerId &&
      other.theme == theme &&
      other.text == text &&
      other.tone == tone &&
      other.pitch == pitch &&
      other.kind == kind &&
      other.time == time &&
      other.days.length == days.length &&
      other.days.containsAll(days) &&
      other.everyMinutes == everyMinutes &&
      other.window == window &&
      other.date == date &&
      other.quietEnabled == quietEnabled &&
      other.quiet == quiet;

  @override
  int get hashCode => Object.hash(
    speakerId,
    theme,
    text,
    tone,
    pitch,
    kind,
    time,
    Object.hashAllUnordered(days),
    everyMinutes,
    window,
    date,
    quietEnabled,
    quiet,
  );
}

String _isoDate(DateTime d) =>
    '${d.year.toString().padLeft(4, '0')}-'
    '${d.month.toString().padLeft(2, '0')}-'
    '${d.day.toString().padLeft(2, '0')}';
