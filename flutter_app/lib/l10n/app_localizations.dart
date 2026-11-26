import 'dart:async';

import 'package:flutter/foundation.dart';
import 'package:flutter/widgets.dart';
import 'package:flutter_localizations/flutter_localizations.dart';
import 'package:intl/intl.dart' as intl;

import 'app_localizations_ar.dart';
import 'app_localizations_de.dart';
import 'app_localizations_en.dart';
import 'app_localizations_es.dart';
import 'app_localizations_fi.dart';
import 'app_localizations_fr.dart';
import 'app_localizations_it.dart';
import 'app_localizations_ja.dart';
import 'app_localizations_nb.dart';
import 'app_localizations_no.dart';
import 'app_localizations_pt.dart';
import 'app_localizations_ru.dart';
import 'app_localizations_tr.dart';
import 'app_localizations_uk.dart';
import 'app_localizations_zh.dart';

// ignore_for_file: type=lint

/// Callers can lookup localized strings with an instance of AppLocalizations
/// returned by `AppLocalizations.of(context)`.
///
/// Applications need to include `AppLocalizations.delegate()` in their app's
/// `localizationDelegates` list, and the locales they support in the app's
/// `supportedLocales` list. For example:
///
/// ```dart
/// import 'l10n/app_localizations.dart';
///
/// return MaterialApp(
///   localizationsDelegates: AppLocalizations.localizationsDelegates,
///   supportedLocales: AppLocalizations.supportedLocales,
///   home: MyApplicationHome(),
/// );
/// ```
///
/// ## Update pubspec.yaml
///
/// Please make sure to update your pubspec.yaml to include the following
/// packages:
///
/// ```yaml
/// dependencies:
///   # Internationalization support.
///   flutter_localizations:
///     sdk: flutter
///   intl: any # Use the pinned version from flutter_localizations
///
///   # Rest of dependencies
/// ```
///
/// ## iOS Applications
///
/// iOS applications define key application metadata, including supported
/// locales, in an Info.plist file that is built into the application bundle.
/// To configure the locales supported by your app, you’ll need to edit this
/// file.
///
/// First, open your project’s ios/Runner.xcworkspace Xcode workspace file.
/// Then, in the Project Navigator, open the Info.plist file under the Runner
/// project’s Runner folder.
///
/// Next, select the Information Property List item, select Add Item from the
/// Editor menu, then select Localizations from the pop-up menu.
///
/// Select and expand the newly-created Localizations item then, for each
/// locale your application supports, add a new item and select the locale
/// you wish to add from the pop-up menu in the Value field. This list should
/// be consistent with the languages listed in the AppLocalizations.supportedLocales
/// property.
abstract class AppLocalizations {
  AppLocalizations(String locale)
    : localeName = intl.Intl.canonicalizedLocale(locale.toString());

  final String localeName;

  static AppLocalizations of(BuildContext context) {
    return Localizations.of<AppLocalizations>(context, AppLocalizations)!;
  }

  static const LocalizationsDelegate<AppLocalizations> delegate =
      _AppLocalizationsDelegate();

  /// A list of this localizations delegate along with the default localizations
  /// delegates.
  ///
  /// Returns a list of localizations delegates containing this delegate along with
  /// GlobalMaterialLocalizations.delegate, GlobalCupertinoLocalizations.delegate,
  /// and GlobalWidgetsLocalizations.delegate.
  ///
  /// Additional delegates can be added by appending to this list in
  /// MaterialApp. This list does not have to be used at all if a custom list
  /// of delegates is preferred or required.
  static const List<LocalizationsDelegate<dynamic>> localizationsDelegates =
      <LocalizationsDelegate<dynamic>>[
        delegate,
        GlobalMaterialLocalizations.delegate,
        GlobalCupertinoLocalizations.delegate,
        GlobalWidgetsLocalizations.delegate,
      ];

  /// A list of this localizations delegate's supported locales.
  static const List<Locale> supportedLocales = <Locale>[
    Locale('ar'),
    Locale('de'),
    Locale('en'),
    Locale('en', 'AU'),
    Locale('en', 'GB'),
    Locale('en', 'NZ'),
    Locale('es'),
    Locale('es', 'AR'),
    Locale('es', 'MX'),
    Locale('fi'),
    Locale('fr'),
    Locale('it'),
    Locale('ja'),
    Locale('nb'),
    Locale('no'),
    Locale('pt'),
    Locale('pt', 'PT'),
    Locale('ru'),
    Locale('tr'),
    Locale('uk'),
    Locale('zh'),
    Locale('zh', 'TW'),
  ];

  /// No description provided for @appName.
  ///
  /// In en, this message translates to:
  /// **'Friendly Voice'**
  String get appName;

  /// No description provided for @actionBack.
  ///
  /// In en, this message translates to:
  /// **'Back'**
  String get actionBack;

  /// No description provided for @errorGeneric.
  ///
  /// In en, this message translates to:
  /// **'Something went wrong. Please try again.'**
  String get errorGeneric;

  /// No description provided for @errorPlayback.
  ///
  /// In en, this message translates to:
  /// **'Could not play the message.'**
  String get errorPlayback;

  /// No description provided for @errorFileOpen.
  ///
  /// In en, this message translates to:
  /// **'Could not open the file.'**
  String get errorFileOpen;

  /// No description provided for @errorNoAudioTrack.
  ///
  /// In en, this message translates to:
  /// **'This file has no audio. Use a WAV, MP3 or FLAC file.'**
  String get errorNoAudioTrack;

  /// No description provided for @errorAudioFormat.
  ///
  /// In en, this message translates to:
  /// **'This audio format is not supported. Convert the file to WAV.'**
  String get errorAudioFormat;

  /// No description provided for @errorAudioTooLong.
  ///
  /// In en, this message translates to:
  /// **'This recording is longer than 3 minutes: split it into pieces.'**
  String get errorAudioTooLong;

  /// No description provided for @errorMicUnavailable.
  ///
  /// In en, this message translates to:
  /// **'The microphone is not available.'**
  String get errorMicUnavailable;

  /// No description provided for @errorRecordingFailed.
  ///
  /// In en, this message translates to:
  /// **'The recording failed. Please try again.'**
  String get errorRecordingFailed;

  /// No description provided for @rangeSeparator.
  ///
  /// In en, this message translates to:
  /// **'–'**
  String get rangeSeparator;

  /// No description provided for @clockFormat.
  ///
  /// In en, this message translates to:
  /// **'{minutes}:{seconds}'**
  String clockFormat(int minutes, String seconds);

  /// No description provided for @tabReminders.
  ///
  /// In en, this message translates to:
  /// **'Reminders'**
  String get tabReminders;

  /// No description provided for @tabVoices.
  ///
  /// In en, this message translates to:
  /// **'Voices'**
  String get tabVoices;

  /// No description provided for @tabSettings.
  ///
  /// In en, this message translates to:
  /// **'Settings'**
  String get tabSettings;

  /// No description provided for @remindersTitle.
  ///
  /// In en, this message translates to:
  /// **'Reminders'**
  String get remindersTitle;

  /// No description provided for @remindersEmpty.
  ///
  /// In en, this message translates to:
  /// **'No reminders yet.\nTap + to hear someone you love at the right time.'**
  String get remindersEmpty;

  /// No description provided for @remindersAdd.
  ///
  /// In en, this message translates to:
  /// **'Add reminder'**
  String get remindersAdd;

  /// No description provided for @reminderSwitchDescription.
  ///
  /// In en, this message translates to:
  /// **'Reminder on or off'**
  String get reminderSwitchDescription;

  /// No description provided for @reminderTimeInterval.
  ///
  /// In en, this message translates to:
  /// **'{minutes} min'**
  String reminderTimeInterval(int minutes);

  /// No description provided for @reminderThemeLine.
  ///
  /// In en, this message translates to:
  /// **'{theme} · {tone}'**
  String reminderThemeLine(String theme, String tone);

  /// No description provided for @reminderOwnText.
  ///
  /// In en, this message translates to:
  /// **'Your own text'**
  String get reminderOwnText;

  /// No description provided for @reminderVoiceUnavailable.
  ///
  /// In en, this message translates to:
  /// **'Voice unavailable — tap to choose another'**
  String get reminderVoiceUnavailable;

  /// No description provided for @editReminderTitleNew.
  ///
  /// In en, this message translates to:
  /// **'New reminder'**
  String get editReminderTitleNew;

  /// No description provided for @editReminderTitleEdit.
  ///
  /// In en, this message translates to:
  /// **'Edit reminder'**
  String get editReminderTitleEdit;

  /// No description provided for @reminderVoiceHint.
  ///
  /// In en, this message translates to:
  /// **'Voice'**
  String get reminderVoiceHint;

  /// No description provided for @reminderNoVoices.
  ///
  /// In en, this message translates to:
  /// **'No voice is ready yet. Create one in the Voices tab first.'**
  String get reminderNoVoices;

  /// No description provided for @reminderGoToVoices.
  ///
  /// In en, this message translates to:
  /// **'Go to Voices'**
  String get reminderGoToVoices;

  /// No description provided for @reminderThemeHint.
  ///
  /// In en, this message translates to:
  /// **'Theme'**
  String get reminderThemeHint;

  /// No description provided for @reminderToneHint.
  ///
  /// In en, this message translates to:
  /// **'Tone of voice'**
  String get reminderToneHint;

  /// No description provided for @toneSuggested.
  ///
  /// In en, this message translates to:
  /// **'Suggested by the theme'**
  String get toneSuggested;

  /// No description provided for @toneSuggestedWith.
  ///
  /// In en, this message translates to:
  /// **'Suggested by the theme ({tone})'**
  String toneSuggestedWith(String tone);

  /// No description provided for @toneOption.
  ///
  /// In en, this message translates to:
  /// **'{tone} — {description}'**
  String toneOption(String tone, String description);

  /// No description provided for @pitchNatural.
  ///
  /// In en, this message translates to:
  /// **'Pitch: natural'**
  String get pitchNatural;

  /// No description provided for @pitchDescription.
  ///
  /// In en, this message translates to:
  /// **'Pitch shift in semitones'**
  String get pitchDescription;

  /// No description provided for @reminderPlayNow.
  ///
  /// In en, this message translates to:
  /// **'Play now'**
  String get reminderPlayNow;

  /// No description provided for @reminderPlaying.
  ///
  /// In en, this message translates to:
  /// **'Preparing the message…'**
  String get reminderPlaying;

  /// No description provided for @repeatTitle.
  ///
  /// In en, this message translates to:
  /// **'Repeat'**
  String get repeatTitle;

  /// No description provided for @repeatKindDaily.
  ///
  /// In en, this message translates to:
  /// **'Daily'**
  String get repeatKindDaily;

  /// No description provided for @repeatKindWeekly.
  ///
  /// In en, this message translates to:
  /// **'Chosen days'**
  String get repeatKindWeekly;

  /// No description provided for @repeatKindInterval.
  ///
  /// In en, this message translates to:
  /// **'Every N minutes'**
  String get repeatKindInterval;

  /// No description provided for @repeatKindOnce.
  ///
  /// In en, this message translates to:
  /// **'Once'**
  String get repeatKindOnce;

  /// No description provided for @repeatTimeLabel.
  ///
  /// In en, this message translates to:
  /// **'At'**
  String get repeatTimeLabel;

  /// No description provided for @repeatDateLabel.
  ///
  /// In en, this message translates to:
  /// **'On'**
  String get repeatDateLabel;

  /// No description provided for @repeatPickDate.
  ///
  /// In en, this message translates to:
  /// **'Pick a date'**
  String get repeatPickDate;

  /// No description provided for @repeatDateValue.
  ///
  /// In en, this message translates to:
  /// **'{day}/{month}'**
  String repeatDateValue(String day, String month);

  /// No description provided for @repeatEveryHint.
  ///
  /// In en, this message translates to:
  /// **'Every'**
  String get repeatEveryHint;

  /// No description provided for @repeatEverySuffix.
  ///
  /// In en, this message translates to:
  /// **'minutes'**
  String get repeatEverySuffix;

  /// No description provided for @repeatWindowLabel.
  ///
  /// In en, this message translates to:
  /// **'Between'**
  String get repeatWindowLabel;

  /// No description provided for @repeatLabelDaily.
  ///
  /// In en, this message translates to:
  /// **'Daily'**
  String get repeatLabelDaily;

  /// No description provided for @repeatLabelWeekdays.
  ///
  /// In en, this message translates to:
  /// **'Weekdays'**
  String get repeatLabelWeekdays;

  /// No description provided for @repeatLabelWeekends.
  ///
  /// In en, this message translates to:
  /// **'Weekends'**
  String get repeatLabelWeekends;

  /// No description provided for @repeatLabelWithin.
  ///
  /// In en, this message translates to:
  /// **'{repeat} · {from}–{to}'**
  String repeatLabelWithin(String repeat, String from, String to);

  /// No description provided for @repeatLabelOnce.
  ///
  /// In en, this message translates to:
  /// **'Once · {date}'**
  String repeatLabelOnce(String date);

  /// No description provided for @dayMonShort.
  ///
  /// In en, this message translates to:
  /// **'M'**
  String get dayMonShort;

  /// No description provided for @dayTueShort.
  ///
  /// In en, this message translates to:
  /// **'T'**
  String get dayTueShort;

  /// No description provided for @dayWedShort.
  ///
  /// In en, this message translates to:
  /// **'W'**
  String get dayWedShort;

  /// No description provided for @dayThuShort.
  ///
  /// In en, this message translates to:
  /// **'T'**
  String get dayThuShort;

  /// No description provided for @dayFriShort.
  ///
  /// In en, this message translates to:
  /// **'F'**
  String get dayFriShort;

  /// No description provided for @daySatShort.
  ///
  /// In en, this message translates to:
  /// **'S'**
  String get daySatShort;

  /// No description provided for @daySunShort.
  ///
  /// In en, this message translates to:
  /// **'S'**
  String get daySunShort;

  /// No description provided for @dayMon.
  ///
  /// In en, this message translates to:
  /// **'Monday'**
  String get dayMon;

  /// No description provided for @dayTue.
  ///
  /// In en, this message translates to:
  /// **'Tuesday'**
  String get dayTue;

  /// No description provided for @dayWed.
  ///
  /// In en, this message translates to:
  /// **'Wednesday'**
  String get dayWed;

  /// No description provided for @dayThu.
  ///
  /// In en, this message translates to:
  /// **'Thursday'**
  String get dayThu;

  /// No description provided for @dayFri.
  ///
  /// In en, this message translates to:
  /// **'Friday'**
  String get dayFri;

  /// No description provided for @daySat.
  ///
  /// In en, this message translates to:
  /// **'Saturday'**
  String get daySat;

  /// No description provided for @daySun.
  ///
  /// In en, this message translates to:
  /// **'Sunday'**
  String get daySun;

  /// No description provided for @dayMonAbbr.
  ///
  /// In en, this message translates to:
  /// **'Mon'**
  String get dayMonAbbr;

  /// No description provided for @dayTueAbbr.
  ///
  /// In en, this message translates to:
  /// **'Tue'**
  String get dayTueAbbr;

  /// No description provided for @dayWedAbbr.
  ///
  /// In en, this message translates to:
  /// **'Wed'**
  String get dayWedAbbr;

  /// No description provided for @dayThuAbbr.
  ///
  /// In en, this message translates to:
  /// **'Thu'**
  String get dayThuAbbr;

  /// No description provided for @dayFriAbbr.
  ///
  /// In en, this message translates to:
  /// **'Fri'**
  String get dayFriAbbr;

  /// No description provided for @daySatAbbr.
  ///
  /// In en, this message translates to:
  /// **'Sat'**
  String get daySatAbbr;

  /// No description provided for @daySunAbbr.
  ///
  /// In en, this message translates to:
  /// **'Sun'**
  String get daySunAbbr;

  /// No description provided for @quietTitle.
  ///
  /// In en, this message translates to:
  /// **'Quiet hours'**
  String get quietTitle;

  /// No description provided for @quietHelp.
  ///
  /// In en, this message translates to:
  /// **'No messages between these times.'**
  String get quietHelp;

  /// No description provided for @formErrorNoVoice.
  ///
  /// In en, this message translates to:
  /// **'Choose a voice first.'**
  String get formErrorNoVoice;

  /// No description provided for @formErrorNoTheme.
  ///
  /// In en, this message translates to:
  /// **'Choose a theme.'**
  String get formErrorNoTheme;

  /// No description provided for @formErrorNoDays.
  ///
  /// In en, this message translates to:
  /// **'Choose at least one day.'**
  String get formErrorNoDays;

  /// No description provided for @formErrorInterval.
  ///
  /// In en, this message translates to:
  /// **'Enter how many minutes between messages (1 or more).'**
  String get formErrorInterval;

  /// No description provided for @formErrorNoDate.
  ///
  /// In en, this message translates to:
  /// **'Pick the date.'**
  String get formErrorNoDate;

  /// No description provided for @voicesTitle.
  ///
  /// In en, this message translates to:
  /// **'Voices'**
  String get voicesTitle;

  /// No description provided for @voicesEmpty.
  ///
  /// In en, this message translates to:
  /// **'No voices yet.\nTap + to add the voice of someone you love.'**
  String get voicesEmpty;

  /// No description provided for @voicesAdd.
  ///
  /// In en, this message translates to:
  /// **'Add voice'**
  String get voicesAdd;

  /// No description provided for @voiceStatusReady.
  ///
  /// In en, this message translates to:
  /// **'Voice ready'**
  String get voiceStatusReady;

  /// No description provided for @voiceStatusNotCreated.
  ///
  /// In en, this message translates to:
  /// **'Voice not created yet'**
  String get voiceStatusNotCreated;

  /// No description provided for @samplesCountAndSecs.
  ///
  /// In en, this message translates to:
  /// **'{recordings} · {duration}'**
  String samplesCountAndSecs(String recordings, String duration);

  /// No description provided for @importVoiceTitleNew.
  ///
  /// In en, this message translates to:
  /// **'New voice'**
  String get importVoiceTitleNew;

  /// No description provided for @importVoiceTitleEdit.
  ///
  /// In en, this message translates to:
  /// **'Voice'**
  String get importVoiceTitleEdit;

  /// No description provided for @voiceNameHint.
  ///
  /// In en, this message translates to:
  /// **'Name (e.g. Grandma Lucia)'**
  String get voiceNameHint;

  /// No description provided for @voiceNameRequired.
  ///
  /// In en, this message translates to:
  /// **'Enter the name.'**
  String get voiceNameRequired;

  /// No description provided for @voiceConsentHint.
  ///
  /// In en, this message translates to:
  /// **'Who authorized the use of this voice'**
  String get voiceConsentHint;

  /// No description provided for @voiceConsentHelp.
  ///
  /// In en, this message translates to:
  /// **'Required: only use a voice with the person\'s permission.'**
  String get voiceConsentHelp;

  /// No description provided for @voiceConsentRequired.
  ///
  /// In en, this message translates to:
  /// **'Enter who authorized the use of this voice.'**
  String get voiceConsentRequired;

  /// No description provided for @voiceLanguageHint.
  ///
  /// In en, this message translates to:
  /// **'Language'**
  String get voiceLanguageHint;

  /// No description provided for @voiceListenerHint.
  ///
  /// In en, this message translates to:
  /// **'How this voice calls you'**
  String get voiceListenerHint;

  /// No description provided for @voiceListenerHelp.
  ///
  /// In en, this message translates to:
  /// **'Optional, e.g. a nickname. Empty: your name from Settings.'**
  String get voiceListenerHelp;

  /// No description provided for @voiceRegistered.
  ///
  /// In en, this message translates to:
  /// **'Voice saved. Now add recordings.'**
  String get voiceRegistered;

  /// No description provided for @voiceSaved.
  ///
  /// In en, this message translates to:
  /// **'Saved.'**
  String get voiceSaved;

  /// No description provided for @voiceCreated.
  ///
  /// In en, this message translates to:
  /// **'Voice created! It is ready for your reminders.'**
  String get voiceCreated;

  /// No description provided for @samplesTitle.
  ///
  /// In en, this message translates to:
  /// **'Recordings'**
  String get samplesTitle;

  /// No description provided for @samplesAddFiles.
  ///
  /// In en, this message translates to:
  /// **'Add audio files'**
  String get samplesAddFiles;

  /// No description provided for @samplesRecord.
  ///
  /// In en, this message translates to:
  /// **'Record'**
  String get samplesRecord;

  /// No description provided for @samplesStop.
  ///
  /// In en, this message translates to:
  /// **'Stop recording'**
  String get samplesStop;

  /// No description provided for @samplesAdded.
  ///
  /// In en, this message translates to:
  /// **'Recordings added: {count}'**
  String samplesAdded(int count);

  /// No description provided for @sampleTitle.
  ///
  /// In en, this message translates to:
  /// **'Recording {number}'**
  String sampleTitle(int number);

  /// No description provided for @sampleDuration.
  ///
  /// In en, this message translates to:
  /// **'{secs} s'**
  String sampleDuration(String secs);

  /// No description provided for @recordingStatus.
  ///
  /// In en, this message translates to:
  /// **'Recording… {elapsed} of {limit}'**
  String recordingStatus(String elapsed, String limit);

  /// No description provided for @recordPermissionDenied.
  ///
  /// In en, this message translates to:
  /// **'Allow the microphone to record a voice.'**
  String get recordPermissionDenied;

  /// No description provided for @actionCancel.
  ///
  /// In en, this message translates to:
  /// **'Cancel'**
  String get actionCancel;

  /// No description provided for @deleteVoice.
  ///
  /// In en, this message translates to:
  /// **'Delete voice'**
  String get deleteVoice;

  /// No description provided for @deleteVoiceTitle.
  ///
  /// In en, this message translates to:
  /// **'Delete this voice?'**
  String get deleteVoiceTitle;

  /// No description provided for @deleteVoiceMessage.
  ///
  /// In en, this message translates to:
  /// **'Its recordings and the created voice are deleted from this phone and the voice server. Reminders that use it are turned off and kept, so you can choose another voice.'**
  String get deleteVoiceMessage;

  /// No description provided for @deleteVoiceConfirm.
  ///
  /// In en, this message translates to:
  /// **'Delete'**
  String get deleteVoiceConfirm;

  /// No description provided for @voiceDeleted.
  ///
  /// In en, this message translates to:
  /// **'Voice deleted.'**
  String get voiceDeleted;

  /// No description provided for @createVoice.
  ///
  /// In en, this message translates to:
  /// **'Create voice'**
  String get createVoice;

  /// No description provided for @createVoiceRunning.
  ///
  /// In en, this message translates to:
  /// **'Creating the voice…'**
  String get createVoiceRunning;

  /// No description provided for @createVoiceHint.
  ///
  /// In en, this message translates to:
  /// **'Add at least 10 seconds of recordings (WAV, MP3 or FLAC, 2 s to 3 min each), then create the voice.'**
  String get createVoiceHint;

  /// No description provided for @createVoiceHintReady.
  ///
  /// In en, this message translates to:
  /// **'The voice is ready. Added more recordings? Create it again to improve it.'**
  String get createVoiceHintReady;

  /// No description provided for @settingsTitle.
  ///
  /// In en, this message translates to:
  /// **'Settings'**
  String get settingsTitle;

  /// No description provided for @settingsListenerHint.
  ///
  /// In en, this message translates to:
  /// **'Your name'**
  String get settingsListenerHint;

  /// No description provided for @settingsListenerHelp.
  ///
  /// In en, this message translates to:
  /// **'Used in the messages when a voice has no nickname for you.'**
  String get settingsListenerHelp;

  /// No description provided for @settingsEngineHint.
  ///
  /// In en, this message translates to:
  /// **'Voice server URL'**
  String get settingsEngineHint;

  /// No description provided for @settingsSaved.
  ///
  /// In en, this message translates to:
  /// **'Settings saved.'**
  String get settingsSaved;

  /// No description provided for @settingsLanguageHint.
  ///
  /// In en, this message translates to:
  /// **'App language'**
  String get settingsLanguageHint;

  /// No description provided for @languageSystemDefault.
  ///
  /// In en, this message translates to:
  /// **'System default'**
  String get languageSystemDefault;

  /// No description provided for @channelName.
  ///
  /// In en, this message translates to:
  /// **'Scheduled messages'**
  String get channelName;

  /// No description provided for @notificationTitle.
  ///
  /// In en, this message translates to:
  /// **'A message for you'**
  String get notificationTitle;

  /// No description provided for @notificationBody.
  ///
  /// In en, this message translates to:
  /// **'{text}\n(tone: {tone})'**
  String notificationBody(String text, String tone);

  /// No description provided for @pitchHigher.
  ///
  /// In en, this message translates to:
  /// **'{count, plural, one{Pitch: {count} semitone higher} other{Pitch: {count} semitones higher}}'**
  String pitchHigher(int count);

  /// No description provided for @pitchLower.
  ///
  /// In en, this message translates to:
  /// **'{count, plural, one{Pitch: {count} semitone lower} other{Pitch: {count} semitones lower}}'**
  String pitchLower(int count);

  /// No description provided for @repeatLabelEvery.
  ///
  /// In en, this message translates to:
  /// **'{count, plural, one{Every minute} other{Every {count} min}}'**
  String repeatLabelEvery(int count);

  /// No description provided for @samplesCount.
  ///
  /// In en, this message translates to:
  /// **'{count, plural, one{{count} recording} other{{count} recordings}}'**
  String samplesCount(int count);

  /// No description provided for @seconds.
  ///
  /// In en, this message translates to:
  /// **'{count, plural, one{{count} second} other{{count} seconds}}'**
  String seconds(int count);

  /// No description provided for @deleteReminder.
  ///
  /// In en, this message translates to:
  /// **'Delete reminder'**
  String get deleteReminder;

  /// No description provided for @samplesFillFirst.
  ///
  /// In en, this message translates to:
  /// **'Fill in the name and who authorized it, then add recordings.'**
  String get samplesFillFirst;
}

class _AppLocalizationsDelegate
    extends LocalizationsDelegate<AppLocalizations> {
  const _AppLocalizationsDelegate();

  @override
  Future<AppLocalizations> load(Locale locale) {
    return SynchronousFuture<AppLocalizations>(lookupAppLocalizations(locale));
  }

  @override
  bool isSupported(Locale locale) => <String>[
    'ar',
    'de',
    'en',
    'es',
    'fi',
    'fr',
    'it',
    'ja',
    'nb',
    'no',
    'pt',
    'ru',
    'tr',
    'uk',
    'zh',
  ].contains(locale.languageCode);

  @override
  bool shouldReload(_AppLocalizationsDelegate old) => false;
}

AppLocalizations lookupAppLocalizations(Locale locale) {
  // Lookup logic when language+country codes are specified.
  switch (locale.languageCode) {
    case 'en':
      {
        switch (locale.countryCode) {
          case 'AU':
            return AppLocalizationsEnAu();
          case 'GB':
            return AppLocalizationsEnGb();
          case 'NZ':
            return AppLocalizationsEnNz();
        }
        break;
      }
    case 'es':
      {
        switch (locale.countryCode) {
          case 'AR':
            return AppLocalizationsEsAr();
          case 'MX':
            return AppLocalizationsEsMx();
        }
        break;
      }
    case 'pt':
      {
        switch (locale.countryCode) {
          case 'PT':
            return AppLocalizationsPtPt();
        }
        break;
      }
    case 'zh':
      {
        switch (locale.countryCode) {
          case 'TW':
            return AppLocalizationsZhTw();
        }
        break;
      }
  }

  // Lookup logic when only language code is specified.
  switch (locale.languageCode) {
    case 'ar':
      return AppLocalizationsAr();
    case 'de':
      return AppLocalizationsDe();
    case 'en':
      return AppLocalizationsEn();
    case 'es':
      return AppLocalizationsEs();
    case 'fi':
      return AppLocalizationsFi();
    case 'fr':
      return AppLocalizationsFr();
    case 'it':
      return AppLocalizationsIt();
    case 'ja':
      return AppLocalizationsJa();
    case 'nb':
      return AppLocalizationsNb();
    case 'no':
      return AppLocalizationsNo();
    case 'pt':
      return AppLocalizationsPt();
    case 'ru':
      return AppLocalizationsRu();
    case 'tr':
      return AppLocalizationsTr();
    case 'uk':
      return AppLocalizationsUk();
    case 'zh':
      return AppLocalizationsZh();
  }

  throw FlutterError(
    'AppLocalizations.delegate failed to load unsupported locale "$locale". This is likely '
    'an issue with the localizations generation tool. Please file an issue '
    'on GitHub with a reproducible sample app and the gen-l10n configuration '
    'that was used.',
  );
}
