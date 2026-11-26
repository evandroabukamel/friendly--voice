import 'package:flutter_test/flutter_test.dart';
import 'package:friendly_voice/domain/voice_core.dart';
import 'package:friendly_voice/l10n/app_localizations_en.dart';
import 'package:friendly_voice/l10n/app_localizations_pt.dart';
import 'package:friendly_voice/presentation/state/labels.dart';
import 'package:friendly_voice/presentation/state/reminder_form.dart';

ScheduleRequest rule(
  ScheduleKind kind, {
  String? at,
  String? days,
  int? every,
  String? within,
  String? theme = 'good-morning',
  String? text,
  String? tone,
}) => ScheduleRequest(
  speakerId: 'spk-1',
  kind: kind,
  at: at,
  days: days,
  everyMinutes: every,
  within: within,
  theme: theme,
  text: text,
  tone: tone,
  pitchShift: 0,
);

const catalog = Catalog(
  themes: [ThemeOption(slug: 'good-morning', defaultTone: 'cheerful')],
  tones: [],
  languages: [],
  maxPitchShift: 6,
);

void main() {
  final en = AppLocalizationsEn();
  final pt = AppLocalizationsPt();
  String repeat(ScheduleRequest r) => repeatLabel(en, r);

  test('how a reminder repeats, in a few words', () {
    expect(repeat(rule(ScheduleKind.daily, at: '08:00')), 'Daily');
    expect(
      repeat(
        rule(ScheduleKind.weekly, at: '08:00', days: 'mon,tue,wed,thu,fri'),
      ),
      'Weekdays',
    );
    expect(
      repeat(rule(ScheduleKind.weekly, at: '08:00', days: 'sat,sun')),
      'Weekends',
    );
    expect(
      repeat(
        rule(
          ScheduleKind.weekly,
          at: '08:00',
          days: 'mon,tue,wed,thu,fri,sat,sun',
        ),
      ),
      'Daily',
    );
    expect(
      repeat(rule(ScheduleKind.weekly, at: '08:00', days: 'fri,mon')),
      'Mon, Fri',
    );
    expect(repeat(rule(ScheduleKind.interval, every: 1)), 'Every minute');
    expect(repeat(rule(ScheduleKind.interval, every: 30)), 'Every 30 min');
    expect(
      repeat(rule(ScheduleKind.interval, every: 90, within: '09:00-18:00')),
      'Every 90 min · 09:00–18:00',
    );
    expect(
      repeat(rule(ScheduleKind.once, at: '2026-12-24 09:00')),
      'Once · 24/12',
    );
  });

  test('labels follow the app language', () {
    expect(
      repeatLabel(pt, rule(ScheduleKind.daily, at: '08:00')),
      pt.repeatLabelDaily,
    );
    expect(
      repeatLabel(pt, rule(ScheduleKind.weekly, at: '08:00', days: 'mon')),
      pt.dayMonAbbr,
    );
    expect(formErrorText(pt, FormError.noDays), pt.formErrorNoDays);
    expect(weekdayName(pt, Weekday.sun), pt.daySun);
  });

  test('what a reminder says: theme or own text, with its tone', () {
    expect(
      messageLabel(en, rule(ScheduleKind.daily, at: '08:00'), catalog),
      'Good morning · Suggested by the theme (Cheerful)',
    );
    expect(
      messageLabel(
        en,
        rule(ScheduleKind.daily, at: '08:00', tone: 'warm'),
        catalog,
      ),
      'Good morning · Warm',
    );
    expect(
      messageLabel(
        en,
        rule(ScheduleKind.daily, at: '08:00', theme: null, text: 'Oi'),
        catalog,
      ),
      'Your own text · Suggested by the theme',
    );
  });

  test('slugs read as words', () {
    expect(humanize('good-morning'), 'Good morning');
    expect(humanize('self_care'), 'Self care');
    expect(humanize(''), '');
  });

  test('a voice line counts recordings and seconds', () {
    expect(recordingsLabel(en, 0, 0), '0 recordings · 0 seconds');
    expect(recordingsLabel(en, 1, 1.04), '1 recording · 1 second');
    expect(recordingsLabel(en, 3, 42.4), '3 recordings · 42 seconds');
  });
}
