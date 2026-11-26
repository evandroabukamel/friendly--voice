import 'package:flutter_test/flutter_test.dart';
import 'package:friendly_voice/domain/app_settings.dart';

void main() {
  test('a blank name means "not set"', () {
    expect(AppSettings(listener: '  ').listener, isNull);
  });

  test('the name is trimmed', () {
    expect(AppSettings(listener: ' Evandro ').listener, 'Evandro');
  });

  test('the voice server comes from the build; blank means none', () {
    expect(engineUrlFromBuild(''), isNull);
    expect(engineUrlFromBuild('  '), isNull);
    expect(engineUrlFromBuild(' http://pc:8020 '), 'http://pc:8020');
  });
}
