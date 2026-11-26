// ignore: unused_import
import 'package:intl/intl.dart' as intl;

import 'app_localizations.dart';

// ignore_for_file: type=lint

/// The translations for Norwegian (`no`).
class AppLocalizationsNo extends AppLocalizations {
  AppLocalizationsNo([String locale = 'no']) : super(locale);

  @override
  String get appName => 'Friendly Voice';

  @override
  String get actionBack => 'Tilbake';

  @override
  String get errorGeneric => 'Noe gikk galt. Prøv igjen.';

  @override
  String get errorPlayback => 'Kunne ikke spille av meldingen.';

  @override
  String get errorFileOpen => 'Kunne ikke åpne filen.';

  @override
  String get errorNoAudioTrack =>
      'Filen har ingen lyd. Bruk en WAV-, MP3- eller FLAC-fil.';

  @override
  String get errorAudioFormat =>
      'Dette lydformatet støttes ikke. Konverter filen til WAV.';

  @override
  String get errorAudioTooLong =>
      'Opptaket er lengre enn 3 minutter: del det opp.';

  @override
  String get errorMicUnavailable => 'Mikrofonen er ikke tilgjengelig.';

  @override
  String get errorRecordingFailed => 'Opptaket mislyktes. Prøv igjen.';

  @override
  String get rangeSeparator => '–';

  @override
  String clockFormat(int minutes, String seconds) {
    return '$minutes:$seconds';
  }

  @override
  String get tabReminders => 'Påminnelser';

  @override
  String get tabVoices => 'Stemmer';

  @override
  String get tabSettings => 'Innstillinger';

  @override
  String get remindersTitle => 'Påminnelser';

  @override
  String get remindersEmpty =>
      'Ingen påminnelser ennå.\nTrykk på + for å høre noen du er glad i til rett tid.';

  @override
  String get remindersAdd => 'Legg til påminnelse';

  @override
  String get reminderSwitchDescription => 'Påminnelse på eller av';

  @override
  String reminderTimeInterval(int minutes) {
    return '$minutes min';
  }

  @override
  String reminderThemeLine(String theme, String tone) {
    return '$theme · $tone';
  }

  @override
  String get reminderOwnText => 'Din egen tekst';

  @override
  String get reminderVoiceUnavailable =>
      'Stemmen er ikke tilgjengelig – trykk for å velge en annen';

  @override
  String get editReminderTitleNew => 'Ny påminnelse';

  @override
  String get editReminderTitleEdit => 'Endre påminnelse';

  @override
  String get reminderVoiceHint => 'Stemme';

  @override
  String get reminderNoVoices =>
      'Ingen stemme er klar ennå. Lag en først i Stemmer-fanen.';

  @override
  String get reminderGoToVoices => 'Gå til Stemmer';

  @override
  String get reminderThemeHint => 'Tema';

  @override
  String get reminderToneHint => 'Tonefall';

  @override
  String get toneSuggested => 'Foreslått av temaet';

  @override
  String toneSuggestedWith(String tone) {
    return 'Foreslått av temaet ($tone)';
  }

  @override
  String toneOption(String tone, String description) {
    return '$tone — $description';
  }

  @override
  String get pitchNatural => 'Tonehøyde: naturlig';

  @override
  String get pitchDescription => 'Tonehøyde i halvtoner';

  @override
  String get reminderPlayNow => 'Spill av nå';

  @override
  String get reminderPlaying => 'Gjør klar meldingen …';

  @override
  String get repeatTitle => 'Gjenta';

  @override
  String get repeatKindDaily => 'Hver dag';

  @override
  String get repeatKindWeekly => 'Valgte dager';

  @override
  String get repeatKindInterval => 'Hvert N. minutt';

  @override
  String get repeatKindOnce => 'Én gang';

  @override
  String get repeatTimeLabel => 'Klokken';

  @override
  String get repeatDateLabel => 'Dato';

  @override
  String get repeatPickDate => 'Velg en dato';

  @override
  String repeatDateValue(String day, String month) {
    return '$day/$month';
  }

  @override
  String get repeatEveryHint => 'Hvert';

  @override
  String get repeatEverySuffix => 'minutt';

  @override
  String get repeatWindowLabel => 'Mellom';

  @override
  String get repeatLabelDaily => 'Hver dag';

  @override
  String get repeatLabelWeekdays => 'Hverdager';

  @override
  String get repeatLabelWeekends => 'Helger';

  @override
  String repeatLabelWithin(String repeat, String from, String to) {
    return '$repeat · $from–$to';
  }

  @override
  String repeatLabelOnce(String date) {
    return 'Én gang · $date';
  }

  @override
  String get dayMonShort => 'M';

  @override
  String get dayTueShort => 'T';

  @override
  String get dayWedShort => 'O';

  @override
  String get dayThuShort => 'T';

  @override
  String get dayFriShort => 'F';

  @override
  String get daySatShort => 'L';

  @override
  String get daySunShort => 'S';

  @override
  String get dayMon => 'Mandag';

  @override
  String get dayTue => 'Tirsdag';

  @override
  String get dayWed => 'Onsdag';

  @override
  String get dayThu => 'Torsdag';

  @override
  String get dayFri => 'Fredag';

  @override
  String get daySat => 'Lørdag';

  @override
  String get daySun => 'Søndag';

  @override
  String get dayMonAbbr => 'Man';

  @override
  String get dayTueAbbr => 'Tir';

  @override
  String get dayWedAbbr => 'Ons';

  @override
  String get dayThuAbbr => 'Tor';

  @override
  String get dayFriAbbr => 'Fre';

  @override
  String get daySatAbbr => 'Lør';

  @override
  String get daySunAbbr => 'Søn';

  @override
  String get quietTitle => 'Stilletimer';

  @override
  String get quietHelp => 'Ingen meldinger mellom disse tidspunktene.';

  @override
  String get formErrorNoVoice => 'Velg en stemme først.';

  @override
  String get formErrorNoTheme => 'Velg et tema.';

  @override
  String get formErrorNoDays => 'Velg minst én dag.';

  @override
  String get formErrorInterval =>
      'Skriv hvor mange minutter det skal gå mellom meldingene (1 eller flere).';

  @override
  String get formErrorNoDate => 'Velg datoen.';

  @override
  String get voicesTitle => 'Stemmer';

  @override
  String get voicesEmpty =>
      'Ingen stemmer ennå.\nTrykk på + for å legge til stemmen til noen du er glad i.';

  @override
  String get voicesAdd => 'Legg til stemme';

  @override
  String get voiceStatusReady => 'Stemmen er klar';

  @override
  String get voiceStatusNotCreated => 'Stemmen er ikke laget ennå';

  @override
  String samplesCountAndSecs(String recordings, String duration) {
    return '$recordings · $duration';
  }

  @override
  String get importVoiceTitleNew => 'Ny stemme';

  @override
  String get importVoiceTitleEdit => 'Stemme';

  @override
  String get voiceNameHint => 'Navn (f.eks. Bestemor Lucia)';

  @override
  String get voiceNameRequired => 'Skriv inn navnet.';

  @override
  String get voiceConsentHint =>
      'Hvem som har gitt tillatelse til å bruke stemmen';

  @override
  String get voiceConsentHelp =>
      'Påkrevd: bruk bare en stemme med personens tillatelse.';

  @override
  String get voiceConsentRequired =>
      'Skriv hvem som har gitt tillatelse til å bruke stemmen.';

  @override
  String get voiceLanguageHint => 'Språk';

  @override
  String get voiceListenerHint => 'Hva denne stemmen kaller deg';

  @override
  String get voiceListenerHelp =>
      'Valgfritt, f.eks. et kallenavn. Tomt: navnet ditt fra Innstillinger.';

  @override
  String get voiceRegistered => 'Stemmen er lagret. Legg til opptak nå.';

  @override
  String get voiceSaved => 'Lagret.';

  @override
  String get voiceCreated =>
      'Stemmen er laget! Den er klar for påminnelsene dine.';

  @override
  String get samplesTitle => 'Opptak';

  @override
  String get samplesAddFiles => 'Legg til lydfiler';

  @override
  String get samplesRecord => 'Ta opp';

  @override
  String get samplesStop => 'Stopp opptaket';

  @override
  String samplesAdded(int count) {
    return 'Opptak lagt til: $count';
  }

  @override
  String sampleTitle(int number) {
    return 'Opptak $number';
  }

  @override
  String sampleDuration(String secs) {
    return '$secs s';
  }

  @override
  String recordingStatus(String elapsed, String limit) {
    return 'Tar opp … $elapsed av $limit';
  }

  @override
  String get recordPermissionDenied =>
      'Gi tilgang til mikrofonen for å ta opp en stemme.';

  @override
  String get actionCancel => 'Avbryt';

  @override
  String get deleteVoice => 'Slett stemme';

  @override
  String get deleteVoiceTitle => 'Vil du slette denne stemmen?';

  @override
  String get deleteVoiceMessage =>
      'Opptakene og den lagde stemmen slettes fra denne telefonen og stemmeserveren. Påminnelser som bruker den, slås av og beholdes, slik at du kan velge en annen stemme.';

  @override
  String get deleteVoiceConfirm => 'Slett';

  @override
  String get voiceDeleted => 'Stemmen er slettet.';

  @override
  String get createVoice => 'Lag stemme';

  @override
  String get createVoiceRunning => 'Lager stemmen …';

  @override
  String get createVoiceHint =>
      'Legg til minst 10 sekunder med opptak (WAV, MP3 eller FLAC, 2 s til 3 min hver), og lag deretter stemmen.';

  @override
  String get createVoiceHintReady =>
      'Stemmen er klar. Har du lagt til flere opptak? Lag den på nytt for å forbedre den.';

  @override
  String get settingsTitle => 'Innstillinger';

  @override
  String get settingsListenerHint => 'Navnet ditt';

  @override
  String get settingsListenerHelp =>
      'Brukes i meldingene når en stemme ikke har et kallenavn på deg.';

  @override
  String get settingsEngineHint => 'Adresse til stemmeserveren';

  @override
  String get settingsSaved => 'Innstillingene er lagret.';

  @override
  String get settingsLanguageHint => 'Språk i appen';

  @override
  String get languageSystemDefault => 'Systemstandard';

  @override
  String get channelName => 'Planlagte meldinger';

  @override
  String get notificationTitle => 'En melding til deg';

  @override
  String notificationBody(String text, String tone) {
    return '$text\n(tonefall: $tone)';
  }

  @override
  String pitchHigher(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: 'Tonehøyde: $count halvtoner lysere',
      one: 'Tonehøyde: $count halvtone lysere',
    );
    return '$_temp0';
  }

  @override
  String pitchLower(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: 'Tonehøyde: $count halvtoner dypere',
      one: 'Tonehøyde: $count halvtone dypere',
    );
    return '$_temp0';
  }

  @override
  String repeatLabelEvery(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: 'Hvert $count. minutt',
      one: 'Hvert minutt',
    );
    return '$_temp0';
  }

  @override
  String samplesCount(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: '$count opptak',
      one: '$count opptak',
    );
    return '$_temp0';
  }

  @override
  String seconds(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: '$count sekunder',
      one: '$count sekund',
    );
    return '$_temp0';
  }

  @override
  String get deleteReminder => 'Slett påminnelse';

  @override
  String get samplesFillFirst =>
      'Skriv inn navnet og hvem som ga tillatelse, og legg deretter til opptak.';
}
