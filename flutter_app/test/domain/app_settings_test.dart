import 'package:flutter_test/flutter_test.dart';
import 'package:friendly_voice/domain/app_settings.dart';

void main() {
  test('blank values mean "not set"', () {
    final s = AppSettings(listener: '  ', engineUrl: '');
    expect(s.listener, isNull);
    expect(s.engineUrl, isNull);
  });

  test('values are trimmed', () {
    final s = AppSettings(listener: ' Evandro ', engineUrl: ' http://pc:8020 ');
    expect(s.listener, 'Evandro');
    expect(s.engineUrl, 'http://pc:8020');
  });
}
