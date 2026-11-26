// ignore: unused_import
import 'package:intl/intl.dart' as intl;

import 'app_localizations.dart';

// ignore_for_file: type=lint

/// The translations for Italian (`it`).
class AppLocalizationsIt extends AppLocalizations {
  AppLocalizationsIt([String locale = 'it']) : super(locale);

  @override
  String get appName => 'Friendly Voice';

  @override
  String get actionBack => 'Indietro';

  @override
  String get errorGeneric => 'Qualcosa è andato storto. Riprova.';

  @override
  String get errorPlayback => 'Impossibile riprodurre il messaggio.';

  @override
  String get errorFileOpen => 'Impossibile aprire il file.';

  @override
  String get errorNoAudioTrack =>
      'Questo file non contiene audio. Usa un file WAV, MP3 o FLAC.';

  @override
  String get errorAudioFormat =>
      'Questo formato audio non è supportato. Converti il file in WAV.';

  @override
  String get errorAudioTooLong =>
      'Questa registrazione supera i 3 minuti: dividila in parti.';

  @override
  String get errorMicUnavailable => 'Il microfono non è disponibile.';

  @override
  String get errorRecordingFailed =>
      'La registrazione non è riuscita. Riprova.';

  @override
  String get rangeSeparator => '–';

  @override
  String clockFormat(int minutes, String seconds) {
    return '$minutes:$seconds';
  }

  @override
  String get tabReminders => 'Promemoria';

  @override
  String get tabVoices => 'Voci';

  @override
  String get tabSettings => 'Impostazioni';

  @override
  String get remindersTitle => 'Promemoria';

  @override
  String get remindersEmpty =>
      'Ancora nessun promemoria.\nTocca + per sentire chi ami al momento giusto.';

  @override
  String get remindersAdd => 'Aggiungi promemoria';

  @override
  String get reminderSwitchDescription => 'Promemoria attivo o disattivo';

  @override
  String reminderTimeInterval(int minutes) {
    return '$minutes min';
  }

  @override
  String reminderThemeLine(String theme, String tone) {
    return '$theme · $tone';
  }

  @override
  String get reminderOwnText => 'Testo personale';

  @override
  String get reminderVoiceUnavailable =>
      'Voce non disponibile — tocca per sceglierne un’altra';

  @override
  String get editReminderTitleNew => 'Nuovo promemoria';

  @override
  String get editReminderTitleEdit => 'Modifica promemoria';

  @override
  String get reminderVoiceHint => 'Voce';

  @override
  String get reminderNoVoices =>
      'Nessuna voce è ancora pronta. Creane prima una nella scheda Voci.';

  @override
  String get reminderGoToVoices => 'Vai a Voci';

  @override
  String get reminderThemeHint => 'Tema';

  @override
  String get reminderToneHint => 'Tono di voce';

  @override
  String get toneSuggested => 'Suggerito dal tema';

  @override
  String toneSuggestedWith(String tone) {
    return 'Suggerito dal tema ($tone)';
  }

  @override
  String toneOption(String tone, String description) {
    return '$tone — $description';
  }

  @override
  String get pitchNatural => 'Altezza: naturale';

  @override
  String get pitchDescription => 'Variazione di altezza in semitoni';

  @override
  String get reminderPlayNow => 'Ascolta ora';

  @override
  String get reminderPlaying => 'Preparo il messaggio…';

  @override
  String get repeatTitle => 'Ripeti';

  @override
  String get repeatKindDaily => 'Ogni giorno';

  @override
  String get repeatKindWeekly => 'Giorni scelti';

  @override
  String get repeatKindInterval => 'Ogni N minuti';

  @override
  String get repeatKindOnce => 'Una volta';

  @override
  String get repeatTimeLabel => 'Alle';

  @override
  String get repeatDateLabel => 'Il';

  @override
  String get repeatPickDate => 'Scegli una data';

  @override
  String repeatDateValue(String day, String month) {
    return '$day/$month';
  }

  @override
  String get repeatEveryHint => 'Ogni';

  @override
  String get repeatEverySuffix => 'minuti';

  @override
  String get repeatWindowLabel => 'Tra';

  @override
  String get repeatLabelDaily => 'Ogni giorno';

  @override
  String get repeatLabelWeekdays => 'Giorni feriali';

  @override
  String get repeatLabelWeekends => 'Fine settimana';

  @override
  String repeatLabelWithin(String repeat, String from, String to) {
    return '$repeat · $from–$to';
  }

  @override
  String repeatLabelOnce(String date) {
    return 'Una volta · $date';
  }

  @override
  String get dayMonShort => 'L';

  @override
  String get dayTueShort => 'M';

  @override
  String get dayWedShort => 'M';

  @override
  String get dayThuShort => 'G';

  @override
  String get dayFriShort => 'V';

  @override
  String get daySatShort => 'S';

  @override
  String get daySunShort => 'D';

  @override
  String get dayMon => 'Lunedì';

  @override
  String get dayTue => 'Martedì';

  @override
  String get dayWed => 'Mercoledì';

  @override
  String get dayThu => 'Giovedì';

  @override
  String get dayFri => 'Venerdì';

  @override
  String get daySat => 'Sabato';

  @override
  String get daySun => 'Domenica';

  @override
  String get dayMonAbbr => 'Lun';

  @override
  String get dayTueAbbr => 'Mar';

  @override
  String get dayWedAbbr => 'Mer';

  @override
  String get dayThuAbbr => 'Gio';

  @override
  String get dayFriAbbr => 'Ven';

  @override
  String get daySatAbbr => 'Sab';

  @override
  String get daySunAbbr => 'Dom';

  @override
  String get quietTitle => 'Ore di silenzio';

  @override
  String get quietHelp => 'Nessun messaggio tra questi orari.';

  @override
  String get formErrorNoVoice => 'Scegli prima una voce.';

  @override
  String get formErrorNoTheme => 'Scegli un tema.';

  @override
  String get formErrorNoDays => 'Scegli almeno un giorno.';

  @override
  String get formErrorInterval =>
      'Indica quanti minuti tra un messaggio e l’altro (1 o più).';

  @override
  String get formErrorNoDate => 'Scegli la data.';

  @override
  String get voicesTitle => 'Voci';

  @override
  String get voicesEmpty =>
      'Ancora nessuna voce.\nTocca + per aggiungere la voce di una persona cara.';

  @override
  String get voicesAdd => 'Aggiungi voce';

  @override
  String get voiceStatusReady => 'Voce pronta';

  @override
  String get voiceStatusNotCreated => 'Voce non ancora creata';

  @override
  String samplesCountAndSecs(String recordings, String duration) {
    return '$recordings · $duration';
  }

  @override
  String get importVoiceTitleNew => 'Nuova voce';

  @override
  String get importVoiceTitleEdit => 'Voce';

  @override
  String get voiceNameHint => 'Nome (es. Nonna Lucia)';

  @override
  String get voiceNameRequired => 'Inserisci il nome.';

  @override
  String get voiceConsentHint => 'Chi ha autorizzato l’uso di questa voce';

  @override
  String get voiceConsentHelp =>
      'Obbligatorio: usa una voce solo con il permesso della persona.';

  @override
  String get voiceConsentRequired =>
      'Indica chi ha autorizzato l’uso di questa voce.';

  @override
  String get voiceLanguageHint => 'Lingua';

  @override
  String get voiceListenerHint => 'Come ti chiama questa voce';

  @override
  String get voiceListenerHelp =>
      'Facoltativo, es. un soprannome. Vuoto: il tuo nome nelle Impostazioni.';

  @override
  String get voiceRegistered => 'Voce salvata. Ora aggiungi le registrazioni.';

  @override
  String get voiceSaved => 'Salvato.';

  @override
  String get voiceCreated => 'Voce creata! È pronta per i tuoi promemoria.';

  @override
  String get samplesTitle => 'Registrazioni';

  @override
  String get samplesAddFiles => 'Aggiungi file audio';

  @override
  String get samplesRecord => 'Registra';

  @override
  String get samplesStop => 'Ferma registrazione';

  @override
  String samplesAdded(int count) {
    return 'Registrazioni aggiunte: $count';
  }

  @override
  String sampleTitle(int number) {
    return 'Registrazione $number';
  }

  @override
  String sampleDuration(String secs) {
    return '$secs s';
  }

  @override
  String recordingStatus(String elapsed, String limit) {
    return 'Registrazione… $elapsed di $limit';
  }

  @override
  String get recordPermissionDenied =>
      'Consenti l’uso del microfono per registrare una voce.';

  @override
  String get actionCancel => 'Annulla';

  @override
  String get deleteVoice => 'Elimina voce';

  @override
  String get deleteVoiceTitle => 'Eliminare questa voce?';

  @override
  String get deleteVoiceMessage =>
      'Le registrazioni e la voce creata vengono cancellate da questo telefono e dal server vocale. I promemoria che la usano vengono disattivati e conservati, così puoi scegliere un’altra voce.';

  @override
  String get deleteVoiceConfirm => 'Elimina';

  @override
  String get voiceDeleted => 'Voce eliminata.';

  @override
  String get createVoice => 'Crea voce';

  @override
  String get createVoiceRunning => 'Creo la voce…';

  @override
  String get createVoiceHint =>
      'Aggiungi almeno 10 secondi di registrazioni (WAV, MP3 o FLAC, da 2 s a 3 min ciascuna), poi crea la voce.';

  @override
  String get createVoiceHintReady =>
      'La voce è pronta. Hai aggiunto altre registrazioni? Ricreala per migliorarla.';

  @override
  String get settingsTitle => 'Impostazioni';

  @override
  String get settingsListenerHint => 'Il tuo nome';

  @override
  String get settingsListenerHelp =>
      'Usato nei messaggi quando una voce non ha un soprannome per te.';

  @override
  String get settingsEngineHint => 'URL del server vocale';

  @override
  String get settingsSaved => 'Impostazioni salvate.';

  @override
  String get settingsLanguageHint => 'Lingua dell’app';

  @override
  String get languageSystemDefault => 'Predefinita di sistema';

  @override
  String get channelName => 'Messaggi programmati';

  @override
  String get notificationTitle => 'Un messaggio per te';

  @override
  String notificationBody(String text, String tone) {
    return '$text\n(tono: $tone)';
  }

  @override
  String pitchHigher(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: 'Altezza: $count semitoni più acuta',
      many: 'Altezza: $count semitoni più acuta',
      one: 'Altezza: $count semitono più acuta',
    );
    return '$_temp0';
  }

  @override
  String pitchLower(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: 'Altezza: $count semitoni più grave',
      many: 'Altezza: $count semitoni più grave',
      one: 'Altezza: $count semitono più grave',
    );
    return '$_temp0';
  }

  @override
  String repeatLabelEvery(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: 'Ogni $count min',
      many: 'Ogni $count min',
      one: 'Ogni minuto',
    );
    return '$_temp0';
  }

  @override
  String samplesCount(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: '$count registrazioni',
      many: '$count registrazioni',
      one: '$count registrazione',
    );
    return '$_temp0';
  }

  @override
  String seconds(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: '$count secondi',
      many: '$count secondi',
      one: '$count secondo',
    );
    return '$_temp0';
  }

  @override
  String get deleteReminder => 'Elimina promemoria';

  @override
  String get samplesFillFirst =>
      'Inserisci il nome e chi ha dato l’autorizzazione, poi aggiungi le registrazioni.';
}
