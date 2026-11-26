// ignore: unused_import
import 'package:intl/intl.dart' as intl;

import 'app_localizations.dart';

// ignore_for_file: type=lint

/// The translations for Turkish (`tr`).
class AppLocalizationsTr extends AppLocalizations {
  AppLocalizationsTr([String locale = 'tr']) : super(locale);

  @override
  String get appName => 'Friendly Voice';

  @override
  String get actionBack => 'Geri';

  @override
  String get errorGeneric => 'Bir sorun oluştu. Lütfen tekrar deneyin.';

  @override
  String get errorPlayback => 'Mesaj çalınamadı.';

  @override
  String get errorFileOpen => 'Dosya açılamadı.';

  @override
  String get errorNoAudioTrack =>
      'Bu dosyada ses yok. WAV, MP3 veya FLAC dosyası kullanın.';

  @override
  String get errorAudioFormat =>
      'Bu ses biçimi desteklenmiyor. Dosyayı WAV biçimine dönüştürün.';

  @override
  String get errorAudioTooLong => 'Bu kayıt 3 dakikadan uzun: parçalara bölün.';

  @override
  String get errorMicUnavailable => 'Mikrofon kullanılamıyor.';

  @override
  String get errorRecordingFailed =>
      'Kayıt başarısız oldu. Lütfen tekrar deneyin.';

  @override
  String get rangeSeparator => '–';

  @override
  String clockFormat(int minutes, String seconds) {
    return '$minutes:$seconds';
  }

  @override
  String get tabReminders => 'Hatırlatıcılar';

  @override
  String get tabVoices => 'Sesler';

  @override
  String get tabSettings => 'Ayarlar';

  @override
  String get remindersTitle => 'Hatırlatıcılar';

  @override
  String get remindersEmpty =>
      'Henüz hatırlatıcı yok.\nSevdiğiniz birini doğru anda duymak için + simgesine dokunun.';

  @override
  String get remindersAdd => 'Hatırlatıcı ekle';

  @override
  String get reminderSwitchDescription => 'Hatırlatıcı açık veya kapalı';

  @override
  String reminderTimeInterval(int minutes) {
    return '$minutes dk';
  }

  @override
  String reminderThemeLine(String theme, String tone) {
    return '$theme · $tone';
  }

  @override
  String get reminderOwnText => 'Kendi metniniz';

  @override
  String get reminderVoiceUnavailable =>
      'Ses kullanılamıyor — başka bir ses seçmek için dokunun';

  @override
  String get editReminderTitleNew => 'Yeni hatırlatıcı';

  @override
  String get editReminderTitleEdit => 'Hatırlatıcıyı düzenle';

  @override
  String get reminderVoiceHint => 'Ses';

  @override
  String get reminderNoVoices =>
      'Henüz hazır ses yok. Önce Sesler sekmesinde bir ses oluşturun.';

  @override
  String get reminderGoToVoices => 'Seslere git';

  @override
  String get reminderThemeHint => 'Tema';

  @override
  String get reminderToneHint => 'Ses tonu';

  @override
  String get toneSuggested => 'Temaya göre önerilen';

  @override
  String toneSuggestedWith(String tone) {
    return 'Temaya göre önerilen ($tone)';
  }

  @override
  String toneOption(String tone, String description) {
    return '$tone — $description';
  }

  @override
  String get pitchNatural => 'Perde: doğal';

  @override
  String get pitchDescription => 'Yarım ton olarak perde kaydırma';

  @override
  String get reminderPlayNow => 'Şimdi dinle';

  @override
  String get reminderPlaying => 'Mesaj hazırlanıyor…';

  @override
  String get repeatTitle => 'Tekrarla';

  @override
  String get repeatKindDaily => 'Her gün';

  @override
  String get repeatKindWeekly => 'Seçilen günler';

  @override
  String get repeatKindInterval => 'Her N dakikada';

  @override
  String get repeatKindOnce => 'Bir kez';

  @override
  String get repeatTimeLabel => 'Saat';

  @override
  String get repeatDateLabel => 'Tarih';

  @override
  String get repeatPickDate => 'Tarih seçin';

  @override
  String repeatDateValue(String day, String month) {
    return '$day/$month';
  }

  @override
  String get repeatEveryHint => 'Her';

  @override
  String get repeatEverySuffix => 'dakikada';

  @override
  String get repeatWindowLabel => 'Saat aralığı';

  @override
  String get repeatLabelDaily => 'Her gün';

  @override
  String get repeatLabelWeekdays => 'Hafta içi';

  @override
  String get repeatLabelWeekends => 'Hafta sonu';

  @override
  String repeatLabelWithin(String repeat, String from, String to) {
    return '$repeat · $from–$to';
  }

  @override
  String repeatLabelOnce(String date) {
    return 'Bir kez · $date';
  }

  @override
  String get dayMonShort => 'P';

  @override
  String get dayTueShort => 'S';

  @override
  String get dayWedShort => 'Ç';

  @override
  String get dayThuShort => 'P';

  @override
  String get dayFriShort => 'C';

  @override
  String get daySatShort => 'C';

  @override
  String get daySunShort => 'P';

  @override
  String get dayMon => 'Pazartesi';

  @override
  String get dayTue => 'Salı';

  @override
  String get dayWed => 'Çarşamba';

  @override
  String get dayThu => 'Perşembe';

  @override
  String get dayFri => 'Cuma';

  @override
  String get daySat => 'Cumartesi';

  @override
  String get daySun => 'Pazar';

  @override
  String get dayMonAbbr => 'Pzt';

  @override
  String get dayTueAbbr => 'Sal';

  @override
  String get dayWedAbbr => 'Çar';

  @override
  String get dayThuAbbr => 'Per';

  @override
  String get dayFriAbbr => 'Cum';

  @override
  String get daySatAbbr => 'Cmt';

  @override
  String get daySunAbbr => 'Paz';

  @override
  String get quietTitle => 'Sessiz saatler';

  @override
  String get quietHelp => 'Bu saatler arasında mesaj gelmez.';

  @override
  String get formErrorNoVoice => 'Önce bir ses seçin.';

  @override
  String get formErrorNoTheme => 'Bir tema seçin.';

  @override
  String get formErrorNoDays => 'En az bir gün seçin.';

  @override
  String get formErrorInterval =>
      'Mesajlar arasında kaç dakika olacağını girin (1 veya daha fazla).';

  @override
  String get formErrorNoDate => 'Tarihi seçin.';

  @override
  String get voicesTitle => 'Sesler';

  @override
  String get voicesEmpty =>
      'Henüz ses yok.\nSevdiğiniz birinin sesini eklemek için + simgesine dokunun.';

  @override
  String get voicesAdd => 'Ses ekle';

  @override
  String get voiceStatusReady => 'Ses hazır';

  @override
  String get voiceStatusNotCreated => 'Ses henüz oluşturulmadı';

  @override
  String samplesCountAndSecs(String recordings, String duration) {
    return '$recordings · $duration';
  }

  @override
  String get importVoiceTitleNew => 'Yeni ses';

  @override
  String get importVoiceTitleEdit => 'Ses';

  @override
  String get voiceNameHint => 'Ad (ör. Lucia Nine)';

  @override
  String get voiceNameRequired => 'Adı girin.';

  @override
  String get voiceConsentHint => 'Bu sesin kullanımına kim izin verdi';

  @override
  String get voiceConsentHelp =>
      'Zorunlu: bir sesi yalnızca kişinin izniyle kullanın.';

  @override
  String get voiceConsentRequired =>
      'Bu sesin kullanımına kimin izin verdiğini girin.';

  @override
  String get voiceLanguageHint => 'Dil';

  @override
  String get voiceListenerHint => 'Bu ses size nasıl hitap ediyor';

  @override
  String get voiceListenerHelp =>
      'İsteğe bağlı, ör. bir lakap. Boşsa: Ayarlar’daki adınız.';

  @override
  String get voiceRegistered => 'Ses kaydedildi. Şimdi kayıt ekleyin.';

  @override
  String get voiceSaved => 'Kaydedildi.';

  @override
  String get voiceCreated => 'Ses oluşturuldu! Hatırlatıcılarınız için hazır.';

  @override
  String get samplesTitle => 'Kayıtlar';

  @override
  String get samplesAddFiles => 'Ses dosyası ekle';

  @override
  String get samplesRecord => 'Kaydet';

  @override
  String get samplesStop => 'Kaydı durdur';

  @override
  String samplesAdded(int count) {
    return 'Eklenen kayıt: $count';
  }

  @override
  String sampleTitle(int number) {
    return 'Kayıt $number';
  }

  @override
  String sampleDuration(String secs) {
    return '$secs sn';
  }

  @override
  String recordingStatus(String elapsed, String limit) {
    return 'Kaydediliyor… $elapsed / $limit';
  }

  @override
  String get recordPermissionDenied =>
      'Ses kaydetmek için mikrofona izin verin.';

  @override
  String get actionCancel => 'İptal';

  @override
  String get deleteVoice => 'Sesi sil';

  @override
  String get deleteVoiceTitle => 'Bu ses silinsin mi?';

  @override
  String get deleteVoiceMessage =>
      'Kayıtları ve oluşturulan ses bu telefondan ve ses sunucusundan silinir. Bu sesi kullanan hatırlatıcılar kapatılır ve saklanır; böylece başka bir ses seçebilirsiniz.';

  @override
  String get deleteVoiceConfirm => 'Sil';

  @override
  String get voiceDeleted => 'Ses silindi.';

  @override
  String get createVoice => 'Ses oluştur';

  @override
  String get createVoiceRunning => 'Ses oluşturuluyor…';

  @override
  String get createVoiceHint =>
      'En az 10 saniyelik kayıt ekleyin (WAV, MP3 veya FLAC, her biri 2 sn ile 3 dk arası), sonra sesi oluşturun.';

  @override
  String get createVoiceHintReady =>
      'Ses hazır. Yeni kayıt mı eklediniz? İyileştirmek için yeniden oluşturun.';

  @override
  String get settingsTitle => 'Ayarlar';

  @override
  String get settingsListenerHint => 'Adınız';

  @override
  String get settingsListenerHelp =>
      'Bir sesin size özel lakabı yoksa mesajlarda kullanılır.';

  @override
  String get settingsEngineHint => 'Ses sunucusu adresi';

  @override
  String get settingsSaved => 'Ayarlar kaydedildi.';

  @override
  String get settingsLanguageHint => 'Uygulama dili';

  @override
  String get languageSystemDefault => 'Sistem varsayılanı';

  @override
  String get channelName => 'Planlanmış mesajlar';

  @override
  String get notificationTitle => 'Size bir mesaj var';

  @override
  String notificationBody(String text, String tone) {
    return '$text\n(ton: $tone)';
  }

  @override
  String pitchHigher(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: 'Perde: $count yarım ton daha ince',
      one: 'Perde: $count yarım ton daha ince',
    );
    return '$_temp0';
  }

  @override
  String pitchLower(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: 'Perde: $count yarım ton daha kalın',
      one: 'Perde: $count yarım ton daha kalın',
    );
    return '$_temp0';
  }

  @override
  String repeatLabelEvery(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: 'Her $count dakikada',
      one: 'Her $count dakikada',
    );
    return '$_temp0';
  }

  @override
  String samplesCount(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: '$count kayıt',
      one: '$count kayıt',
    );
    return '$_temp0';
  }

  @override
  String seconds(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: '$count saniye',
      one: '$count saniye',
    );
    return '$_temp0';
  }

  @override
  String get deleteReminder => 'Hatırlatıcıyı sil';

  @override
  String get samplesFillFirst =>
      'Adı ve izni vereni yazın, sonra kayıt ekleyin.';
}
