import 'package:friendly_voice/domain/voice_core.dart';
import 'package:friendly_voice/presentation/state/reminder_form.dart';

/// "good-morning" → "Good morning".
String humanize(String slug) {
  final words = slug.replaceAll(RegExp('[-_]'), ' ');
  return words.isEmpty ? words : words[0].toUpperCase() + words.substring(1);
}

const Map<Weekday, String> _dayNames = {
  Weekday.mon: 'Mon',
  Weekday.tue: 'Tue',
  Weekday.wed: 'Wed',
  Weekday.thu: 'Thu',
  Weekday.fri: 'Fri',
  Weekday.sat: 'Sat',
  Weekday.sun: 'Sun',
};

/// Short label for how a reminder repeats ("Weekdays", "Mon, Fri"…).
String repeatLabel(ScheduleRequest r) => switch (r.kind) {
  ScheduleKind.daily => 'Daily',
  ScheduleKind.weekly => _weekly(Weekday.parseList(r.days)),
  ScheduleKind.interval => [
    'Every ${r.everyMinutes ?? '?'} min',
    if (TimeRange.parse(r.within) case final w?) '${w.from}–${w.to}',
  ].join(' · '),
  ScheduleKind.once => 'Once · ${_dayMonth(r.at ?? '')}',
};

String _weekly(List<Weekday> days) {
  final set = days.toSet();
  if (set.length == Weekday.values.length) return 'Daily';
  if (_same(set, Weekday.workdays)) return 'Weekdays';
  if (_same(set, Weekday.weekend)) return 'Weekends';
  return days.map((d) => _dayNames[d]).join(', ');
}

bool _same(Set<Weekday> a, Set<Weekday> b) =>
    a.length == b.length && a.containsAll(b);

/// "2026-12-24 09:00" → "24/12" (the style of the core's next firing).
String _dayMonth(String at) {
  final parts = at.split(' ').first.split('-');
  return parts.length == 3 ? '${parts[2]}/${parts[1]}' : at;
}

/// What the reminder says: its theme (or "Your own text") and tone.
String messageLabel(ScheduleRequest r) => [
  if (r.text case final t? when t.trim().isNotEmpty)
    'Your own text'
  else
    humanize(r.theme ?? ''),
  if (r.tone case final tone?) humanize(tone),
].join(' · ');

/// "3 recordings · 42 s".
String recordingsLabel(int count, double secs) => count == 0
    ? 'No recordings yet'
    : '$count recording${count == 1 ? '' : 's'} · ${secs.round()} s';

/// Words for each reason the reminder form cannot be saved yet.
String formErrorText(FormError e) => switch (e) {
  FormError.noVoice => 'Choose a voice first.',
  FormError.noTheme => 'Choose a theme.',
  FormError.noDays => 'Choose at least one day.',
  FormError.badInterval =>
    'Enter how many minutes between messages (1 or more).',
  FormError.noDate => 'Pick the date.',
};

const Map<Weekday, String> weekdayInitials = {
  Weekday.mon: 'M',
  Weekday.tue: 'T',
  Weekday.wed: 'W',
  Weekday.thu: 'T',
  Weekday.fri: 'F',
  Weekday.sat: 'S',
  Weekday.sun: 'S',
};

const Map<Weekday, String> weekdayNames = {
  Weekday.mon: 'Monday',
  Weekday.tue: 'Tuesday',
  Weekday.wed: 'Wednesday',
  Weekday.thu: 'Thursday',
  Weekday.fri: 'Friday',
  Weekday.sat: 'Saturday',
  Weekday.sun: 'Sunday',
};
