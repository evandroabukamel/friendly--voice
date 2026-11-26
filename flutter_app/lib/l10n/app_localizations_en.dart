// ignore: unused_import
import 'package:intl/intl.dart' as intl;

import 'app_localizations.dart';

// ignore_for_file: type=lint

/// The translations for English (`en`).
class AppLocalizationsEn extends AppLocalizations {
  AppLocalizationsEn([String locale = 'en']) : super(locale);

  @override
  String get appName => 'Friendly Voice';

  @override
  String get actionBack => 'Back';

  @override
  String get errorGeneric => 'Something went wrong. Please try again.';

  @override
  String get errorPlayback => 'Could not play the message.';

  @override
  String get errorFileOpen => 'Could not open the file.';

  @override
  String get errorNoAudioTrack =>
      'This file has no audio. Use a WAV, MP3 or FLAC file.';

  @override
  String get errorAudioFormat =>
      'This audio format is not supported. Convert the file to WAV.';

  @override
  String get errorAudioTooLong =>
      'This recording is longer than 3 minutes: split it into pieces.';

  @override
  String get errorMicUnavailable => 'The microphone is not available.';

  @override
  String get errorRecordingFailed => 'The recording failed. Please try again.';

  @override
  String get rangeSeparator => '–';

  @override
  String clockFormat(int minutes, String seconds) {
    return '$minutes:$seconds';
  }

  @override
  String get tabReminders => 'Reminders';

  @override
  String get tabVoices => 'Voices';

  @override
  String get tabSettings => 'Settings';

  @override
  String get remindersTitle => 'Reminders';

  @override
  String get remindersEmpty =>
      'No reminders yet.\nTap + to hear someone you love at the right time.';

  @override
  String get remindersAdd => 'Add reminder';

  @override
  String get reminderSwitchDescription => 'Reminder on or off';

  @override
  String reminderTimeInterval(int minutes) {
    return '$minutes min';
  }

  @override
  String reminderThemeLine(String theme, String tone) {
    return '$theme · $tone';
  }

  @override
  String get reminderOwnText => 'Your own text';

  @override
  String get reminderVoiceUnavailable =>
      'Voice unavailable — tap to choose another';

  @override
  String get editReminderTitleNew => 'New reminder';

  @override
  String get editReminderTitleEdit => 'Edit reminder';

  @override
  String get reminderVoiceHint => 'Voice';

  @override
  String get reminderNoVoices =>
      'No voice is ready yet. Create one in the Voices tab first.';

  @override
  String get reminderGoToVoices => 'Go to Voices';

  @override
  String get reminderThemeHint => 'Theme';

  @override
  String get reminderToneHint => 'Tone of voice';

  @override
  String get toneSuggested => 'Suggested by the theme';

  @override
  String toneSuggestedWith(String tone) {
    return 'Suggested by the theme ($tone)';
  }

  @override
  String toneOption(String tone, String description) {
    return '$tone — $description';
  }

  @override
  String get pitchNatural => 'Pitch: natural';

  @override
  String get pitchDescription => 'Pitch shift in semitones';

  @override
  String get reminderPlayNow => 'Play now';

  @override
  String get reminderPlaying => 'Preparing the message…';

  @override
  String get repeatTitle => 'Repeat';

  @override
  String get repeatKindDaily => 'Daily';

  @override
  String get repeatKindWeekly => 'Chosen days';

  @override
  String get repeatKindInterval => 'Every N minutes';

  @override
  String get repeatKindOnce => 'Once';

  @override
  String get repeatTimeLabel => 'At';

  @override
  String get repeatDateLabel => 'On';

  @override
  String get repeatPickDate => 'Pick a date';

  @override
  String repeatDateValue(String day, String month) {
    return '$day/$month';
  }

  @override
  String get repeatEveryHint => 'Every';

  @override
  String get repeatEverySuffix => 'minutes';

  @override
  String get repeatWindowLabel => 'Between';

  @override
  String get repeatLabelDaily => 'Daily';

  @override
  String get repeatLabelWeekdays => 'Weekdays';

  @override
  String get repeatLabelWeekends => 'Weekends';

  @override
  String repeatLabelWithin(String repeat, String from, String to) {
    return '$repeat · $from–$to';
  }

  @override
  String repeatLabelOnce(String date) {
    return 'Once · $date';
  }

  @override
  String get dayMonShort => 'M';

  @override
  String get dayTueShort => 'T';

  @override
  String get dayWedShort => 'W';

  @override
  String get dayThuShort => 'T';

  @override
  String get dayFriShort => 'F';

  @override
  String get daySatShort => 'S';

  @override
  String get daySunShort => 'S';

  @override
  String get dayMon => 'Monday';

  @override
  String get dayTue => 'Tuesday';

  @override
  String get dayWed => 'Wednesday';

  @override
  String get dayThu => 'Thursday';

  @override
  String get dayFri => 'Friday';

  @override
  String get daySat => 'Saturday';

  @override
  String get daySun => 'Sunday';

  @override
  String get dayMonAbbr => 'Mon';

  @override
  String get dayTueAbbr => 'Tue';

  @override
  String get dayWedAbbr => 'Wed';

  @override
  String get dayThuAbbr => 'Thu';

  @override
  String get dayFriAbbr => 'Fri';

  @override
  String get daySatAbbr => 'Sat';

  @override
  String get daySunAbbr => 'Sun';

  @override
  String get quietTitle => 'Quiet hours';

  @override
  String get quietHelp => 'No messages between these times.';

  @override
  String get formErrorNoVoice => 'Choose a voice first.';

  @override
  String get formErrorNoTheme => 'Choose a theme.';

  @override
  String get formErrorNoDays => 'Choose at least one day.';

  @override
  String get formErrorInterval =>
      'Enter how many minutes between messages (1 or more).';

  @override
  String get formErrorNoDate => 'Pick the date.';

  @override
  String get voicesTitle => 'Voices';

  @override
  String get voicesEmpty =>
      'No voices yet.\nTap + to add the voice of someone you love.';

  @override
  String get voicesAdd => 'Add voice';

  @override
  String get voiceStatusReady => 'Voice ready';

  @override
  String get voiceStatusNotCreated => 'Voice not created yet';

  @override
  String samplesCountAndSecs(String recordings, String duration) {
    return '$recordings · $duration';
  }

  @override
  String get importVoiceTitleNew => 'New voice';

  @override
  String get importVoiceTitleEdit => 'Voice';

  @override
  String get voiceNameHint => 'Name (e.g. Grandma Lucia)';

  @override
  String get voiceNameRequired => 'Enter the name.';

  @override
  String get voiceConsentHint => 'Who authorized the use of this voice';

  @override
  String get voiceConsentHelp =>
      'Required: only use a voice with the person\'s permission.';

  @override
  String get voiceConsentRequired =>
      'Enter who authorized the use of this voice.';

  @override
  String get voiceLanguageHint => 'Language';

  @override
  String get voiceListenerHint => 'How this voice calls you';

  @override
  String get voiceListenerHelp =>
      'Optional, e.g. a nickname. Empty: your name from Settings.';

  @override
  String get voiceRegistered => 'Voice saved. Now add recordings.';

  @override
  String get voiceSaved => 'Saved.';

  @override
  String get voiceCreated => 'Voice created! It is ready for your reminders.';

  @override
  String get samplesTitle => 'Recordings';

  @override
  String get samplesAddFiles => 'Add audio files';

  @override
  String get samplesRecord => 'Record';

  @override
  String get samplesStop => 'Stop recording';

  @override
  String samplesAdded(int count) {
    return 'Recordings added: $count';
  }

  @override
  String sampleTitle(int number) {
    return 'Recording $number';
  }

  @override
  String sampleDuration(String secs) {
    return '$secs s';
  }

  @override
  String recordingStatus(String elapsed, String limit) {
    return 'Recording… $elapsed of $limit';
  }

  @override
  String get recordPermissionDenied =>
      'Allow the microphone to record a voice.';

  @override
  String get actionCancel => 'Cancel';

  @override
  String get deleteVoice => 'Delete voice';

  @override
  String get deleteVoiceTitle => 'Delete this voice?';

  @override
  String get deleteVoiceMessage =>
      'Its recordings and the created voice are deleted from this phone and the voice server. Reminders that use it are turned off and kept, so you can choose another voice.';

  @override
  String get deleteVoiceConfirm => 'Delete';

  @override
  String get voiceDeleted => 'Voice deleted.';

  @override
  String get createVoice => 'Create voice';

  @override
  String get createVoiceRunning => 'Creating the voice…';

  @override
  String get createVoiceHint =>
      'Add at least 10 seconds of recordings (WAV, MP3 or FLAC, 2 s to 3 min each), then create the voice.';

  @override
  String get createVoiceHintReady =>
      'The voice is ready. Added more recordings? Create it again to improve it.';

  @override
  String get settingsTitle => 'Settings';

  @override
  String get settingsListenerHint => 'Your name';

  @override
  String get settingsListenerHelp =>
      'Used in the messages when a voice has no nickname for you.';

  @override
  String get settingsEngineHint => 'Voice server URL';

  @override
  String get settingsSaved => 'Settings saved.';

  @override
  String get settingsLanguageHint => 'App language';

  @override
  String get languageSystemDefault => 'System default';

  @override
  String get channelName => 'Scheduled messages';

  @override
  String get notificationTitle => 'A message for you';

  @override
  String notificationBody(String text, String tone) {
    return '$text\n(tone: $tone)';
  }

  @override
  String pitchHigher(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: 'Pitch: $count semitones higher',
      one: 'Pitch: $count semitone higher',
    );
    return '$_temp0';
  }

  @override
  String pitchLower(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: 'Pitch: $count semitones lower',
      one: 'Pitch: $count semitone lower',
    );
    return '$_temp0';
  }

  @override
  String repeatLabelEvery(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: 'Every $count min',
      one: 'Every minute',
    );
    return '$_temp0';
  }

  @override
  String samplesCount(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: '$count recordings',
      one: '$count recording',
    );
    return '$_temp0';
  }

  @override
  String seconds(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: '$count seconds',
      one: '$count second',
    );
    return '$_temp0';
  }

  @override
  String get deleteReminder => 'Delete reminder';

  @override
  String get samplesFillFirst =>
      'Fill in the name and who authorized it, then add recordings.';
}

/// The translations for English, as used in Australia (`en_AU`).
class AppLocalizationsEnAu extends AppLocalizationsEn {
  AppLocalizationsEnAu() : super('en_AU');

  @override
  String get voiceConsentHint => 'Who authorised the use of this voice';

  @override
  String get voiceConsentRequired =>
      'Enter who authorised the use of this voice.';
}

/// The translations for English, as used in the United Kingdom (`en_GB`).
class AppLocalizationsEnGb extends AppLocalizationsEn {
  AppLocalizationsEnGb() : super('en_GB');

  @override
  String get voiceConsentHint => 'Who authorised the use of this voice';

  @override
  String get voiceConsentRequired =>
      'Enter who authorised the use of this voice.';
}

/// The translations for English, as used in New Zealand (`en_NZ`).
class AppLocalizationsEnNz extends AppLocalizationsEn {
  AppLocalizationsEnNz() : super('en_NZ');

  @override
  String get voiceConsentHint => 'Who authorised the use of this voice';

  @override
  String get voiceConsentRequired =>
      'Enter who authorised the use of this voice.';
}
