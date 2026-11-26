import 'package:flutter_test/flutter_test.dart';
import 'package:friendly_voice/domain/voice_core.dart';
import 'package:friendly_voice/presentation/state/labels.dart';

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

void main() {
  test('how a reminder repeats, in a few words', () {
    expect(repeatLabel(rule(ScheduleKind.daily, at: '08:00')), 'Daily');
    expect(
      repeatLabel(
        rule(ScheduleKind.weekly, at: '08:00', days: 'mon,tue,wed,thu,fri'),
      ),
      'Weekdays',
    );
    expect(
      repeatLabel(rule(ScheduleKind.weekly, at: '08:00', days: 'sat,sun')),
      'Weekends',
    );
    expect(
      repeatLabel(
        rule(
          ScheduleKind.weekly,
          at: '08:00',
          days: 'mon,tue,wed,thu,fri,sat,sun',
        ),
      ),
      'Daily',
    );
    expect(
      repeatLabel(rule(ScheduleKind.weekly, at: '08:00', days: 'fri,mon')),
      'Mon, Fri',
    );
    expect(repeatLabel(rule(ScheduleKind.interval, every: 30)), 'Every 30 min');
    expect(
      repeatLabel(
        rule(ScheduleKind.interval, every: 90, within: '09:00-18:00'),
      ),
      'Every 90 min · 09:00–18:00',
    );
    expect(
      repeatLabel(rule(ScheduleKind.once, at: '2026-12-24 09:00')),
      'Once · 24/12',
    );
  });

  test('what a reminder says: theme or own text, with its tone', () {
    expect(messageLabel(rule(ScheduleKind.daily, at: '08:00')), 'Good morning');
    expect(
      messageLabel(rule(ScheduleKind.daily, at: '08:00', tone: 'warm')),
      'Good morning · Warm',
    );
    expect(
      messageLabel(
        rule(ScheduleKind.daily, at: '08:00', theme: null, text: 'Oi'),
      ),
      'Your own text',
    );
  });

  test('slugs read as words', () {
    expect(humanize('good-morning'), 'Good morning');
    expect(humanize('self_care'), 'Self care');
    expect(humanize(''), '');
  });

  test('a voice line counts recordings and seconds', () {
    expect(recordingsLabel(0, 0), 'No recordings yet');
    expect(recordingsLabel(1, 6.04), '1 recording · 6 s');
    expect(recordingsLabel(3, 42.4), '3 recordings · 42 s');
  });
}
