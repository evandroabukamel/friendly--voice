// ignore: unused_import
import 'package:intl/intl.dart' as intl;

import 'app_localizations.dart';

// ignore_for_file: type=lint

/// The translations for Ukrainian (`uk`).
class AppLocalizationsUk extends AppLocalizations {
  AppLocalizationsUk([String locale = 'uk']) : super(locale);

  @override
  String get appName => 'Friendly Voice';

  @override
  String get actionBack => 'Назад';

  @override
  String get errorGeneric => 'Щось пішло не так. Спробуйте ще раз.';

  @override
  String get errorPlayback => 'Не вдалося відтворити повідомлення.';

  @override
  String get errorFileOpen => 'Не вдалося відкрити файл.';

  @override
  String get errorNoAudioTrack =>
      'У цьому файлі немає звуку. Використайте файл WAV, MP3 або FLAC.';

  @override
  String get errorAudioFormat =>
      'Цей аудіоформат не підтримується. Перетворіть файл на WAV.';

  @override
  String get errorAudioTooLong =>
      'Запис довший за 3 хвилини: розділіть його на частини.';

  @override
  String get errorMicUnavailable => 'Мікрофон недоступний.';

  @override
  String get errorRecordingFailed => 'Не вдалося записати. Спробуйте ще раз.';

  @override
  String get rangeSeparator => '–';

  @override
  String clockFormat(int minutes, String seconds) {
    return '$minutes:$seconds';
  }

  @override
  String get tabReminders => 'Нагадування';

  @override
  String get tabVoices => 'Голоси';

  @override
  String get tabSettings => 'Налаштування';

  @override
  String get remindersTitle => 'Нагадування';

  @override
  String get remindersEmpty =>
      'Нагадувань поки немає.\nТоркніться +, щоб чути рідну людину в потрібний час.';

  @override
  String get remindersAdd => 'Додати нагадування';

  @override
  String get reminderSwitchDescription => 'Нагадування увімкнено або вимкнено';

  @override
  String reminderTimeInterval(int minutes) {
    return '$minutes хв';
  }

  @override
  String reminderThemeLine(String theme, String tone) {
    return '$theme · $tone';
  }

  @override
  String get reminderOwnText => 'Власний текст';

  @override
  String get reminderVoiceUnavailable =>
      'Голос недоступний — торкніться, щоб вибрати інший';

  @override
  String get editReminderTitleNew => 'Нове нагадування';

  @override
  String get editReminderTitleEdit => 'Змінити нагадування';

  @override
  String get reminderVoiceHint => 'Голос';

  @override
  String get reminderNoVoices =>
      'Готових голосів ще немає. Спершу створіть голос на вкладці «Голоси».';

  @override
  String get reminderGoToVoices => 'До голосів';

  @override
  String get reminderThemeHint => 'Тема';

  @override
  String get reminderToneHint => 'Тон голосу';

  @override
  String get toneSuggested => 'За темою';

  @override
  String toneSuggestedWith(String tone) {
    return 'За темою ($tone)';
  }

  @override
  String toneOption(String tone, String description) {
    return '$tone — $description';
  }

  @override
  String get pitchNatural => 'Висота: природна';

  @override
  String get pitchDescription => 'Зсув висоти в півтонах';

  @override
  String get reminderPlayNow => 'Прослухати';

  @override
  String get reminderPlaying => 'Готуємо повідомлення…';

  @override
  String get repeatTitle => 'Повтор';

  @override
  String get repeatKindDaily => 'Щодня';

  @override
  String get repeatKindWeekly => 'За днями';

  @override
  String get repeatKindInterval => 'Кожні N хвилин';

  @override
  String get repeatKindOnce => 'Один раз';

  @override
  String get repeatTimeLabel => 'Час';

  @override
  String get repeatDateLabel => 'Дата';

  @override
  String get repeatPickDate => 'Виберіть дату';

  @override
  String repeatDateValue(String day, String month) {
    return '$day/$month';
  }

  @override
  String get repeatEveryHint => 'Кожні';

  @override
  String get repeatEverySuffix => 'хв';

  @override
  String get repeatWindowLabel => 'З і до';

  @override
  String get repeatLabelDaily => 'Щодня';

  @override
  String get repeatLabelWeekdays => 'У будні';

  @override
  String get repeatLabelWeekends => 'У вихідні';

  @override
  String repeatLabelWithin(String repeat, String from, String to) {
    return '$repeat · $from–$to';
  }

  @override
  String repeatLabelOnce(String date) {
    return 'Один раз · $date';
  }

  @override
  String get dayMonShort => 'П';

  @override
  String get dayTueShort => 'В';

  @override
  String get dayWedShort => 'С';

  @override
  String get dayThuShort => 'Ч';

  @override
  String get dayFriShort => 'П';

  @override
  String get daySatShort => 'С';

  @override
  String get daySunShort => 'Н';

  @override
  String get dayMon => 'Понеділок';

  @override
  String get dayTue => 'Вівторок';

  @override
  String get dayWed => 'Середа';

  @override
  String get dayThu => 'Четвер';

  @override
  String get dayFri => 'Пʼятниця';

  @override
  String get daySat => 'Субота';

  @override
  String get daySun => 'Неділя';

  @override
  String get dayMonAbbr => 'Пн';

  @override
  String get dayTueAbbr => 'Вт';

  @override
  String get dayWedAbbr => 'Ср';

  @override
  String get dayThuAbbr => 'Чт';

  @override
  String get dayFriAbbr => 'Пт';

  @override
  String get daySatAbbr => 'Сб';

  @override
  String get daySunAbbr => 'Нд';

  @override
  String get quietTitle => 'Тихі години';

  @override
  String get quietHelp => 'У цей час повідомлень не буде.';

  @override
  String get formErrorNoVoice => 'Спершу виберіть голос.';

  @override
  String get formErrorNoTheme => 'Виберіть тему.';

  @override
  String get formErrorNoDays => 'Виберіть хоча б один день.';

  @override
  String get formErrorInterval =>
      'Укажіть, скільки хвилин між повідомленнями (1 або більше).';

  @override
  String get formErrorNoDate => 'Виберіть дату.';

  @override
  String get voicesTitle => 'Голоси';

  @override
  String get voicesEmpty =>
      'Голосів поки немає.\nТоркніться +, щоб додати голос рідної людини.';

  @override
  String get voicesAdd => 'Додати голос';

  @override
  String get voiceStatusReady => 'Голос готовий';

  @override
  String get voiceStatusNotCreated => 'Голос ще не створено';

  @override
  String samplesCountAndSecs(String recordings, String duration) {
    return '$recordings · $duration';
  }

  @override
  String get importVoiceTitleNew => 'Новий голос';

  @override
  String get importVoiceTitleEdit => 'Голос';

  @override
  String get voiceNameHint => 'Імʼя (наприклад, бабуся Люся)';

  @override
  String get voiceNameRequired => 'Введіть імʼя.';

  @override
  String get voiceConsentHint => 'Хто дозволив використовувати цей голос';

  @override
  String get voiceConsentHelp =>
      'Обовʼязково: використовуйте голос лише за згодою людини.';

  @override
  String get voiceConsentRequired =>
      'Укажіть, хто дозволив використовувати цей голос.';

  @override
  String get voiceLanguageHint => 'Мова';

  @override
  String get voiceListenerHint => 'Як цей голос вас називає';

  @override
  String get voiceListenerHelp =>
      'Необовʼязково, наприклад прізвисько. Порожньо — ваше імʼя з налаштувань.';

  @override
  String get voiceRegistered => 'Голос збережено. Тепер додайте записи.';

  @override
  String get voiceSaved => 'Збережено.';

  @override
  String get voiceCreated =>
      'Голос створено! Він готовий для ваших нагадувань.';

  @override
  String get samplesTitle => 'Записи';

  @override
  String get samplesAddFiles => 'Додати аудіофайли';

  @override
  String get samplesRecord => 'Записати';

  @override
  String get samplesStop => 'Зупинити запис';

  @override
  String samplesAdded(int count) {
    return 'Додано записів: $count';
  }

  @override
  String sampleTitle(int number) {
    return 'Запис $number';
  }

  @override
  String sampleDuration(String secs) {
    return '$secs с';
  }

  @override
  String recordingStatus(String elapsed, String limit) {
    return 'Запис… $elapsed з $limit';
  }

  @override
  String get recordPermissionDenied =>
      'Дозвольте доступ до мікрофона, щоб записати голос.';

  @override
  String get actionCancel => 'Скасувати';

  @override
  String get deleteVoice => 'Видалити голос';

  @override
  String get deleteVoiceTitle => 'Видалити цей голос?';

  @override
  String get deleteVoiceMessage =>
      'Його записи та створений голос буде видалено з цього телефона й голосового сервера. Нагадування з цим голосом вимкнуться й збережуться, щоб ви могли вибрати інший голос.';

  @override
  String get deleteVoiceConfirm => 'Видалити';

  @override
  String get voiceDeleted => 'Голос видалено.';

  @override
  String get createVoice => 'Створити голос';

  @override
  String get createVoiceRunning => 'Створюємо голос…';

  @override
  String get createVoiceHint =>
      'Додайте щонайменше 10 секунд записів (WAV, MP3 або FLAC, від 2 с до 3 хв кожен), потім створіть голос.';

  @override
  String get createVoiceHintReady =>
      'Голос готовий. Додали нові записи? Створіть його знову, щоб покращити.';

  @override
  String get settingsTitle => 'Налаштування';

  @override
  String get settingsListenerHint => 'Ваше імʼя';

  @override
  String get settingsListenerHelp =>
      'Використовується в повідомленнях, якщо голос не має для вас прізвиська.';

  @override
  String get settingsEngineHint => 'Адреса голосового сервера';

  @override
  String get settingsSaved => 'Налаштування збережено.';

  @override
  String get settingsLanguageHint => 'Мова застосунку';

  @override
  String get languageSystemDefault => 'Як у системі';

  @override
  String get channelName => 'Заплановані повідомлення';

  @override
  String get notificationTitle => 'Повідомлення для вас';

  @override
  String notificationBody(String text, String tone) {
    return '$text\n(тон: $tone)';
  }

  @override
  String pitchHigher(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: 'Висота: вище на $count півтону',
      many: 'Висота: вище на $count півтонів',
      few: 'Висота: вище на $count півтони',
      one: 'Висота: вище на $count півтон',
    );
    return '$_temp0';
  }

  @override
  String pitchLower(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: 'Висота: нижче на $count півтону',
      many: 'Висота: нижче на $count півтонів',
      few: 'Висота: нижче на $count півтони',
      one: 'Висота: нижче на $count півтон',
    );
    return '$_temp0';
  }

  @override
  String repeatLabelEvery(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: 'Раз на $count хв',
      many: 'Раз на $count хв',
      few: 'Раз на $count хв',
      one: 'Раз на $count хв',
    );
    return '$_temp0';
  }

  @override
  String samplesCount(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: '$count запису',
      many: '$count записів',
      few: '$count записи',
      one: '$count запис',
    );
    return '$_temp0';
  }

  @override
  String seconds(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: '$count секунди',
      many: '$count секунд',
      few: '$count секунди',
      one: '$count секунда',
    );
    return '$_temp0';
  }

  @override
  String get deleteReminder => 'Видалити нагадування';

  @override
  String get samplesFillFirst =>
      'Введіть ім’я та того, хто дав дозвіл, потім додайте записи.';
}
