import 'package:flutter_test/flutter_test.dart';
import 'package:friendly_voice/domain/voice_core.dart';
import 'package:friendly_voice/presentation/state/reminder_form.dart';

ScheduleRequest ok(FormResult r) => switch (r) {
  FormOk(:final request) => request,
  FormInvalid(:final error) => fail('invalid: $error'),
};

FormError? error(FormResult r) => switch (r) {
  FormOk() => null,
  FormInvalid(:final error) => error,
};

void main() {
  const base = ReminderForm(speakerId: 'spk-1', theme: 'good-morning');

  group('clock time', () {
    test('parses and prints HH:MM', () {
      expect(ClockTime.parse('7:05').toString(), '07:05');
      expect(ClockTime.parse('23:59'), const ClockTime(23, 59));
    });

    test('falls back to 08:00 on nonsense', () {
      for (final bad in ['', 'x', '25:00', '10:61']) {
        expect(ClockTime.parse(bad), const ClockTime(8, 0), reason: bad);
      }
    });
  });

  group('weekdays', () {
    test('parse a list in calendar order, skipping unknown tokens', () {
      expect(Weekday.parseList('fri, Monday,xyz,mon'), [
        Weekday.mon,
        Weekday.fri,
      ]);
      expect(Weekday.parseList(null), isEmpty);
    });
  });

  group('form → request', () {
    test('a daily reminder at the chosen time', () {
      final r = ok(base.copyWith(time: const ClockTime(7, 30)).toRequest());
      expect(
        (r.kind, r.at, r.theme, r.text, r.quiet),
        (ScheduleKind.daily, '07:30', 'good-morning', null, null),
      );
    });

    test('chosen days are sorted and joined', () {
      final r = ok(
        base
            .copyWith(
              kind: ScheduleKind.weekly,
              days: {Weekday.fri, Weekday.mon},
            )
            .toRequest(),
      );
      expect((r.kind, r.days, r.at), (ScheduleKind.weekly, 'mon,fri', '08:00'));
    });

    test('an interval inside its window', () {
      final r = ok(
        base
            .copyWith(kind: ScheduleKind.interval, everyMinutes: 90)
            .toRequest(),
      );
      expect((r.everyMinutes, r.within, r.at), (90, '09:00-18:00', null));
    });

    test('once on a date at a time', () {
      final r = ok(
        base
            .copyWith(
              kind: ScheduleKind.once,
              date: DateTime(2026, 12, 24),
              time: const ClockTime(9, 0),
            )
            .toRequest(),
      );
      expect(r.at, '2026-12-24 09:00');
    });

    test('quiet hours only when switched on; own text wins over theme', () {
      final r = ok(
        base.copyWith(quietEnabled: true, text: () => 'Respira').toRequest(),
      );
      expect((r.quiet, r.text, r.theme), ('22:00-07:00', 'Respira', null));
    });

    test('tone and pitch go along', () {
      final r = ok(base.copyWith(tone: () => 'warm', pitch: -2).toRequest());
      expect((r.tone, r.pitchShift), ('warm', -2.0));
    });

    test('missing pieces are named, not guessed', () {
      expect(
        error(base.copyWith(speakerId: () => null).toRequest()),
        FormError.noVoice,
      );
      expect(
        error(base.copyWith(theme: () => null, text: () => ' ').toRequest()),
        FormError.noTheme,
      );
      expect(
        error(base.copyWith(kind: ScheduleKind.weekly, days: {}).toRequest()),
        FormError.noDays,
      );
      expect(
        error(
          base
              .copyWith(kind: ScheduleKind.interval, everyMinutes: 0)
              .toRequest(),
        ),
        FormError.badInterval,
      );
      expect(
        error(base.copyWith(kind: ScheduleKind.once).toRequest()),
        FormError.noDate,
      );
    });
  });

  group('request → form', () {
    test('every kind comes back as it was sent', () {
      final forms = [
        base.copyWith(time: const ClockTime(6, 45), quietEnabled: true),
        base.copyWith(
          kind: ScheduleKind.weekly,
          days: {Weekday.sat, Weekday.sun},
        ),
        base.copyWith(kind: ScheduleKind.interval, everyMinutes: 45),
        base.copyWith(kind: ScheduleKind.once, date: DateTime(2027, 1, 2)),
        base.copyWith(theme: () => null, text: () => 'Oi', tone: () => 'calm'),
      ];
      for (final f in forms) {
        final request = ok(f.toRequest());
        expect(ok(ReminderForm.from(request).toRequest()), request);
      }
    });
  });

  group('selectable voices', () {
    test('a new reminder starts with the first ready voice', () {
      const empty = ReminderForm();
      expect(
        empty.withSelectableVoice(['a', 'b'], editing: false).speakerId,
        'a',
      );
    });

    test('an edited reminder whose voice is gone must choose again', () {
      final gone = base.copyWith(speakerId: () => 'old');
      expect(gone.withSelectableVoice(['a'], editing: true).speakerId, isNull);
      expect(base.withSelectableVoice(['spk-1'], editing: true), base);
    });
  });
}
