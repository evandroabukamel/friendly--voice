// ignore: unused_import
import 'package:intl/intl.dart' as intl;

import 'app_localizations.dart';

// ignore_for_file: type=lint

/// The translations for Finnish (`fi`).
class AppLocalizationsFi extends AppLocalizations {
  AppLocalizationsFi([String locale = 'fi']) : super(locale);

  @override
  String get appName => 'Friendly Voice';

  @override
  String get actionBack => 'Takaisin';

  @override
  String get errorGeneric => 'Jokin meni pieleen. Yritä uudelleen.';

  @override
  String get errorPlayback => 'Viestiä ei voitu toistaa.';

  @override
  String get errorFileOpen => 'Tiedostoa ei voitu avata.';

  @override
  String get errorNoAudioTrack =>
      'Tiedostossa ei ole ääntä. Käytä WAV-, MP3- tai FLAC-tiedostoa.';

  @override
  String get errorAudioFormat =>
      'Tätä äänimuotoa ei tueta. Muunna tiedosto WAV-muotoon.';

  @override
  String get errorAudioTooLong =>
      'Tallenne on yli 3 minuuttia pitkä: jaa se osiin.';

  @override
  String get errorMicUnavailable => 'Mikrofoni ei ole käytettävissä.';

  @override
  String get errorRecordingFailed => 'Tallennus epäonnistui. Yritä uudelleen.';

  @override
  String get rangeSeparator => '–';

  @override
  String clockFormat(int minutes, String seconds) {
    return '$minutes:$seconds';
  }

  @override
  String get tabReminders => 'Muistutukset';

  @override
  String get tabVoices => 'Äänet';

  @override
  String get tabSettings => 'Asetukset';

  @override
  String get remindersTitle => 'Muistutukset';

  @override
  String get remindersEmpty =>
      'Ei vielä muistutuksia.\nNapauta + ja kuule rakkaasi ääni juuri oikeaan aikaan.';

  @override
  String get remindersAdd => 'Lisää muistutus';

  @override
  String get reminderSwitchDescription => 'Muistutus päällä tai pois';

  @override
  String reminderTimeInterval(int minutes) {
    return '$minutes min';
  }

  @override
  String reminderThemeLine(String theme, String tone) {
    return '$theme · $tone';
  }

  @override
  String get reminderOwnText => 'Oma teksti';

  @override
  String get reminderVoiceUnavailable =>
      'Ääni ei ole käytettävissä — valitse toinen napauttamalla';

  @override
  String get editReminderTitleNew => 'Uusi muistutus';

  @override
  String get editReminderTitleEdit => 'Muokkaa muistutusta';

  @override
  String get reminderVoiceHint => 'Ääni';

  @override
  String get reminderNoVoices =>
      'Valmiita ääniä ei vielä ole. Luo ensin ääni Äänet-välilehdellä.';

  @override
  String get reminderGoToVoices => 'Siirry ääniin';

  @override
  String get reminderThemeHint => 'Teema';

  @override
  String get reminderToneHint => 'Äänensävy';

  @override
  String get toneSuggested => 'Teeman ehdottama';

  @override
  String toneSuggestedWith(String tone) {
    return 'Teeman ehdottama ($tone)';
  }

  @override
  String toneOption(String tone, String description) {
    return '$tone — $description';
  }

  @override
  String get pitchNatural => 'Korkeus: luonnollinen';

  @override
  String get pitchDescription => 'Äänenkorkeuden siirto puolisävelaskelina';

  @override
  String get reminderPlayNow => 'Kuuntele nyt';

  @override
  String get reminderPlaying => 'Viestiä valmistellaan…';

  @override
  String get repeatTitle => 'Toisto';

  @override
  String get repeatKindDaily => 'Päivittäin';

  @override
  String get repeatKindWeekly => 'Valitut päivät';

  @override
  String get repeatKindInterval => 'N minuutin välein';

  @override
  String get repeatKindOnce => 'Kerran';

  @override
  String get repeatTimeLabel => 'Kello';

  @override
  String get repeatDateLabel => 'Päivä';

  @override
  String get repeatPickDate => 'Valitse päivä';

  @override
  String repeatDateValue(String day, String month) {
    return '$day/$month';
  }

  @override
  String get repeatEveryHint => 'Välein';

  @override
  String get repeatEverySuffix => 'minuuttia';

  @override
  String get repeatWindowLabel => 'Aikaväli';

  @override
  String get repeatLabelDaily => 'Päivittäin';

  @override
  String get repeatLabelWeekdays => 'Arkipäivisin';

  @override
  String get repeatLabelWeekends => 'Viikonloppuisin';

  @override
  String repeatLabelWithin(String repeat, String from, String to) {
    return '$repeat · $from–$to';
  }

  @override
  String repeatLabelOnce(String date) {
    return 'Kerran · $date';
  }

  @override
  String get dayMonShort => 'M';

  @override
  String get dayTueShort => 'T';

  @override
  String get dayWedShort => 'K';

  @override
  String get dayThuShort => 'T';

  @override
  String get dayFriShort => 'P';

  @override
  String get daySatShort => 'L';

  @override
  String get daySunShort => 'S';

  @override
  String get dayMon => 'Maanantai';

  @override
  String get dayTue => 'Tiistai';

  @override
  String get dayWed => 'Keskiviikko';

  @override
  String get dayThu => 'Torstai';

  @override
  String get dayFri => 'Perjantai';

  @override
  String get daySat => 'Lauantai';

  @override
  String get daySun => 'Sunnuntai';

  @override
  String get dayMonAbbr => 'Ma';

  @override
  String get dayTueAbbr => 'Ti';

  @override
  String get dayWedAbbr => 'Ke';

  @override
  String get dayThuAbbr => 'To';

  @override
  String get dayFriAbbr => 'Pe';

  @override
  String get daySatAbbr => 'La';

  @override
  String get daySunAbbr => 'Su';

  @override
  String get quietTitle => 'Hiljaiset tunnit';

  @override
  String get quietHelp => 'Ei viestejä näiden aikojen välillä.';

  @override
  String get formErrorNoVoice => 'Valitse ensin ääni.';

  @override
  String get formErrorNoTheme => 'Valitse teema.';

  @override
  String get formErrorNoDays => 'Valitse vähintään yksi päivä.';

  @override
  String get formErrorInterval =>
      'Anna viestien väli minuutteina (1 tai enemmän).';

  @override
  String get formErrorNoDate => 'Valitse päivä.';

  @override
  String get voicesTitle => 'Äänet';

  @override
  String get voicesEmpty =>
      'Ei vielä ääniä.\nNapauta + ja lisää rakkaasi ääni.';

  @override
  String get voicesAdd => 'Lisää ääni';

  @override
  String get voiceStatusReady => 'Ääni valmis';

  @override
  String get voiceStatusNotCreated => 'Ääntä ei ole vielä luotu';

  @override
  String samplesCountAndSecs(String recordings, String duration) {
    return '$recordings · $duration';
  }

  @override
  String get importVoiceTitleNew => 'Uusi ääni';

  @override
  String get importVoiceTitleEdit => 'Ääni';

  @override
  String get voiceNameHint => 'Nimi (esim. Mummo Lucia)';

  @override
  String get voiceNameRequired => 'Anna nimi.';

  @override
  String get voiceConsentHint => 'Kuka antoi luvan käyttää tätä ääntä';

  @override
  String get voiceConsentHelp =>
      'Pakollinen: käytä ääntä vain henkilön luvalla.';

  @override
  String get voiceConsentRequired =>
      'Kerro, kuka antoi luvan käyttää tätä ääntä.';

  @override
  String get voiceLanguageHint => 'Kieli';

  @override
  String get voiceListenerHint => 'Miksi tämä ääni sinua kutsuu';

  @override
  String get voiceListenerHelp =>
      'Valinnainen, esim. lempinimi. Tyhjä: nimesi asetuksista.';

  @override
  String get voiceRegistered => 'Ääni tallennettu. Lisää nyt tallenteita.';

  @override
  String get voiceSaved => 'Tallennettu.';

  @override
  String get voiceCreated => 'Ääni luotu! Se on valmis muistutuksiisi.';

  @override
  String get samplesTitle => 'Tallenteet';

  @override
  String get samplesAddFiles => 'Lisää äänitiedostoja';

  @override
  String get samplesRecord => 'Nauhoita';

  @override
  String get samplesStop => 'Lopeta nauhoitus';

  @override
  String samplesAdded(int count) {
    return 'Tallenteita lisätty: $count';
  }

  @override
  String sampleTitle(int number) {
    return 'Tallenne $number';
  }

  @override
  String sampleDuration(String secs) {
    return '$secs s';
  }

  @override
  String recordingStatus(String elapsed, String limit) {
    return 'Nauhoitetaan… $elapsed / $limit';
  }

  @override
  String get recordPermissionDenied =>
      'Salli mikrofoni, jotta voit nauhoittaa äänen.';

  @override
  String get actionCancel => 'Peruuta';

  @override
  String get deleteVoice => 'Poista ääni';

  @override
  String get deleteVoiceTitle => 'Poistetaanko tämä ääni?';

  @override
  String get deleteVoiceMessage =>
      'Sen tallenteet ja luotu ääni poistetaan tästä puhelimesta ja äänipalvelimelta. Ääntä käyttävät muistutukset kytketään pois ja säilytetään, joten voit valita toisen äänen.';

  @override
  String get deleteVoiceConfirm => 'Poista';

  @override
  String get voiceDeleted => 'Ääni poistettu.';

  @override
  String get createVoice => 'Luo ääni';

  @override
  String get createVoiceRunning => 'Ääntä luodaan…';

  @override
  String get createVoiceHint =>
      'Lisää vähintään 10 sekuntia tallenteita (WAV, MP3 tai FLAC, kukin 2 s – 3 min) ja luo sitten ääni.';

  @override
  String get createVoiceHintReady =>
      'Ääni on valmis. Lisäsitkö tallenteita? Luo se uudelleen, niin se paranee.';

  @override
  String get settingsTitle => 'Asetukset';

  @override
  String get settingsListenerHint => 'Nimesi';

  @override
  String get settingsListenerHelp =>
      'Käytetään viesteissä, kun äänellä ei ole sinulle lempinimeä.';

  @override
  String get settingsEngineHint => 'Äänipalvelimen osoite';

  @override
  String get settingsSaved => 'Asetukset tallennettu.';

  @override
  String get settingsLanguageHint => 'Sovelluksen kieli';

  @override
  String get languageSystemDefault => 'Järjestelmän oletus';

  @override
  String get channelName => 'Ajastetut viestit';

  @override
  String get notificationTitle => 'Viesti sinulle';

  @override
  String notificationBody(String text, String tone) {
    return '$text\n(sävy: $tone)';
  }

  @override
  String pitchHigher(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: 'Korkeus: $count puolisäveltä ylempänä',
      one: 'Korkeus: $count puolisävel ylempänä',
    );
    return '$_temp0';
  }

  @override
  String pitchLower(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: 'Korkeus: $count puolisäveltä alempana',
      one: 'Korkeus: $count puolisävel alempana',
    );
    return '$_temp0';
  }

  @override
  String repeatLabelEvery(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: '$count min välein',
      one: 'Joka minuutti',
    );
    return '$_temp0';
  }

  @override
  String samplesCount(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: '$count tallennetta',
      one: '$count tallenne',
    );
    return '$_temp0';
  }

  @override
  String seconds(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: '$count sekuntia',
      one: '$count sekunti',
    );
    return '$_temp0';
  }

  @override
  String get deleteReminder => 'Poista muistutus';

  @override
  String get samplesFillFirst =>
      'Kirjoita nimi ja luvan antaja, ja lisää sitten tallenteita.';
}
