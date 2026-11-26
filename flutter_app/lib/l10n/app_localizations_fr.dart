// ignore: unused_import
import 'package:intl/intl.dart' as intl;

import 'app_localizations.dart';

// ignore_for_file: type=lint

/// The translations for French (`fr`).
class AppLocalizationsFr extends AppLocalizations {
  AppLocalizationsFr([String locale = 'fr']) : super(locale);

  @override
  String get appName => 'Friendly Voice';

  @override
  String get actionBack => 'Retour';

  @override
  String get errorGeneric => 'Un problème est survenu. Réessayez.';

  @override
  String get errorPlayback => 'Impossible de lire le message.';

  @override
  String get errorFileOpen => 'Impossible d’ouvrir le fichier.';

  @override
  String get errorNoAudioTrack =>
      'Ce fichier ne contient pas d’audio. Utilisez un fichier WAV, MP3 ou FLAC.';

  @override
  String get errorAudioFormat =>
      'Ce format audio n’est pas pris en charge. Convertissez le fichier en WAV.';

  @override
  String get errorAudioTooLong =>
      'Cet enregistrement dépasse 3 minutes : découpez-le en morceaux.';

  @override
  String get errorMicUnavailable => 'Le micro n’est pas disponible.';

  @override
  String get errorRecordingFailed => 'L’enregistrement a échoué. Réessayez.';

  @override
  String get rangeSeparator => '–';

  @override
  String clockFormat(int minutes, String seconds) {
    return '$minutes:$seconds';
  }

  @override
  String get tabReminders => 'Rappels';

  @override
  String get tabVoices => 'Voix';

  @override
  String get tabSettings => 'Réglages';

  @override
  String get remindersTitle => 'Rappels';

  @override
  String get remindersEmpty =>
      'Aucun rappel pour l’instant.\nTouchez + pour entendre un être cher au bon moment.';

  @override
  String get remindersAdd => 'Ajouter un rappel';

  @override
  String get reminderSwitchDescription => 'Rappel activé ou désactivé';

  @override
  String reminderTimeInterval(int minutes) {
    return '$minutes min';
  }

  @override
  String reminderThemeLine(String theme, String tone) {
    return '$theme · $tone';
  }

  @override
  String get reminderOwnText => 'Votre propre texte';

  @override
  String get reminderVoiceUnavailable =>
      'Voix indisponible — touchez pour en choisir une autre';

  @override
  String get editReminderTitleNew => 'Nouveau rappel';

  @override
  String get editReminderTitleEdit => 'Modifier le rappel';

  @override
  String get reminderVoiceHint => 'Voix';

  @override
  String get reminderNoVoices =>
      'Aucune voix n’est prête. Créez-en une d’abord dans l’onglet Voix.';

  @override
  String get reminderGoToVoices => 'Aller aux voix';

  @override
  String get reminderThemeHint => 'Thème';

  @override
  String get reminderToneHint => 'Ton de voix';

  @override
  String get toneSuggested => 'Suggéré par le thème';

  @override
  String toneSuggestedWith(String tone) {
    return 'Suggéré par le thème ($tone)';
  }

  @override
  String toneOption(String tone, String description) {
    return '$tone — $description';
  }

  @override
  String get pitchNatural => 'Hauteur : naturelle';

  @override
  String get pitchDescription => 'Décalage de hauteur en demi-tons';

  @override
  String get reminderPlayNow => 'Écouter';

  @override
  String get reminderPlaying => 'Préparation du message…';

  @override
  String get repeatTitle => 'Répéter';

  @override
  String get repeatKindDaily => 'Chaque jour';

  @override
  String get repeatKindWeekly => 'Jours choisis';

  @override
  String get repeatKindInterval => 'Toutes les N minutes';

  @override
  String get repeatKindOnce => 'Une fois';

  @override
  String get repeatTimeLabel => 'À';

  @override
  String get repeatDateLabel => 'Le';

  @override
  String get repeatPickDate => 'Choisir une date';

  @override
  String repeatDateValue(String day, String month) {
    return '$day/$month';
  }

  @override
  String get repeatEveryHint => 'Toutes les';

  @override
  String get repeatEverySuffix => 'minutes';

  @override
  String get repeatWindowLabel => 'Entre';

  @override
  String get repeatLabelDaily => 'Chaque jour';

  @override
  String get repeatLabelWeekdays => 'En semaine';

  @override
  String get repeatLabelWeekends => 'Le week-end';

  @override
  String repeatLabelWithin(String repeat, String from, String to) {
    return '$repeat · $from–$to';
  }

  @override
  String repeatLabelOnce(String date) {
    return 'Une fois · $date';
  }

  @override
  String get dayMonShort => 'L';

  @override
  String get dayTueShort => 'M';

  @override
  String get dayWedShort => 'M';

  @override
  String get dayThuShort => 'J';

  @override
  String get dayFriShort => 'V';

  @override
  String get daySatShort => 'S';

  @override
  String get daySunShort => 'D';

  @override
  String get dayMon => 'Lundi';

  @override
  String get dayTue => 'Mardi';

  @override
  String get dayWed => 'Mercredi';

  @override
  String get dayThu => 'Jeudi';

  @override
  String get dayFri => 'Vendredi';

  @override
  String get daySat => 'Samedi';

  @override
  String get daySun => 'Dimanche';

  @override
  String get dayMonAbbr => 'Lun';

  @override
  String get dayTueAbbr => 'Mar';

  @override
  String get dayWedAbbr => 'Mer';

  @override
  String get dayThuAbbr => 'Jeu';

  @override
  String get dayFriAbbr => 'Ven';

  @override
  String get daySatAbbr => 'Sam';

  @override
  String get daySunAbbr => 'Dim';

  @override
  String get quietTitle => 'Heures calmes';

  @override
  String get quietHelp => 'Aucun message entre ces heures.';

  @override
  String get formErrorNoVoice => 'Choisissez d’abord une voix.';

  @override
  String get formErrorNoTheme => 'Choisissez un thème.';

  @override
  String get formErrorNoDays => 'Choisissez au moins un jour.';

  @override
  String get formErrorInterval =>
      'Indiquez le nombre de minutes entre les messages (1 ou plus).';

  @override
  String get formErrorNoDate => 'Choisissez la date.';

  @override
  String get voicesTitle => 'Voix';

  @override
  String get voicesEmpty =>
      'Aucune voix pour l’instant.\nTouchez + pour ajouter la voix d’un être cher.';

  @override
  String get voicesAdd => 'Ajouter une voix';

  @override
  String get voiceStatusReady => 'Voix prête';

  @override
  String get voiceStatusNotCreated => 'Voix pas encore créée';

  @override
  String samplesCountAndSecs(String recordings, String duration) {
    return '$recordings · $duration';
  }

  @override
  String get importVoiceTitleNew => 'Nouvelle voix';

  @override
  String get importVoiceTitleEdit => 'Voix';

  @override
  String get voiceNameHint => 'Nom (ex. : Mamie Lucie)';

  @override
  String get voiceNameRequired => 'Saisissez le nom.';

  @override
  String get voiceConsentHint => 'Qui a autorisé l’utilisation de cette voix';

  @override
  String get voiceConsentHelp =>
      'Obligatoire : n’utilisez une voix qu’avec l’accord de la personne.';

  @override
  String get voiceConsentRequired =>
      'Indiquez qui a autorisé l’utilisation de cette voix.';

  @override
  String get voiceLanguageHint => 'Langue';

  @override
  String get voiceListenerHint => 'Comment cette voix vous appelle';

  @override
  String get voiceListenerHelp =>
      'Facultatif, ex. : un surnom. Vide : votre nom dans Réglages.';

  @override
  String get voiceRegistered =>
      'Voix enregistrée. Ajoutez maintenant des enregistrements.';

  @override
  String get voiceSaved => 'Enregistré.';

  @override
  String get voiceCreated => 'Voix créée ! Elle est prête pour vos rappels.';

  @override
  String get samplesTitle => 'Enregistrements';

  @override
  String get samplesAddFiles => 'Ajouter des fichiers audio';

  @override
  String get samplesRecord => 'Enregistrer';

  @override
  String get samplesStop => 'Arrêter';

  @override
  String samplesAdded(int count) {
    return 'Enregistrements ajoutés : $count';
  }

  @override
  String sampleTitle(int number) {
    return 'Enregistrement $number';
  }

  @override
  String sampleDuration(String secs) {
    return '$secs s';
  }

  @override
  String recordingStatus(String elapsed, String limit) {
    return 'Enregistrement… $elapsed sur $limit';
  }

  @override
  String get recordPermissionDenied =>
      'Autorisez le micro pour enregistrer une voix.';

  @override
  String get actionCancel => 'Annuler';

  @override
  String get deleteVoice => 'Supprimer la voix';

  @override
  String get deleteVoiceTitle => 'Supprimer cette voix ?';

  @override
  String get deleteVoiceMessage =>
      'Ses enregistrements et la voix créée sont effacés de ce téléphone et du serveur vocal. Les rappels qui l’utilisent sont désactivés et conservés, pour que vous puissiez choisir une autre voix.';

  @override
  String get deleteVoiceConfirm => 'Supprimer';

  @override
  String get voiceDeleted => 'Voix supprimée.';

  @override
  String get createVoice => 'Créer la voix';

  @override
  String get createVoiceRunning => 'Création de la voix…';

  @override
  String get createVoiceHint =>
      'Ajoutez au moins 10 secondes d’enregistrements (WAV, MP3 ou FLAC, de 2 s à 3 min chacun), puis créez la voix.';

  @override
  String get createVoiceHintReady =>
      'La voix est prête. Vous avez ajouté des enregistrements ? Recréez-la pour l’améliorer.';

  @override
  String get settingsTitle => 'Réglages';

  @override
  String get settingsListenerHint => 'Votre prénom';

  @override
  String get settingsListenerHelp =>
      'Utilisé dans les messages quand une voix n’a pas de surnom pour vous.';

  @override
  String get settingsEngineHint => 'URL du serveur vocal';

  @override
  String get settingsSaved => 'Réglages enregistrés.';

  @override
  String get settingsLanguageHint => 'Langue de l’app';

  @override
  String get languageSystemDefault => 'Langue du système';

  @override
  String get channelName => 'Messages programmés';

  @override
  String get notificationTitle => 'Un message pour vous';

  @override
  String notificationBody(String text, String tone) {
    return '$text\n(ton : $tone)';
  }

  @override
  String pitchHigher(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: 'Hauteur : $count demi-tons plus aigu',
      many: 'Hauteur : $count demi-tons plus aigu',
      one: 'Hauteur : $count demi-ton plus aigu',
    );
    return '$_temp0';
  }

  @override
  String pitchLower(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: 'Hauteur : $count demi-tons plus grave',
      many: 'Hauteur : $count demi-tons plus grave',
      one: 'Hauteur : $count demi-ton plus grave',
    );
    return '$_temp0';
  }

  @override
  String repeatLabelEvery(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: 'Toutes les $count min',
      many: 'Toutes les $count min',
      one: 'Chaque minute',
    );
    return '$_temp0';
  }

  @override
  String samplesCount(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: '$count enregistrements',
      many: '$count enregistrements',
      one: '$count enregistrement',
    );
    return '$_temp0';
  }

  @override
  String seconds(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: '$count secondes',
      many: '$count secondes',
      one: '$count seconde',
    );
    return '$_temp0';
  }

  @override
  String get deleteReminder => 'Supprimer le rappel';

  @override
  String get samplesFillFirst =>
      'Indiquez le nom et qui a donné l’autorisation, puis ajoutez des enregistrements.';
}
