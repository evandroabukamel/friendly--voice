// ignore: unused_import
import 'package:intl/intl.dart' as intl;

import 'app_localizations.dart';

// ignore_for_file: type=lint

/// The translations for Japanese (`ja`).
class AppLocalizationsJa extends AppLocalizations {
  AppLocalizationsJa([String locale = 'ja']) : super(locale);

  @override
  String get appName => 'Friendly Voice';

  @override
  String get actionBack => '戻る';

  @override
  String get errorGeneric => '問題が発生しました。もう一度お試しください。';

  @override
  String get errorPlayback => 'メッセージを再生できませんでした。';

  @override
  String get errorFileOpen => 'ファイルを開けませんでした。';

  @override
  String get errorNoAudioTrack => 'このファイルには音声がありません。WAV、MP3、FLAC ファイルを使ってください。';

  @override
  String get errorAudioFormat => 'この音声形式には対応していません。ファイルを WAV に変換してください。';

  @override
  String get errorAudioTooLong => 'この録音は 3 分を超えています。いくつかに分けてください。';

  @override
  String get errorMicUnavailable => 'マイクを使用できません。';

  @override
  String get errorRecordingFailed => '録音に失敗しました。もう一度お試しください。';

  @override
  String get rangeSeparator => '–';

  @override
  String clockFormat(int minutes, String seconds) {
    return '$minutes:$seconds';
  }

  @override
  String get tabReminders => 'リマインダー';

  @override
  String get tabVoices => '声';

  @override
  String get tabSettings => '設定';

  @override
  String get remindersTitle => 'リマインダー';

  @override
  String get remindersEmpty => 'リマインダーはまだありません。\n+ をタップして、大切な人の声をちょうどいい時間に。';

  @override
  String get remindersAdd => 'リマインダーを追加';

  @override
  String get reminderSwitchDescription => 'リマインダーのオン・オフ';

  @override
  String reminderTimeInterval(int minutes) {
    return '$minutes 分';
  }

  @override
  String reminderThemeLine(String theme, String tone) {
    return '$theme · $tone';
  }

  @override
  String get reminderOwnText => '自分で書いた文';

  @override
  String get reminderVoiceUnavailable => '声が使えません — タップして別の声を選択';

  @override
  String get editReminderTitleNew => '新しいリマインダー';

  @override
  String get editReminderTitleEdit => 'リマインダーを編集';

  @override
  String get reminderVoiceHint => '声';

  @override
  String get reminderNoVoices => '使える声がまだありません。まず「声」タブで作成してください。';

  @override
  String get reminderGoToVoices => '「声」へ';

  @override
  String get reminderThemeHint => 'テーマ';

  @override
  String get reminderToneHint => '話し方';

  @override
  String get toneSuggested => 'テーマのおすすめ';

  @override
  String toneSuggestedWith(String tone) {
    return 'テーマのおすすめ（$tone）';
  }

  @override
  String toneOption(String tone, String description) {
    return '$tone — $description';
  }

  @override
  String get pitchNatural => '高さ：そのまま';

  @override
  String get pitchDescription => '声の高さ（半音）';

  @override
  String get reminderPlayNow => '今すぐ再生';

  @override
  String get reminderPlaying => 'メッセージを準備中…';

  @override
  String get repeatTitle => '繰り返し';

  @override
  String get repeatKindDaily => '毎日';

  @override
  String get repeatKindWeekly => '曜日を選ぶ';

  @override
  String get repeatKindInterval => 'N 分ごと';

  @override
  String get repeatKindOnce => '1 回だけ';

  @override
  String get repeatTimeLabel => '時刻';

  @override
  String get repeatDateLabel => '日付';

  @override
  String get repeatPickDate => '日付を選択';

  @override
  String repeatDateValue(String day, String month) {
    return '$day/$month';
  }

  @override
  String get repeatEveryHint => '間隔';

  @override
  String get repeatEverySuffix => '分';

  @override
  String get repeatWindowLabel => '時間帯';

  @override
  String get repeatLabelDaily => '毎日';

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
    return '1 回 · $date';
  }

  @override
  String get dayMonShort => '月';

  @override
  String get dayTueShort => '火';

  @override
  String get dayWedShort => '水';

  @override
  String get dayThuShort => '木';

  @override
  String get dayFriShort => '金';

  @override
  String get daySatShort => '土';

  @override
  String get daySunShort => '日';

  @override
  String get dayMon => '月曜日';

  @override
  String get dayTue => '火曜日';

  @override
  String get dayWed => '水曜日';

  @override
  String get dayThu => '木曜日';

  @override
  String get dayFri => '金曜日';

  @override
  String get daySat => '土曜日';

  @override
  String get daySun => '日曜日';

  @override
  String get dayMonAbbr => '月';

  @override
  String get dayTueAbbr => '火';

  @override
  String get dayWedAbbr => '水';

  @override
  String get dayThuAbbr => '木';

  @override
  String get dayFriAbbr => '金';

  @override
  String get daySatAbbr => '土';

  @override
  String get daySunAbbr => '日';

  @override
  String get quietTitle => 'おやすみ時間';

  @override
  String get quietHelp => 'この時間帯はメッセージを送りません。';

  @override
  String get formErrorNoVoice => 'まず声を選んでください。';

  @override
  String get formErrorNoTheme => 'テーマを選んでください。';

  @override
  String get formErrorNoDays => '曜日を 1 つ以上選んでください。';

  @override
  String get formErrorInterval => 'メッセージの間隔を分で入力してください（1 以上）。';

  @override
  String get formErrorNoDate => '日付を選んでください。';

  @override
  String get voicesTitle => '声';

  @override
  String get voicesEmpty => '声はまだありません。\n+ をタップして、大切な人の声を追加しましょう。';

  @override
  String get voicesAdd => '声を追加';

  @override
  String get voiceStatusReady => '声の準備ができました';

  @override
  String get voiceStatusNotCreated => '声はまだ作成されていません';

  @override
  String samplesCountAndSecs(String recordings, String duration) {
    return '$recordings · $duration';
  }

  @override
  String get importVoiceTitleNew => '新しい声';

  @override
  String get importVoiceTitleEdit => '声';

  @override
  String get voiceNameHint => '名前（例：ルシアおばあちゃん）';

  @override
  String get voiceNameRequired => '名前を入力してください。';

  @override
  String get voiceConsentHint => 'この声の使用を許可した人';

  @override
  String get voiceConsentHelp => '必須：本人の許可がある声だけを使ってください。';

  @override
  String get voiceConsentRequired => 'この声の使用を許可した人を入力してください。';

  @override
  String get voiceLanguageHint => '言語';

  @override
  String get voiceListenerHint => 'この声があなたを呼ぶ名前';

  @override
  String get voiceListenerHelp => '任意（例：ニックネーム）。空欄なら設定の名前を使います。';

  @override
  String get voiceRegistered => '声を保存しました。録音を追加しましょう。';

  @override
  String get voiceSaved => '保存しました。';

  @override
  String get voiceCreated => '声ができました！リマインダーで使えます。';

  @override
  String get samplesTitle => '録音';

  @override
  String get samplesAddFiles => '音声ファイルを追加';

  @override
  String get samplesRecord => '録音';

  @override
  String get samplesStop => '録音を停止';

  @override
  String samplesAdded(int count) {
    return '追加した録音：$count';
  }

  @override
  String sampleTitle(int number) {
    return '録音 $number';
  }

  @override
  String sampleDuration(String secs) {
    return '$secs 秒';
  }

  @override
  String recordingStatus(String elapsed, String limit) {
    return '録音中… $elapsed / $limit';
  }

  @override
  String get recordPermissionDenied => '声を録音するにはマイクを許可してください。';

  @override
  String get actionCancel => 'キャンセル';

  @override
  String get deleteVoice => '声を削除';

  @override
  String get deleteVoiceTitle => 'この声を削除しますか？';

  @override
  String get deleteVoiceMessage =>
      '録音と作成した声は、このスマートフォンと音声サーバーから削除されます。この声を使うリマインダーはオフになり、残るので、別の声を選べます。';

  @override
  String get deleteVoiceConfirm => '削除';

  @override
  String get voiceDeleted => '声を削除しました。';

  @override
  String get createVoice => '声を作成';

  @override
  String get createVoiceRunning => '声を作成中…';

  @override
  String get createVoiceHint =>
      '録音を合計 10 秒以上追加してから（WAV、MP3、FLAC、1 件 2 秒〜3 分）、声を作成してください。';

  @override
  String get createVoiceHintReady => '声の準備ができています。録音を追加したら、もう一度作成するとよくなります。';

  @override
  String get settingsTitle => '設定';

  @override
  String get settingsListenerHint => 'あなたの名前';

  @override
  String get settingsListenerHelp => '声にあなたのニックネームがないとき、メッセージで使われます。';

  @override
  String get settingsEngineHint => '音声サーバーの URL';

  @override
  String get settingsSaved => '設定を保存しました。';

  @override
  String get settingsLanguageHint => 'アプリの言語';

  @override
  String get languageSystemDefault => 'システムのデフォルト';

  @override
  String get channelName => '予定したメッセージ';

  @override
  String get notificationTitle => 'あなたへのメッセージ';

  @override
  String notificationBody(String text, String tone) {
    return '$text\n（話し方：$tone）';
  }

  @override
  String pitchHigher(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: '高さ：$count 半音高く',
    );
    return '$_temp0';
  }

  @override
  String pitchLower(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: '高さ：$count 半音低く',
    );
    return '$_temp0';
  }

  @override
  String repeatLabelEvery(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: '$count 分ごと',
    );
    return '$_temp0';
  }

  @override
  String samplesCount(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: '録音 $count 件',
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
  String get deleteReminder => 'リマインダーを削除';

  @override
  String get samplesFillFirst => '名前と許可した人を入力してから、録音を追加してください。';
}
