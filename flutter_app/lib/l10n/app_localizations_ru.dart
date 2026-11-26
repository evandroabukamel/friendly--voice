// ignore: unused_import
import 'package:intl/intl.dart' as intl;

import 'app_localizations.dart';

// ignore_for_file: type=lint

/// The translations for Russian (`ru`).
class AppLocalizationsRu extends AppLocalizations {
  AppLocalizationsRu([String locale = 'ru']) : super(locale);

  @override
  String get appName => 'Friendly Voice';

  @override
  String get actionBack => 'Назад';

  @override
  String get errorGeneric => 'Что-то пошло не так. Попробуйте ещё раз.';

  @override
  String get errorPlayback => 'Не удалось воспроизвести сообщение.';

  @override
  String get errorFileOpen => 'Не удалось открыть файл.';

  @override
  String get errorNoAudioTrack =>
      'В этом файле нет звука. Используйте файл WAV, MP3 или FLAC.';

  @override
  String get errorAudioFormat =>
      'Этот формат аудио не поддерживается. Преобразуйте файл в WAV.';

  @override
  String get errorAudioTooLong =>
      'Запись длиннее 3 минут: разделите её на части.';

  @override
  String get errorMicUnavailable => 'Микрофон недоступен.';

  @override
  String get errorRecordingFailed => 'Не удалось записать. Попробуйте ещё раз.';

  @override
  String get rangeSeparator => '–';

  @override
  String clockFormat(int minutes, String seconds) {
    return '$minutes:$seconds';
  }

  @override
  String get tabReminders => 'Напоминания';

  @override
  String get tabVoices => 'Голоса';

  @override
  String get tabSettings => 'Настройки';

  @override
  String get remindersTitle => 'Напоминания';

  @override
  String get remindersEmpty =>
      'Напоминаний пока нет.\nНажмите +, чтобы слышать близкого человека в нужное время.';

  @override
  String get remindersAdd => 'Добавить напоминание';

  @override
  String get reminderSwitchDescription => 'Напоминание включено или выключено';

  @override
  String reminderTimeInterval(int minutes) {
    return '$minutes мин';
  }

  @override
  String reminderThemeLine(String theme, String tone) {
    return '$theme · $tone';
  }

  @override
  String get reminderOwnText => 'Свой текст';

  @override
  String get reminderVoiceUnavailable =>
      'Голос недоступен — нажмите, чтобы выбрать другой';

  @override
  String get editReminderTitleNew => 'Новое напоминание';

  @override
  String get editReminderTitleEdit => 'Изменить напоминание';

  @override
  String get reminderVoiceHint => 'Голос';

  @override
  String get reminderNoVoices =>
      'Готовых голосов пока нет. Сначала создайте голос на вкладке «Голоса».';

  @override
  String get reminderGoToVoices => 'К голосам';

  @override
  String get reminderThemeHint => 'Тема';

  @override
  String get reminderToneHint => 'Тон голоса';

  @override
  String get toneSuggested => 'По теме';

  @override
  String toneSuggestedWith(String tone) {
    return 'По теме ($tone)';
  }

  @override
  String toneOption(String tone, String description) {
    return '$tone — $description';
  }

  @override
  String get pitchNatural => 'Высота: естественная';

  @override
  String get pitchDescription => 'Сдвиг высоты в полутонах';

  @override
  String get reminderPlayNow => 'Прослушать';

  @override
  String get reminderPlaying => 'Готовим сообщение…';

  @override
  String get repeatTitle => 'Повтор';

  @override
  String get repeatKindDaily => 'Каждый день';

  @override
  String get repeatKindWeekly => 'По дням';

  @override
  String get repeatKindInterval => 'Каждые N минут';

  @override
  String get repeatKindOnce => 'Один раз';

  @override
  String get repeatTimeLabel => 'Время';

  @override
  String get repeatDateLabel => 'Дата';

  @override
  String get repeatPickDate => 'Выберите дату';

  @override
  String repeatDateValue(String day, String month) {
    return '$day/$month';
  }

  @override
  String get repeatEveryHint => 'Каждые';

  @override
  String get repeatEverySuffix => 'мин';

  @override
  String get repeatWindowLabel => 'С и до';

  @override
  String get repeatLabelDaily => 'Каждый день';

  @override
  String get repeatLabelWeekdays => 'По будням';

  @override
  String get repeatLabelWeekends => 'По выходным';

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
  String get daySunShort => 'В';

  @override
  String get dayMon => 'Понедельник';

  @override
  String get dayTue => 'Вторник';

  @override
  String get dayWed => 'Среда';

  @override
  String get dayThu => 'Четверг';

  @override
  String get dayFri => 'Пятница';

  @override
  String get daySat => 'Суббота';

  @override
  String get daySun => 'Воскресенье';

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
  String get daySunAbbr => 'Вс';

  @override
  String get quietTitle => 'Тихие часы';

  @override
  String get quietHelp => 'В это время сообщений не будет.';

  @override
  String get formErrorNoVoice => 'Сначала выберите голос.';

  @override
  String get formErrorNoTheme => 'Выберите тему.';

  @override
  String get formErrorNoDays => 'Выберите хотя бы один день.';

  @override
  String get formErrorInterval =>
      'Укажите, сколько минут между сообщениями (1 или больше).';

  @override
  String get formErrorNoDate => 'Выберите дату.';

  @override
  String get voicesTitle => 'Голоса';

  @override
  String get voicesEmpty =>
      'Голосов пока нет.\nНажмите +, чтобы добавить голос близкого человека.';

  @override
  String get voicesAdd => 'Добавить голос';

  @override
  String get voiceStatusReady => 'Голос готов';

  @override
  String get voiceStatusNotCreated => 'Голос ещё не создан';

  @override
  String samplesCountAndSecs(String recordings, String duration) {
    return '$recordings · $duration';
  }

  @override
  String get importVoiceTitleNew => 'Новый голос';

  @override
  String get importVoiceTitleEdit => 'Голос';

  @override
  String get voiceNameHint => 'Имя (например, бабушка Люся)';

  @override
  String get voiceNameRequired => 'Введите имя.';

  @override
  String get voiceConsentHint => 'Кто разрешил использовать этот голос';

  @override
  String get voiceConsentHelp =>
      'Обязательно: используйте голос только с согласия человека.';

  @override
  String get voiceConsentRequired =>
      'Укажите, кто разрешил использовать этот голос.';

  @override
  String get voiceLanguageHint => 'Язык';

  @override
  String get voiceListenerHint => 'Как этот голос вас называет';

  @override
  String get voiceListenerHelp =>
      'Необязательно, например прозвище. Пусто — ваше имя из настроек.';

  @override
  String get voiceRegistered => 'Голос сохранён. Теперь добавьте записи.';

  @override
  String get voiceSaved => 'Сохранено.';

  @override
  String get voiceCreated => 'Голос создан! Он готов для ваших напоминаний.';

  @override
  String get samplesTitle => 'Записи';

  @override
  String get samplesAddFiles => 'Добавить аудиофайлы';

  @override
  String get samplesRecord => 'Записать';

  @override
  String get samplesStop => 'Остановить запись';

  @override
  String samplesAdded(int count) {
    return 'Добавлено записей: $count';
  }

  @override
  String sampleTitle(int number) {
    return 'Запись $number';
  }

  @override
  String sampleDuration(String secs) {
    return '$secs с';
  }

  @override
  String recordingStatus(String elapsed, String limit) {
    return 'Запись… $elapsed из $limit';
  }

  @override
  String get recordPermissionDenied =>
      'Разрешите доступ к микрофону, чтобы записать голос.';

  @override
  String get actionCancel => 'Отмена';

  @override
  String get deleteVoice => 'Удалить голос';

  @override
  String get deleteVoiceTitle => 'Удалить этот голос?';

  @override
  String get deleteVoiceMessage =>
      'Его записи и созданный голос будут удалены с этого телефона и с голосового сервера. Напоминания с этим голосом выключатся и сохранятся, чтобы вы могли выбрать другой голос.';

  @override
  String get deleteVoiceConfirm => 'Удалить';

  @override
  String get voiceDeleted => 'Голос удалён.';

  @override
  String get createVoice => 'Создать голос';

  @override
  String get createVoiceRunning => 'Создаём голос…';

  @override
  String get createVoiceHint =>
      'Добавьте не меньше 10 секунд записей (WAV, MP3 или FLAC, от 2 с до 3 мин каждая), затем создайте голос.';

  @override
  String get createVoiceHintReady =>
      'Голос готов. Добавили новые записи? Создайте его заново, чтобы улучшить.';

  @override
  String get settingsTitle => 'Настройки';

  @override
  String get settingsListenerHint => 'Ваше имя';

  @override
  String get settingsListenerHelp =>
      'Используется в сообщениях, если у голоса нет для вас прозвища.';

  @override
  String get settingsEngineHint => 'Адрес голосового сервера';

  @override
  String get settingsSaved => 'Настройки сохранены.';

  @override
  String get settingsLanguageHint => 'Язык приложения';

  @override
  String get languageSystemDefault => 'Как в системе';

  @override
  String get channelName => 'Запланированные сообщения';

  @override
  String get notificationTitle => 'Сообщение для вас';

  @override
  String notificationBody(String text, String tone) {
    return '$text\n(тон: $tone)';
  }

  @override
  String pitchHigher(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: 'Высота: выше на $count полутона',
      many: 'Высота: выше на $count полутонов',
      few: 'Высота: выше на $count полутона',
      one: 'Высота: выше на $count полутон',
    );
    return '$_temp0';
  }

  @override
  String pitchLower(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: 'Высота: ниже на $count полутона',
      many: 'Высота: ниже на $count полутонов',
      few: 'Высота: ниже на $count полутона',
      one: 'Высота: ниже на $count полутон',
    );
    return '$_temp0';
  }

  @override
  String repeatLabelEvery(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: 'Раз в $count мин',
      many: 'Раз в $count мин',
      few: 'Раз в $count мин',
      one: 'Раз в $count мин',
    );
    return '$_temp0';
  }

  @override
  String samplesCount(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: '$count записи',
      many: '$count записей',
      few: '$count записи',
      one: '$count запись',
    );
    return '$_temp0';
  }

  @override
  String seconds(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: '$count секунды',
      many: '$count секунд',
      few: '$count секунды',
      one: '$count секунда',
    );
    return '$_temp0';
  }

  @override
  String get deleteReminder => 'Удалить напоминание';

  @override
  String get samplesFillFirst =>
      'Введите имя и того, кто дал разрешение, затем добавьте записи.';
}
