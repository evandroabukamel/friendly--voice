// ignore: unused_import
import 'package:intl/intl.dart' as intl;

import 'app_localizations.dart';

// ignore_for_file: type=lint

/// The translations for Arabic (`ar`).
class AppLocalizationsAr extends AppLocalizations {
  AppLocalizationsAr([String locale = 'ar']) : super(locale);

  @override
  String get appName => 'Friendly Voice';

  @override
  String get actionBack => 'رجوع';

  @override
  String get errorGeneric => 'حدث خطأ ما. حاول مرة أخرى.';

  @override
  String get errorPlayback => 'تعذّر تشغيل الرسالة.';

  @override
  String get errorFileOpen => 'تعذّر فتح الملف.';

  @override
  String get errorNoAudioTrack =>
      'لا يحتوي هذا الملف على صوت. استخدم ملف WAV أو MP3 أو FLAC.';

  @override
  String get errorAudioFormat =>
      'صيغة الصوت هذه غير مدعومة. حوّل الملف إلى WAV.';

  @override
  String get errorAudioTooLong =>
      'هذا التسجيل أطول من 3 دقائق: قسّمه إلى أجزاء.';

  @override
  String get errorMicUnavailable => 'الميكروفون غير متاح.';

  @override
  String get errorRecordingFailed => 'فشل التسجيل. حاول مرة أخرى.';

  @override
  String get rangeSeparator => '–';

  @override
  String clockFormat(int minutes, String seconds) {
    return '$minutes:$seconds';
  }

  @override
  String get tabReminders => 'التذكيرات';

  @override
  String get tabVoices => 'الأصوات';

  @override
  String get tabSettings => 'الإعدادات';

  @override
  String get remindersTitle => 'التذكيرات';

  @override
  String get remindersEmpty =>
      'لا توجد تذكيرات بعد.\nاضغط + لتسمع صوت من تحب في الوقت المناسب.';

  @override
  String get remindersAdd => 'إضافة تذكير';

  @override
  String get reminderSwitchDescription => 'تشغيل التذكير أو إيقافه';

  @override
  String reminderTimeInterval(int minutes) {
    return '$minutes د';
  }

  @override
  String reminderThemeLine(String theme, String tone) {
    return '$theme · $tone';
  }

  @override
  String get reminderOwnText => 'نصّك الخاص';

  @override
  String get reminderVoiceUnavailable =>
      'الصوت غير متاح — اضغط لاختيار صوت آخر';

  @override
  String get editReminderTitleNew => 'تذكير جديد';

  @override
  String get editReminderTitleEdit => 'تعديل التذكير';

  @override
  String get reminderVoiceHint => 'الصوت';

  @override
  String get reminderNoVoices =>
      'لا يوجد صوت جاهز بعد. أنشئ صوتًا أولًا من تبويب الأصوات.';

  @override
  String get reminderGoToVoices => 'الانتقال إلى الأصوات';

  @override
  String get reminderThemeHint => 'الموضوع';

  @override
  String get reminderToneHint => 'نبرة الصوت';

  @override
  String get toneSuggested => 'مقترحة حسب الموضوع';

  @override
  String toneSuggestedWith(String tone) {
    return 'مقترحة حسب الموضوع ($tone)';
  }

  @override
  String toneOption(String tone, String description) {
    return '$tone — $description';
  }

  @override
  String get pitchNatural => 'طبقة الصوت: طبيعية';

  @override
  String get pitchDescription => 'تغيير طبقة الصوت بأنصاف النغمات';

  @override
  String get reminderPlayNow => 'استمع الآن';

  @override
  String get reminderPlaying => 'جارٍ تحضير الرسالة…';

  @override
  String get repeatTitle => 'التكرار';

  @override
  String get repeatKindDaily => 'يوميًا';

  @override
  String get repeatKindWeekly => 'أيام مختارة';

  @override
  String get repeatKindInterval => 'كل N دقيقة';

  @override
  String get repeatKindOnce => 'مرة واحدة';

  @override
  String get repeatTimeLabel => 'الساعة';

  @override
  String get repeatDateLabel => 'التاريخ';

  @override
  String get repeatPickDate => 'اختر تاريخًا';

  @override
  String repeatDateValue(String day, String month) {
    return '$day/$month';
  }

  @override
  String get repeatEveryHint => 'كل';

  @override
  String get repeatEverySuffix => 'دقيقة';

  @override
  String get repeatWindowLabel => 'بين';

  @override
  String get repeatLabelDaily => 'يوميًا';

  @override
  String get repeatLabelWeekdays => 'أيام العمل';

  @override
  String get repeatLabelWeekends => 'عطلة نهاية الأسبوع';

  @override
  String repeatLabelWithin(String repeat, String from, String to) {
    return '$repeat · $from–$to';
  }

  @override
  String repeatLabelOnce(String date) {
    return 'مرة واحدة · $date';
  }

  @override
  String get dayMonShort => 'ن';

  @override
  String get dayTueShort => 'ث';

  @override
  String get dayWedShort => 'ر';

  @override
  String get dayThuShort => 'خ';

  @override
  String get dayFriShort => 'ج';

  @override
  String get daySatShort => 'س';

  @override
  String get daySunShort => 'ح';

  @override
  String get dayMon => 'الاثنين';

  @override
  String get dayTue => 'الثلاثاء';

  @override
  String get dayWed => 'الأربعاء';

  @override
  String get dayThu => 'الخميس';

  @override
  String get dayFri => 'الجمعة';

  @override
  String get daySat => 'السبت';

  @override
  String get daySun => 'الأحد';

  @override
  String get dayMonAbbr => 'اثنين';

  @override
  String get dayTueAbbr => 'ثلاثاء';

  @override
  String get dayWedAbbr => 'أربعاء';

  @override
  String get dayThuAbbr => 'خميس';

  @override
  String get dayFriAbbr => 'جمعة';

  @override
  String get daySatAbbr => 'سبت';

  @override
  String get daySunAbbr => 'أحد';

  @override
  String get quietTitle => 'ساعات الهدوء';

  @override
  String get quietHelp => 'لا رسائل بين هذين الوقتين.';

  @override
  String get formErrorNoVoice => 'اختر صوتًا أولًا.';

  @override
  String get formErrorNoTheme => 'اختر موضوعًا.';

  @override
  String get formErrorNoDays => 'اختر يومًا واحدًا على الأقل.';

  @override
  String get formErrorInterval => 'أدخل عدد الدقائق بين الرسائل (1 أو أكثر).';

  @override
  String get formErrorNoDate => 'اختر التاريخ.';

  @override
  String get voicesTitle => 'الأصوات';

  @override
  String get voicesEmpty => 'لا توجد أصوات بعد.\nاضغط + لإضافة صوت شخص تحبه.';

  @override
  String get voicesAdd => 'إضافة صوت';

  @override
  String get voiceStatusReady => 'الصوت جاهز';

  @override
  String get voiceStatusNotCreated => 'لم يُنشأ الصوت بعد';

  @override
  String samplesCountAndSecs(String recordings, String duration) {
    return '$recordings · $duration';
  }

  @override
  String get importVoiceTitleNew => 'صوت جديد';

  @override
  String get importVoiceTitleEdit => 'الصوت';

  @override
  String get voiceNameHint => 'الاسم (مثلًا: الجدة لوسيا)';

  @override
  String get voiceNameRequired => 'أدخل الاسم.';

  @override
  String get voiceConsentHint => 'من أذِن باستخدام هذا الصوت';

  @override
  String get voiceConsentHelp => 'مطلوب: لا تستخدم صوت أحد إلا بإذنه.';

  @override
  String get voiceConsentRequired => 'أدخل اسم من أذِن باستخدام هذا الصوت.';

  @override
  String get voiceLanguageHint => 'اللغة';

  @override
  String get voiceListenerHint => 'كيف يناديك هذا الصوت';

  @override
  String get voiceListenerHelp =>
      'اختياري، مثل اسم دلع. إن تُرك فارغًا: اسمك من الإعدادات.';

  @override
  String get voiceRegistered => 'تم حفظ الصوت. أضف الآن تسجيلات.';

  @override
  String get voiceSaved => 'تم الحفظ.';

  @override
  String get voiceCreated => 'تم إنشاء الصوت! إنه جاهز لتذكيراتك.';

  @override
  String get samplesTitle => 'التسجيلات';

  @override
  String get samplesAddFiles => 'إضافة ملفات صوتية';

  @override
  String get samplesRecord => 'تسجيل';

  @override
  String get samplesStop => 'إيقاف التسجيل';

  @override
  String samplesAdded(int count) {
    return 'التسجيلات المضافة: $count';
  }

  @override
  String sampleTitle(int number) {
    return 'تسجيل $number';
  }

  @override
  String sampleDuration(String secs) {
    return '$secs ث';
  }

  @override
  String recordingStatus(String elapsed, String limit) {
    return 'جارٍ التسجيل… $elapsed من $limit';
  }

  @override
  String get recordPermissionDenied => 'اسمح باستخدام الميكروفون لتسجيل صوت.';

  @override
  String get actionCancel => 'إلغاء';

  @override
  String get deleteVoice => 'حذف الصوت';

  @override
  String get deleteVoiceTitle => 'هل تريد حذف هذا الصوت؟';

  @override
  String get deleteVoiceMessage =>
      'ستُحذف تسجيلاته والصوت المُنشأ من هذا الهاتف ومن خادم الصوت. تُوقَف التذكيرات التي تستخدمه وتبقى محفوظة، لتختار صوتًا آخر.';

  @override
  String get deleteVoiceConfirm => 'حذف';

  @override
  String get voiceDeleted => 'تم حذف الصوت.';

  @override
  String get createVoice => 'إنشاء الصوت';

  @override
  String get createVoiceRunning => 'جارٍ إنشاء الصوت…';

  @override
  String get createVoiceHint =>
      'أضف 10 ثوانٍ على الأقل من التسجيلات (WAV أو MP3 أو FLAC، من ثانيتين إلى 3 دقائق لكل منها)، ثم أنشئ الصوت.';

  @override
  String get createVoiceHintReady =>
      'الصوت جاهز. أضفت تسجيلات جديدة؟ أنشئه مرة أخرى لتحسينه.';

  @override
  String get settingsTitle => 'الإعدادات';

  @override
  String get settingsListenerHint => 'اسمك';

  @override
  String get settingsListenerHelp =>
      'يُستخدم في الرسائل عندما لا يكون للصوت اسم دلع لك.';

  @override
  String get settingsEngineHint => 'عنوان خادم الصوت';

  @override
  String get settingsSaved => 'تم حفظ الإعدادات.';

  @override
  String get settingsLanguageHint => 'لغة التطبيق';

  @override
  String get languageSystemDefault => 'لغة النظام';

  @override
  String get channelName => 'الرسائل المجدولة';

  @override
  String get notificationTitle => 'رسالة لك';

  @override
  String notificationBody(String text, String tone) {
    return '$text\n(النبرة: $tone)';
  }

  @override
  String pitchHigher(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: 'الطبقة: أعلى بـ $count نصف نغمة',
      many: 'الطبقة: أعلى بـ $count نصف نغمة',
      few: 'الطبقة: أعلى بـ $count أنصاف نغمات',
      two: 'الطبقة: أعلى بنصفي نغمة',
      one: 'الطبقة: أعلى بنصف نغمة واحدة',
      zero: 'الطبقة: أعلى بـ $count نصف نغمة',
    );
    return '$_temp0';
  }

  @override
  String pitchLower(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: 'الطبقة: أخفض بـ $count نصف نغمة',
      many: 'الطبقة: أخفض بـ $count نصف نغمة',
      few: 'الطبقة: أخفض بـ $count أنصاف نغمات',
      two: 'الطبقة: أخفض بنصفي نغمة',
      one: 'الطبقة: أخفض بنصف نغمة واحدة',
      zero: 'الطبقة: أخفض بـ $count نصف نغمة',
    );
    return '$_temp0';
  }

  @override
  String repeatLabelEvery(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: 'كل $count دقيقة',
      many: 'كل $count دقيقة',
      few: 'كل $count دقائق',
      two: 'كل دقيقتين',
      one: 'كل دقيقة',
      zero: 'كل $count دقيقة',
    );
    return '$_temp0';
  }

  @override
  String samplesCount(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: '$count تسجيل',
      many: '$count تسجيلًا',
      few: '$count تسجيلات',
      two: 'تسجيلان',
      one: 'تسجيل واحد',
      zero: 'لا تسجيلات',
    );
    return '$_temp0';
  }

  @override
  String seconds(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: '$count ثانية',
      many: '$count ثانية',
      few: '$count ثوانٍ',
      two: 'ثانيتان',
      one: 'ثانية واحدة',
      zero: '$count ثانية',
    );
    return '$_temp0';
  }

  @override
  String get deleteReminder => 'حذف التذكير';

  @override
  String get samplesFillFirst =>
      'اكتب الاسم ومن أذن باستخدامه، ثم أضف التسجيلات.';
}
