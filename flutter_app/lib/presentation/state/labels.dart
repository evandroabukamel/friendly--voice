import 'package:friendly_voice/domain/voice_core.dart';
import 'package:friendly_voice/l10n/app_localizations.dart';
import 'package:friendly_voice/presentation/state/reminder_form.dart';

/// "good-morning" → "Good morning" (themes and tones come from the core as
/// slugs, as in the Kotlin app).
String humanize(String slug) {
  final words = slug.replaceAll(RegExp('[-_]'), ' ');
  return words.isEmpty ? words : words[0].toUpperCase() + words.substring(1);
}

/// Short label for how a reminder repeats ("Weekdays", "Mon, Fri"…).
String repeatLabel(AppLocalizations l, ScheduleRequest r) => switch (r.kind) {
  ScheduleKind.daily => l.repeatLabelDaily,
  ScheduleKind.weekly => _weekly(l, Weekday.parseList(r.days)),
  ScheduleKind.interval => switch (TimeRange.parse(r.within)) {
    final w? => l.repeatLabelWithin(
      l.repeatLabelEvery(r.everyMinutes ?? 0),
      '${w.from}',
      '${w.to}',
    ),
    null => l.repeatLabelEvery(r.everyMinutes ?? 0),
  },
  ScheduleKind.once => l.repeatLabelOnce(dayMonth(l, r.at ?? '')),
};

String _weekly(AppLocalizations l, List<Weekday> days) {
  final set = days.toSet();
  if (set.length == Weekday.values.length) return l.repeatLabelDaily;
  if (_same(set, Weekday.workdays)) return l.repeatLabelWeekdays;
  if (_same(set, Weekday.weekend)) return l.repeatLabelWeekends;
  return days.map((d) => weekdayAbbr(l, d)).join(', ');
}

bool _same(Set<Weekday> a, Set<Weekday> b) =>
    a.length == b.length && a.containsAll(b);

/// "2026-12-24 09:00" → "24/12" in the language's day-month order.
String dayMonth(AppLocalizations l, String at) {
  final parts = at.split(' ').first.split('-');
  return parts.length == 3 ? l.repeatDateValue(parts[2], parts[1]) : at;
}

/// What the reminder says and how: theme (or own text) · tone.
String messageLabel(AppLocalizations l, ScheduleRequest r, Catalog? catalog) {
  final own = r.text?.trim().isNotEmpty ?? false;
  final theme = own ? l.reminderOwnText : humanize(r.theme ?? '');
  return l.reminderThemeLine(theme, switch (r.tone) {
    final tone? => humanize(tone),
    null => suggestedTone(l, own ? null : r.theme, catalog),
  });
}

/// "Suggested by the theme (Cheerful)" or, without a theme, the plain one.
String suggestedTone(AppLocalizations l, String? theme, Catalog? catalog) {
  for (final t in catalog?.themes ?? const <ThemeOption>[]) {
    if (t.slug == theme) return l.toneSuggestedWith(humanize(t.defaultTone));
  }
  return l.toneSuggested;
}

/// "3 recordings · 42 seconds".
String recordingsLabel(AppLocalizations l, int count, double secs) {
  final whole = secs.round();
  return l.samplesCountAndSecs(l.samplesCount(count), l.seconds(whole));
}

/// Words for each reason the reminder form cannot be saved yet.
String formErrorText(AppLocalizations l, FormError e) => switch (e) {
  FormError.noVoice => l.formErrorNoVoice,
  FormError.noTheme => l.formErrorNoTheme,
  FormError.noDays => l.formErrorNoDays,
  FormError.badInterval => l.formErrorInterval,
  FormError.noDate => l.formErrorNoDate,
};

String weekdayInitial(AppLocalizations l, Weekday d) => switch (d) {
  Weekday.mon => l.dayMonShort,
  Weekday.tue => l.dayTueShort,
  Weekday.wed => l.dayWedShort,
  Weekday.thu => l.dayThuShort,
  Weekday.fri => l.dayFriShort,
  Weekday.sat => l.daySatShort,
  Weekday.sun => l.daySunShort,
};

String weekdayAbbr(AppLocalizations l, Weekday d) => switch (d) {
  Weekday.mon => l.dayMonAbbr,
  Weekday.tue => l.dayTueAbbr,
  Weekday.wed => l.dayWedAbbr,
  Weekday.thu => l.dayThuAbbr,
  Weekday.fri => l.dayFriAbbr,
  Weekday.sat => l.daySatAbbr,
  Weekday.sun => l.daySunAbbr,
};

String weekdayName(AppLocalizations l, Weekday d) => switch (d) {
  Weekday.mon => l.dayMon,
  Weekday.tue => l.dayTue,
  Weekday.wed => l.dayWed,
  Weekday.thu => l.dayThu,
  Weekday.fri => l.dayFri,
  Weekday.sat => l.daySat,
  Weekday.sun => l.daySun,
};
