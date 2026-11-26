import 'dart:convert';
import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:friendly_voice/l10n/app_localizations.dart';
import 'package:friendly_voice/presentation/state/app_language.dart';
import 'package:material_ui/material_ui.dart';

/// Keys of features still being ported; each later unit empties its part.
const pending = <String>{
  // The core's audio failures, translated once the bridge reports their kind.
  'errorNoAudioTrack', 'errorAudioFormat', 'errorAudioTooLong',
};

Map<String, Object?> arb(String locale) =>
    (jsonDecode(File('lib/l10n/app_$locale.arb').readAsStringSync()) as Map)
        .cast<String, Object?>();

Set<String> messages(Map<String, Object?> file) => {
  for (final k in file.keys)
    if (!k.startsWith('@')) k,
};

void main() {
  final template = messages(arb('en'));
  final locales = [
    for (final f in Directory('lib/l10n').listSync())
      if (RegExp(r'app_(\w+)\.arb$').firstMatch(f.path) case final m?)
        m.group(1)!,
  ];

  test('every language has every text; regional files only override', () {
    for (final locale in locales) {
      final keys = messages(arb(locale));
      final regional = locale.contains('_') && locale != 'zh_TW';
      if (regional) {
        expect(keys.difference(template), isEmpty, reason: locale);
      } else {
        expect(template.difference(keys), isEmpty, reason: locale);
      }
    }
  });

  test(
    'every text is used by the app (an unused one is a missing feature)',
    () {
      final code = [
        for (final f in Directory('lib').listSync(recursive: true))
          if (f is File && f.path.endsWith('.dart') && !f.path.contains('l10n'))
            f.readAsStringSync(),
      ].join('\n');
      final unused = {
        for (final key in template)
          if (!RegExp('\\.$key\\b').hasMatch(code)) key,
      }.difference(pending);
      expect(unused, isEmpty);
    },
  );

  test("the app offers the Android app's 21 languages, each translated", () {
    expect(appLanguages, hasLength(21));
    for (final (:tag, :name) in appLanguages) {
      final locale = localeOf(tag);
      expect(
        AppLocalizations.delegate.isSupported(locale),
        isTrue,
        reason: tag,
      );
      expect(name, isNotEmpty);
    }
    expect(
      {for (final l in appLanguages) l.tag},
      containsAll([
        'pt-BR',
        'pt-PT',
        'en-GB',
        'en-US',
        'en-NZ',
        'en-AU',
        'es-ES',
        'es-MX',
        'es-AR',
        'de',
        'fr',
        'it',
        'zh-CN',
        'zh-TW',
        'ja',
        'ru',
        'uk',
        'tr',
        'ar',
        'fi',
        'nb',
      ]),
    );
  });

  test('regional variants keep their own wording', () async {
    final br = await AppLocalizations.delegate.load(localeOf('pt-BR'));
    final pt = await AppLocalizations.delegate.load(localeOf('pt-PT'));
    final tw = await AppLocalizations.delegate.load(localeOf('zh-TW'));
    final cn = await AppLocalizations.delegate.load(localeOf('zh-CN'));
    expect(pt.settingsTitle, isNot(br.settingsTitle));
    expect(tw.settingsTitle, isNot(cn.settingsTitle));
    expect(localeOf('pt-BR'), const Locale('pt', 'BR'));
  });
}
