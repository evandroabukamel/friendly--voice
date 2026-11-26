// ignore: unused_import
import 'package:intl/intl.dart' as intl;

import 'app_localizations.dart';

// ignore_for_file: type=lint

/// The translations for German (`de`).
class AppLocalizationsDe extends AppLocalizations {
  AppLocalizationsDe([String locale = 'de']) : super(locale);

  @override
  String get appName => 'Friendly Voice';

  @override
  String get actionBack => 'Zurück';

  @override
  String get errorGeneric =>
      'Etwas ist schiefgelaufen. Bitte versuch es noch einmal.';

  @override
  String get errorPlayback => 'Die Nachricht konnte nicht abgespielt werden.';

  @override
  String get errorFileOpen => 'Die Datei konnte nicht geöffnet werden.';

  @override
  String get errorNoAudioTrack =>
      'Diese Datei enthält kein Audio. Nimm eine WAV-, MP3- oder FLAC-Datei.';

  @override
  String get errorAudioFormat =>
      'Dieses Audioformat wird nicht unterstützt. Wandle die Datei in WAV um.';

  @override
  String get errorAudioTooLong =>
      'Diese Aufnahme ist länger als 3 Minuten: Teile sie in Stücke.';

  @override
  String get errorMicUnavailable => 'Das Mikrofon ist nicht verfügbar.';

  @override
  String get errorRecordingFailed =>
      'Die Aufnahme ist fehlgeschlagen. Bitte versuch es noch einmal.';

  @override
  String get rangeSeparator => '–';

  @override
  String clockFormat(int minutes, String seconds) {
    return '$minutes:$seconds';
  }

  @override
  String get tabReminders => 'Erinnerungen';

  @override
  String get tabVoices => 'Stimmen';

  @override
  String get tabSettings => 'Einstellungen';

  @override
  String get remindersTitle => 'Erinnerungen';

  @override
  String get remindersEmpty =>
      'Noch keine Erinnerungen.\nTippe auf +, um einen lieben Menschen zur richtigen Zeit zu hören.';

  @override
  String get remindersAdd => 'Erinnerung hinzufügen';

  @override
  String get reminderSwitchDescription => 'Erinnerung an oder aus';

  @override
  String reminderTimeInterval(int minutes) {
    return '$minutes Min.';
  }

  @override
  String reminderThemeLine(String theme, String tone) {
    return '$theme · $tone';
  }

  @override
  String get reminderOwnText => 'Eigener Text';

  @override
  String get reminderVoiceUnavailable =>
      'Stimme nicht verfügbar – tippen, um eine andere zu wählen';

  @override
  String get editReminderTitleNew => 'Neue Erinnerung';

  @override
  String get editReminderTitleEdit => 'Erinnerung bearbeiten';

  @override
  String get reminderVoiceHint => 'Stimme';

  @override
  String get reminderNoVoices =>
      'Noch keine Stimme bereit. Erstelle zuerst eine im Tab Stimmen.';

  @override
  String get reminderGoToVoices => 'Zu den Stimmen';

  @override
  String get reminderThemeHint => 'Thema';

  @override
  String get reminderToneHint => 'Tonfall';

  @override
  String get toneSuggested => 'Vom Thema vorgeschlagen';

  @override
  String toneSuggestedWith(String tone) {
    return 'Vom Thema vorgeschlagen ($tone)';
  }

  @override
  String toneOption(String tone, String description) {
    return '$tone — $description';
  }

  @override
  String get pitchNatural => 'Tonhöhe: natürlich';

  @override
  String get pitchDescription => 'Tonhöhe in Halbtönen verschieben';

  @override
  String get reminderPlayNow => 'Jetzt anhören';

  @override
  String get reminderPlaying => 'Nachricht wird vorbereitet …';

  @override
  String get repeatTitle => 'Wiederholen';

  @override
  String get repeatKindDaily => 'Täglich';

  @override
  String get repeatKindWeekly => 'Bestimmte Tage';

  @override
  String get repeatKindInterval => 'Alle N Minuten';

  @override
  String get repeatKindOnce => 'Einmal';

  @override
  String get repeatTimeLabel => 'Um';

  @override
  String get repeatDateLabel => 'Am';

  @override
  String get repeatPickDate => 'Datum wählen';

  @override
  String repeatDateValue(String day, String month) {
    return '$day/$month';
  }

  @override
  String get repeatEveryHint => 'Alle';

  @override
  String get repeatEverySuffix => 'Minuten';

  @override
  String get repeatWindowLabel => 'Zwischen';

  @override
  String get repeatLabelDaily => 'Täglich';

  @override
  String get repeatLabelWeekdays => 'Werktags';

  @override
  String get repeatLabelWeekends => 'Am Wochenende';

  @override
  String repeatLabelWithin(String repeat, String from, String to) {
    return '$repeat · $from–$to';
  }

  @override
  String repeatLabelOnce(String date) {
    return 'Einmal · $date';
  }

  @override
  String get dayMonShort => 'M';

  @override
  String get dayTueShort => 'D';

  @override
  String get dayWedShort => 'M';

  @override
  String get dayThuShort => 'D';

  @override
  String get dayFriShort => 'F';

  @override
  String get daySatShort => 'S';

  @override
  String get daySunShort => 'S';

  @override
  String get dayMon => 'Montag';

  @override
  String get dayTue => 'Dienstag';

  @override
  String get dayWed => 'Mittwoch';

  @override
  String get dayThu => 'Donnerstag';

  @override
  String get dayFri => 'Freitag';

  @override
  String get daySat => 'Samstag';

  @override
  String get daySun => 'Sonntag';

  @override
  String get dayMonAbbr => 'Mo';

  @override
  String get dayTueAbbr => 'Di';

  @override
  String get dayWedAbbr => 'Mi';

  @override
  String get dayThuAbbr => 'Do';

  @override
  String get dayFriAbbr => 'Fr';

  @override
  String get daySatAbbr => 'Sa';

  @override
  String get daySunAbbr => 'So';

  @override
  String get quietTitle => 'Ruhezeit';

  @override
  String get quietHelp => 'Keine Nachrichten zwischen diesen Uhrzeiten.';

  @override
  String get formErrorNoVoice => 'Wähle zuerst eine Stimme.';

  @override
  String get formErrorNoTheme => 'Wähle ein Thema.';

  @override
  String get formErrorNoDays => 'Wähle mindestens einen Tag.';

  @override
  String get formErrorInterval =>
      'Gib an, wie viele Minuten zwischen den Nachrichten liegen (1 oder mehr).';

  @override
  String get formErrorNoDate => 'Wähle das Datum.';

  @override
  String get voicesTitle => 'Stimmen';

  @override
  String get voicesEmpty =>
      'Noch keine Stimmen.\nTippe auf +, um die Stimme eines lieben Menschen hinzuzufügen.';

  @override
  String get voicesAdd => 'Stimme hinzufügen';

  @override
  String get voiceStatusReady => 'Stimme bereit';

  @override
  String get voiceStatusNotCreated => 'Stimme noch nicht erstellt';

  @override
  String samplesCountAndSecs(String recordings, String duration) {
    return '$recordings · $duration';
  }

  @override
  String get importVoiceTitleNew => 'Neue Stimme';

  @override
  String get importVoiceTitleEdit => 'Stimme';

  @override
  String get voiceNameHint => 'Name (z. B. Oma Lucia)';

  @override
  String get voiceNameRequired => 'Gib den Namen ein.';

  @override
  String get voiceConsentHint => 'Wer hat die Nutzung dieser Stimme erlaubt?';

  @override
  String get voiceConsentHelp =>
      'Pflichtfeld: Nutze eine Stimme nur mit Erlaubnis der Person.';

  @override
  String get voiceConsentRequired =>
      'Gib an, wer die Nutzung dieser Stimme erlaubt hat.';

  @override
  String get voiceLanguageHint => 'Sprache';

  @override
  String get voiceListenerHint => 'Wie diese Stimme dich nennt';

  @override
  String get voiceListenerHelp =>
      'Optional, z. B. ein Spitzname. Leer: dein Name aus den Einstellungen.';

  @override
  String get voiceRegistered =>
      'Stimme gespeichert. Füge jetzt Aufnahmen hinzu.';

  @override
  String get voiceSaved => 'Gespeichert.';

  @override
  String get voiceCreated =>
      'Stimme erstellt! Sie ist bereit für deine Erinnerungen.';

  @override
  String get samplesTitle => 'Aufnahmen';

  @override
  String get samplesAddFiles => 'Audiodateien hinzufügen';

  @override
  String get samplesRecord => 'Aufnehmen';

  @override
  String get samplesStop => 'Aufnahme beenden';

  @override
  String samplesAdded(int count) {
    return 'Aufnahmen hinzugefügt: $count';
  }

  @override
  String sampleTitle(int number) {
    return 'Aufnahme $number';
  }

  @override
  String sampleDuration(String secs) {
    return '$secs s';
  }

  @override
  String recordingStatus(String elapsed, String limit) {
    return 'Aufnahme … $elapsed von $limit';
  }

  @override
  String get recordPermissionDenied =>
      'Erlaube das Mikrofon, um eine Stimme aufzunehmen.';

  @override
  String get actionCancel => 'Abbrechen';

  @override
  String get deleteVoice => 'Stimme löschen';

  @override
  String get deleteVoiceTitle => 'Diese Stimme löschen?';

  @override
  String get deleteVoiceMessage =>
      'Ihre Aufnahmen und die erstellte Stimme werden von diesem Handy und vom Stimmserver gelöscht. Erinnerungen mit dieser Stimme werden ausgeschaltet und bleiben erhalten, damit du eine andere Stimme wählen kannst.';

  @override
  String get deleteVoiceConfirm => 'Löschen';

  @override
  String get voiceDeleted => 'Stimme gelöscht.';

  @override
  String get createVoice => 'Stimme erstellen';

  @override
  String get createVoiceRunning => 'Stimme wird erstellt …';

  @override
  String get createVoiceHint =>
      'Füge mindestens 10 Sekunden Aufnahmen hinzu (WAV, MP3 oder FLAC, je 2 s bis 3 min) und erstelle dann die Stimme.';

  @override
  String get createVoiceHintReady =>
      'Die Stimme ist bereit. Neue Aufnahmen hinzugefügt? Erstelle sie erneut, um sie zu verbessern.';

  @override
  String get settingsTitle => 'Einstellungen';

  @override
  String get settingsListenerHint => 'Dein Name';

  @override
  String get settingsListenerHelp =>
      'Wird in den Nachrichten verwendet, wenn eine Stimme keinen Spitznamen für dich hat.';

  @override
  String get settingsEngineHint => 'URL des Stimmservers';

  @override
  String get settingsSaved => 'Einstellungen gespeichert.';

  @override
  String get settingsLanguageHint => 'App-Sprache';

  @override
  String get languageSystemDefault => 'Systemstandard';

  @override
  String get channelName => 'Geplante Nachrichten';

  @override
  String get notificationTitle => 'Eine Nachricht für dich';

  @override
  String notificationBody(String text, String tone) {
    return '$text\n(Tonfall: $tone)';
  }

  @override
  String pitchHigher(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: 'Tonhöhe: $count Halbtöne höher',
      one: 'Tonhöhe: $count Halbton höher',
    );
    return '$_temp0';
  }

  @override
  String pitchLower(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: 'Tonhöhe: $count Halbtöne tiefer',
      one: 'Tonhöhe: $count Halbton tiefer',
    );
    return '$_temp0';
  }

  @override
  String repeatLabelEvery(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: 'Alle $count Min.',
      one: 'Jede Minute',
    );
    return '$_temp0';
  }

  @override
  String samplesCount(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: '$count Aufnahmen',
      one: '$count Aufnahme',
    );
    return '$_temp0';
  }

  @override
  String seconds(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: '$count Sekunden',
      one: '$count Sekunde',
    );
    return '$_temp0';
  }

  @override
  String get deleteReminder => 'Erinnerung löschen';

  @override
  String get samplesFillFirst =>
      'Gib den Namen ein und wer zugestimmt hat, dann füge Aufnahmen hinzu.';
}
