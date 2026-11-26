// ignore: unused_import
import 'package:intl/intl.dart' as intl;

import 'app_localizations.dart';

// ignore_for_file: type=lint

/// The translations for Chinese (`zh`).
class AppLocalizationsZh extends AppLocalizations {
  AppLocalizationsZh([String locale = 'zh']) : super(locale);

  @override
  String get appName => 'Friendly Voice';

  @override
  String get actionBack => '返回';

  @override
  String get errorGeneric => '出了点问题，请再试一次。';

  @override
  String get errorPlayback => '无法播放这条消息。';

  @override
  String get errorFileOpen => '无法打开文件。';

  @override
  String get errorNoAudioTrack => '此文件没有音频。请使用 WAV、MP3 或 FLAC 文件。';

  @override
  String get errorAudioFormat => '不支持此音频格式。请将文件转换为 WAV。';

  @override
  String get errorAudioTooLong => '这段录音超过 3 分钟：请分成几段。';

  @override
  String get errorMicUnavailable => '麦克风不可用。';

  @override
  String get errorRecordingFailed => '录音失败，请再试一次。';

  @override
  String get rangeSeparator => '–';

  @override
  String clockFormat(int minutes, String seconds) {
    return '$minutes:$seconds';
  }

  @override
  String get tabReminders => '提醒';

  @override
  String get tabVoices => '声音';

  @override
  String get tabSettings => '设置';

  @override
  String get remindersTitle => '提醒';

  @override
  String get remindersEmpty => '还没有提醒。\n点按 + 在合适的时间听到你爱的人的声音。';

  @override
  String get remindersAdd => '添加提醒';

  @override
  String get reminderSwitchDescription => '提醒开或关';

  @override
  String reminderTimeInterval(int minutes) {
    return '$minutes 分钟';
  }

  @override
  String reminderThemeLine(String theme, String tone) {
    return '$theme · $tone';
  }

  @override
  String get reminderOwnText => '自定义文字';

  @override
  String get reminderVoiceUnavailable => '声音不可用 — 点按选择其他声音';

  @override
  String get editReminderTitleNew => '新提醒';

  @override
  String get editReminderTitleEdit => '编辑提醒';

  @override
  String get reminderVoiceHint => '声音';

  @override
  String get reminderNoVoices => '还没有可用的声音。请先在“声音”标签中创建一个。';

  @override
  String get reminderGoToVoices => '前往“声音”';

  @override
  String get reminderThemeHint => '主题';

  @override
  String get reminderToneHint => '语气';

  @override
  String get toneSuggested => '由主题推荐';

  @override
  String toneSuggestedWith(String tone) {
    return '由主题推荐（$tone）';
  }

  @override
  String toneOption(String tone, String description) {
    return '$tone — $description';
  }

  @override
  String get pitchNatural => '音高：自然';

  @override
  String get pitchDescription => '音高调整（半音）';

  @override
  String get reminderPlayNow => '立即播放';

  @override
  String get reminderPlaying => '正在准备消息…';

  @override
  String get repeatTitle => '重复';

  @override
  String get repeatKindDaily => '每天';

  @override
  String get repeatKindWeekly => '指定日期';

  @override
  String get repeatKindInterval => '每隔 N 分钟';

  @override
  String get repeatKindOnce => '一次';

  @override
  String get repeatTimeLabel => '时间';

  @override
  String get repeatDateLabel => '日期';

  @override
  String get repeatPickDate => '选择日期';

  @override
  String repeatDateValue(String day, String month) {
    return '$day/$month';
  }

  @override
  String get repeatEveryHint => '每隔';

  @override
  String get repeatEverySuffix => '分钟';

  @override
  String get repeatWindowLabel => '时间段';

  @override
  String get repeatLabelDaily => '每天';

  @override
  String get repeatLabelWeekdays => '工作日';

  @override
  String get repeatLabelWeekends => '周末';

  @override
  String repeatLabelWithin(String repeat, String from, String to) {
    return '$repeat · $from–$to';
  }

  @override
  String repeatLabelOnce(String date) {
    return '一次 · $date';
  }

  @override
  String get dayMonShort => '一';

  @override
  String get dayTueShort => '二';

  @override
  String get dayWedShort => '三';

  @override
  String get dayThuShort => '四';

  @override
  String get dayFriShort => '五';

  @override
  String get daySatShort => '六';

  @override
  String get daySunShort => '日';

  @override
  String get dayMon => '星期一';

  @override
  String get dayTue => '星期二';

  @override
  String get dayWed => '星期三';

  @override
  String get dayThu => '星期四';

  @override
  String get dayFri => '星期五';

  @override
  String get daySat => '星期六';

  @override
  String get daySun => '星期日';

  @override
  String get dayMonAbbr => '周一';

  @override
  String get dayTueAbbr => '周二';

  @override
  String get dayWedAbbr => '周三';

  @override
  String get dayThuAbbr => '周四';

  @override
  String get dayFriAbbr => '周五';

  @override
  String get daySatAbbr => '周六';

  @override
  String get daySunAbbr => '周日';

  @override
  String get quietTitle => '免打扰时段';

  @override
  String get quietHelp => '这段时间内不发送消息。';

  @override
  String get formErrorNoVoice => '请先选择一个声音。';

  @override
  String get formErrorNoTheme => '请选择一个主题。';

  @override
  String get formErrorNoDays => '请至少选择一天。';

  @override
  String get formErrorInterval => '请输入消息之间间隔的分钟数（1 或以上）。';

  @override
  String get formErrorNoDate => '请选择日期。';

  @override
  String get voicesTitle => '声音';

  @override
  String get voicesEmpty => '还没有声音。\n点按 + 添加你爱的人的声音。';

  @override
  String get voicesAdd => '添加声音';

  @override
  String get voiceStatusReady => '声音已就绪';

  @override
  String get voiceStatusNotCreated => '声音尚未创建';

  @override
  String samplesCountAndSecs(String recordings, String duration) {
    return '$recordings · $duration';
  }

  @override
  String get importVoiceTitleNew => '新声音';

  @override
  String get importVoiceTitleEdit => '声音';

  @override
  String get voiceNameHint => '名字（例如：露西亚奶奶）';

  @override
  String get voiceNameRequired => '请输入名字。';

  @override
  String get voiceConsentHint => '谁授权使用这个声音';

  @override
  String get voiceConsentHelp => '必填：只能在本人同意的情况下使用其声音。';

  @override
  String get voiceConsentRequired => '请输入授权使用这个声音的人。';

  @override
  String get voiceLanguageHint => '语言';

  @override
  String get voiceListenerHint => '这个声音怎么称呼你';

  @override
  String get voiceListenerHelp => '可选，例如昵称。留空则使用设置中的名字。';

  @override
  String get voiceRegistered => '声音已保存。现在添加录音吧。';

  @override
  String get voiceSaved => '已保存。';

  @override
  String get voiceCreated => '声音已创建！可以用于你的提醒了。';

  @override
  String get samplesTitle => '录音';

  @override
  String get samplesAddFiles => '添加音频文件';

  @override
  String get samplesRecord => '录音';

  @override
  String get samplesStop => '停止录音';

  @override
  String samplesAdded(int count) {
    return '已添加录音：$count';
  }

  @override
  String sampleTitle(int number) {
    return '录音 $number';
  }

  @override
  String sampleDuration(String secs) {
    return '$secs 秒';
  }

  @override
  String recordingStatus(String elapsed, String limit) {
    return '正在录音… $elapsed / $limit';
  }

  @override
  String get recordPermissionDenied => '请允许使用麦克风来录制声音。';

  @override
  String get actionCancel => '取消';

  @override
  String get deleteVoice => '删除声音';

  @override
  String get deleteVoiceTitle => '删除这个声音？';

  @override
  String get deleteVoiceMessage =>
      '它的录音和已创建的声音将从这部手机和语音服务器上删除。使用它的提醒会被关闭并保留，方便你选择其他声音。';

  @override
  String get deleteVoiceConfirm => '删除';

  @override
  String get voiceDeleted => '声音已删除。';

  @override
  String get createVoice => '创建声音';

  @override
  String get createVoiceRunning => '正在创建声音…';

  @override
  String get createVoiceHint =>
      '请添加至少 10 秒的录音（WAV、MP3 或 FLAC，每段 2 秒到 3 分钟），然后创建声音。';

  @override
  String get createVoiceHintReady => '声音已就绪。添加了更多录音？重新创建即可改进。';

  @override
  String get settingsTitle => '设置';

  @override
  String get settingsListenerHint => '你的名字';

  @override
  String get settingsListenerHelp => '当声音没有为你设置昵称时，消息中会使用这个名字。';

  @override
  String get settingsEngineHint => '语音服务器地址';

  @override
  String get settingsSaved => '设置已保存。';

  @override
  String get settingsLanguageHint => '应用语言';

  @override
  String get languageSystemDefault => '跟随系统';

  @override
  String get channelName => '定时消息';

  @override
  String get notificationTitle => '给你的一条消息';

  @override
  String notificationBody(String text, String tone) {
    return '$text\n（语气：$tone）';
  }

  @override
  String pitchHigher(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: '音高：升高 $count 个半音',
    );
    return '$_temp0';
  }

  @override
  String pitchLower(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: '音高：降低 $count 个半音',
    );
    return '$_temp0';
  }

  @override
  String repeatLabelEvery(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: '每 $count 分钟',
    );
    return '$_temp0';
  }

  @override
  String samplesCount(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: '$count 段录音',
    );
    return '$_temp0';
  }

  @override
  String seconds(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: '$count 秒',
    );
    return '$_temp0';
  }

  @override
  String get deleteReminder => '删除提醒';

  @override
  String get samplesFillFirst => '请先填写名字和授权人，再添加录音。';
}

/// The translations for Chinese, as used in Taiwan (`zh_TW`).
class AppLocalizationsZhTw extends AppLocalizationsZh {
  AppLocalizationsZhTw() : super('zh_TW');

  @override
  String get appName => 'Friendly Voice';

  @override
  String get actionBack => '返回';

  @override
  String get errorGeneric => '發生問題，請再試一次。';

  @override
  String get errorPlayback => '無法播放這則訊息。';

  @override
  String get errorFileOpen => '無法開啟檔案。';

  @override
  String get errorNoAudioTrack => '這個檔案沒有音訊。請使用 WAV、MP3 或 FLAC 檔案。';

  @override
  String get errorAudioFormat => '不支援這種音訊格式。請將檔案轉換為 WAV。';

  @override
  String get errorAudioTooLong => '這段錄音超過 3 分鐘：請分成幾段。';

  @override
  String get errorMicUnavailable => '無法使用麥克風。';

  @override
  String get errorRecordingFailed => '錄音失敗，請再試一次。';

  @override
  String get rangeSeparator => '–';

  @override
  String clockFormat(int minutes, String seconds) {
    return '$minutes:$seconds';
  }

  @override
  String get tabReminders => '提醒';

  @override
  String get tabVoices => '聲音';

  @override
  String get tabSettings => '設定';

  @override
  String get remindersTitle => '提醒';

  @override
  String get remindersEmpty => '還沒有提醒。\n輕觸 + 在對的時間聽見你愛的人。';

  @override
  String get remindersAdd => '新增提醒';

  @override
  String get reminderSwitchDescription => '提醒開啟或關閉';

  @override
  String reminderTimeInterval(int minutes) {
    return '$minutes 分鐘';
  }

  @override
  String reminderThemeLine(String theme, String tone) {
    return '$theme · $tone';
  }

  @override
  String get reminderOwnText => '自訂文字';

  @override
  String get reminderVoiceUnavailable => '聲音無法使用 — 輕觸以選擇其他聲音';

  @override
  String get editReminderTitleNew => '新提醒';

  @override
  String get editReminderTitleEdit => '編輯提醒';

  @override
  String get reminderVoiceHint => '聲音';

  @override
  String get reminderNoVoices => '還沒有可用的聲音。請先在「聲音」分頁建立一個。';

  @override
  String get reminderGoToVoices => '前往「聲音」';

  @override
  String get reminderThemeHint => '主題';

  @override
  String get reminderToneHint => '語氣';

  @override
  String get toneSuggested => '依主題建議';

  @override
  String toneSuggestedWith(String tone) {
    return '依主題建議（$tone）';
  }

  @override
  String toneOption(String tone, String description) {
    return '$tone — $description';
  }

  @override
  String get pitchNatural => '音高：自然';

  @override
  String get pitchDescription => '音高調整（半音）';

  @override
  String get reminderPlayNow => '立即播放';

  @override
  String get reminderPlaying => '正在準備訊息…';

  @override
  String get repeatTitle => '重複';

  @override
  String get repeatKindDaily => '每天';

  @override
  String get repeatKindWeekly => '指定日期';

  @override
  String get repeatKindInterval => '每隔 N 分鐘';

  @override
  String get repeatKindOnce => '一次';

  @override
  String get repeatTimeLabel => '時間';

  @override
  String get repeatDateLabel => '日期';

  @override
  String get repeatPickDate => '選擇日期';

  @override
  String repeatDateValue(String day, String month) {
    return '$day/$month';
  }

  @override
  String get repeatEveryHint => '每隔';

  @override
  String get repeatEverySuffix => '分鐘';

  @override
  String get repeatWindowLabel => '時段';

  @override
  String get repeatLabelDaily => '每天';

  @override
  String get repeatLabelWeekdays => '平日';

  @override
  String get repeatLabelWeekends => '週末';

  @override
  String repeatLabelWithin(String repeat, String from, String to) {
    return '$repeat · $from–$to';
  }

  @override
  String repeatLabelOnce(String date) {
    return '一次 · $date';
  }

  @override
  String get dayMonShort => '一';

  @override
  String get dayTueShort => '二';

  @override
  String get dayWedShort => '三';

  @override
  String get dayThuShort => '四';

  @override
  String get dayFriShort => '五';

  @override
  String get daySatShort => '六';

  @override
  String get daySunShort => '日';

  @override
  String get dayMon => '星期一';

  @override
  String get dayTue => '星期二';

  @override
  String get dayWed => '星期三';

  @override
  String get dayThu => '星期四';

  @override
  String get dayFri => '星期五';

  @override
  String get daySat => '星期六';

  @override
  String get daySun => '星期日';

  @override
  String get dayMonAbbr => '週一';

  @override
  String get dayTueAbbr => '週二';

  @override
  String get dayWedAbbr => '週三';

  @override
  String get dayThuAbbr => '週四';

  @override
  String get dayFriAbbr => '週五';

  @override
  String get daySatAbbr => '週六';

  @override
  String get daySunAbbr => '週日';

  @override
  String get quietTitle => '勿擾時段';

  @override
  String get quietHelp => '這段時間內不會傳送訊息。';

  @override
  String get formErrorNoVoice => '請先選擇聲音。';

  @override
  String get formErrorNoTheme => '請選擇主題。';

  @override
  String get formErrorNoDays => '請至少選擇一天。';

  @override
  String get formErrorInterval => '請輸入訊息之間間隔的分鐘數（1 以上）。';

  @override
  String get formErrorNoDate => '請選擇日期。';

  @override
  String get voicesTitle => '聲音';

  @override
  String get voicesEmpty => '還沒有聲音。\n輕觸 + 新增你愛的人的聲音。';

  @override
  String get voicesAdd => '新增聲音';

  @override
  String get voiceStatusReady => '聲音已就緒';

  @override
  String get voiceStatusNotCreated => '尚未建立聲音';

  @override
  String samplesCountAndSecs(String recordings, String duration) {
    return '$recordings · $duration';
  }

  @override
  String get importVoiceTitleNew => '新聲音';

  @override
  String get importVoiceTitleEdit => '聲音';

  @override
  String get voiceNameHint => '名字（例如：露西亞奶奶）';

  @override
  String get voiceNameRequired => '請輸入名字。';

  @override
  String get voiceConsentHint => '誰授權使用這個聲音';

  @override
  String get voiceConsentHelp => '必填：只能在本人同意下使用其聲音。';

  @override
  String get voiceConsentRequired => '請輸入授權使用這個聲音的人。';

  @override
  String get voiceLanguageHint => '語言';

  @override
  String get voiceListenerHint => '這個聲音怎麼稱呼你';

  @override
  String get voiceListenerHelp => '選填，例如暱稱。留空則使用設定中的名字。';

  @override
  String get voiceRegistered => '聲音已儲存。現在新增錄音吧。';

  @override
  String get voiceSaved => '已儲存。';

  @override
  String get voiceCreated => '聲音建立完成！可以用在你的提醒了。';

  @override
  String get samplesTitle => '錄音';

  @override
  String get samplesAddFiles => '新增音訊檔案';

  @override
  String get samplesRecord => '錄音';

  @override
  String get samplesStop => '停止錄音';

  @override
  String samplesAdded(int count) {
    return '已新增錄音：$count';
  }

  @override
  String sampleTitle(int number) {
    return '錄音 $number';
  }

  @override
  String sampleDuration(String secs) {
    return '$secs 秒';
  }

  @override
  String recordingStatus(String elapsed, String limit) {
    return '錄音中… $elapsed / $limit';
  }

  @override
  String get recordPermissionDenied => '請允許使用麥克風來錄製聲音。';

  @override
  String get actionCancel => '取消';

  @override
  String get deleteVoice => '刪除聲音';

  @override
  String get deleteVoiceTitle => '要刪除這個聲音嗎？';

  @override
  String get deleteVoiceMessage =>
      '它的錄音和已建立的聲音會從這支手機和語音伺服器上刪除。使用它的提醒會被關閉並保留，方便你選擇其他聲音。';

  @override
  String get deleteVoiceConfirm => '刪除';

  @override
  String get voiceDeleted => '聲音已刪除。';

  @override
  String get createVoice => '建立聲音';

  @override
  String get createVoiceRunning => '正在建立聲音…';

  @override
  String get createVoiceHint =>
      '請新增至少 10 秒的錄音（WAV、MP3 或 FLAC，每段 2 秒到 3 分鐘），再建立聲音。';

  @override
  String get createVoiceHintReady => '聲音已就緒。新增了更多錄音？重新建立即可改善。';

  @override
  String get settingsTitle => '設定';

  @override
  String get settingsListenerHint => '你的名字';

  @override
  String get settingsListenerHelp => '當聲音沒有為你設定暱稱時，訊息中會使用這個名字。';

  @override
  String get settingsEngineHint => '語音伺服器網址';

  @override
  String get settingsSaved => '設定已儲存。';

  @override
  String get settingsLanguageHint => '應用程式語言';

  @override
  String get languageSystemDefault => '依照系統';

  @override
  String get channelName => '排定的訊息';

  @override
  String get notificationTitle => '給你的一則訊息';

  @override
  String notificationBody(String text, String tone) {
    return '$text\n（語氣：$tone）';
  }

  @override
  String pitchHigher(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: '音高：升高 $count 個半音',
    );
    return '$_temp0';
  }

  @override
  String pitchLower(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: '音高：降低 $count 個半音',
    );
    return '$_temp0';
  }

  @override
  String repeatLabelEvery(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: '每 $count 分鐘',
    );
    return '$_temp0';
  }

  @override
  String samplesCount(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: '$count 段錄音',
    );
    return '$_temp0';
  }

  @override
  String seconds(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: '$count 秒',
    );
    return '$_temp0';
  }

  @override
  String get deleteReminder => '刪除提醒';

  @override
  String get samplesFillFirst => '請先填寫名字和授權人，再新增錄音。';
}
