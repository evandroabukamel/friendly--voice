import 'package:material_ui/material_ui.dart';

/// The languages the app speaks, as in the Kotlin app's Settings picker:
/// a BCP-47 tag and the language written in itself, so anyone finds theirs.
const List<({String tag, String name})> appLanguages = [
  (tag: 'pt-BR', name: 'Português (Brasil)'),
  (tag: 'pt-PT', name: 'Português (Portugal)'),
  (tag: 'en-GB', name: 'English (UK)'),
  (tag: 'en-US', name: 'English (US)'),
  (tag: 'en-NZ', name: 'English (New Zealand)'),
  (tag: 'en-AU', name: 'English (Australia)'),
  (tag: 'es-ES', name: 'Español (España)'),
  (tag: 'es-MX', name: 'Español (México)'),
  (tag: 'es-AR', name: 'Español (Argentina)'),
  (tag: 'de', name: 'Deutsch'),
  (tag: 'fr', name: 'Français'),
  (tag: 'it', name: 'Italiano'),
  (tag: 'zh-CN', name: '简体中文'),
  (tag: 'zh-TW', name: '繁體中文（台灣）'),
  (tag: 'ja', name: '日本語'),
  (tag: 'ru', name: 'Русский'),
  (tag: 'uk', name: 'Українська'),
  (tag: 'tr', name: 'Türkçe'),
  (tag: 'ar', name: 'العربية'),
  (tag: 'fi', name: 'Suomi'),
  (tag: 'nb', name: 'Norsk bokmål'),
];

/// "pt-BR" → `Locale('pt', 'BR')`; parsed once, where the stored tag enters.
Locale localeOf(String tag) {
  final [language, ...rest] = tag.split(RegExp('[-_]'));
  return Locale(language, rest.isEmpty ? null : rest.last);
}
