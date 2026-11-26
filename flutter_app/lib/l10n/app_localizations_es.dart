// ignore: unused_import
import 'package:intl/intl.dart' as intl;

import 'app_localizations.dart';

// ignore_for_file: type=lint

/// The translations for Spanish Castilian (`es`).
class AppLocalizationsEs extends AppLocalizations {
  AppLocalizationsEs([String locale = 'es']) : super(locale);

  @override
  String get appName => 'Friendly Voice';

  @override
  String get actionBack => 'Atrás';

  @override
  String get errorGeneric => 'Algo salió mal. Inténtalo de nuevo.';

  @override
  String get errorPlayback => 'No se pudo reproducir el mensaje.';

  @override
  String get errorFileOpen => 'No se pudo abrir el archivo.';

  @override
  String get errorNoAudioTrack =>
      'Este archivo no tiene audio. Usa un archivo WAV, MP3 o FLAC.';

  @override
  String get errorAudioFormat =>
      'Este formato de audio no es compatible. Convierte el archivo a WAV.';

  @override
  String get errorAudioTooLong =>
      'Esta grabación dura más de 3 minutos: divídela en partes.';

  @override
  String get errorMicUnavailable => 'El micrófono no está disponible.';

  @override
  String get errorRecordingFailed => 'La grabación falló. Inténtalo de nuevo.';

  @override
  String get rangeSeparator => '–';

  @override
  String clockFormat(int minutes, String seconds) {
    return '$minutes:$seconds';
  }

  @override
  String get tabReminders => 'Recordatorios';

  @override
  String get tabVoices => 'Voces';

  @override
  String get tabSettings => 'Ajustes';

  @override
  String get remindersTitle => 'Recordatorios';

  @override
  String get remindersEmpty =>
      'Aún no hay recordatorios.\nToca + para oír a quien quieres en el momento justo.';

  @override
  String get remindersAdd => 'Añadir recordatorio';

  @override
  String get reminderSwitchDescription => 'Recordatorio activado o desactivado';

  @override
  String reminderTimeInterval(int minutes) {
    return '$minutes min';
  }

  @override
  String reminderThemeLine(String theme, String tone) {
    return '$theme · $tone';
  }

  @override
  String get reminderOwnText => 'Tu propio texto';

  @override
  String get reminderVoiceUnavailable =>
      'Voz no disponible — toca para elegir otra';

  @override
  String get editReminderTitleNew => 'Nuevo recordatorio';

  @override
  String get editReminderTitleEdit => 'Editar recordatorio';

  @override
  String get reminderVoiceHint => 'Voz';

  @override
  String get reminderNoVoices =>
      'Todavía no hay ninguna voz lista. Crea una primero en la pestaña Voces.';

  @override
  String get reminderGoToVoices => 'Ir a Voces';

  @override
  String get reminderThemeHint => 'Tema';

  @override
  String get reminderToneHint => 'Tono de voz';

  @override
  String get toneSuggested => 'Sugerido por el tema';

  @override
  String toneSuggestedWith(String tone) {
    return 'Sugerido por el tema ($tone)';
  }

  @override
  String toneOption(String tone, String description) {
    return '$tone — $description';
  }

  @override
  String get pitchNatural => 'Altura: natural';

  @override
  String get pitchDescription => 'Cambio de altura en semitonos';

  @override
  String get reminderPlayNow => 'Escuchar ahora';

  @override
  String get reminderPlaying => 'Preparando el mensaje…';

  @override
  String get repeatTitle => 'Repetir';

  @override
  String get repeatKindDaily => 'Cada día';

  @override
  String get repeatKindWeekly => 'Días elegidos';

  @override
  String get repeatKindInterval => 'Cada N minutos';

  @override
  String get repeatKindOnce => 'Una vez';

  @override
  String get repeatTimeLabel => 'A las';

  @override
  String get repeatDateLabel => 'El';

  @override
  String get repeatPickDate => 'Elige una fecha';

  @override
  String repeatDateValue(String day, String month) {
    return '$day/$month';
  }

  @override
  String get repeatEveryHint => 'Cada';

  @override
  String get repeatEverySuffix => 'minutos';

  @override
  String get repeatWindowLabel => 'Entre';

  @override
  String get repeatLabelDaily => 'Cada día';

  @override
  String get repeatLabelWeekdays => 'Entre semana';

  @override
  String get repeatLabelWeekends => 'Fines de semana';

  @override
  String repeatLabelWithin(String repeat, String from, String to) {
    return '$repeat · $from–$to';
  }

  @override
  String repeatLabelOnce(String date) {
    return 'Una vez · $date';
  }

  @override
  String get dayMonShort => 'L';

  @override
  String get dayTueShort => 'M';

  @override
  String get dayWedShort => 'X';

  @override
  String get dayThuShort => 'J';

  @override
  String get dayFriShort => 'V';

  @override
  String get daySatShort => 'S';

  @override
  String get daySunShort => 'D';

  @override
  String get dayMon => 'Lunes';

  @override
  String get dayTue => 'Martes';

  @override
  String get dayWed => 'Miércoles';

  @override
  String get dayThu => 'Jueves';

  @override
  String get dayFri => 'Viernes';

  @override
  String get daySat => 'Sábado';

  @override
  String get daySun => 'Domingo';

  @override
  String get dayMonAbbr => 'Lun';

  @override
  String get dayTueAbbr => 'Mar';

  @override
  String get dayWedAbbr => 'Mié';

  @override
  String get dayThuAbbr => 'Jue';

  @override
  String get dayFriAbbr => 'Vie';

  @override
  String get daySatAbbr => 'Sáb';

  @override
  String get daySunAbbr => 'Dom';

  @override
  String get quietTitle => 'Horas de silencio';

  @override
  String get quietHelp => 'Ningún mensaje entre estas horas.';

  @override
  String get formErrorNoVoice => 'Primero elige una voz.';

  @override
  String get formErrorNoTheme => 'Elige un tema.';

  @override
  String get formErrorNoDays => 'Elige al menos un día.';

  @override
  String get formErrorInterval =>
      'Indica cuántos minutos entre mensajes (1 o más).';

  @override
  String get formErrorNoDate => 'Elige la fecha.';

  @override
  String get voicesTitle => 'Voces';

  @override
  String get voicesEmpty =>
      'Aún no hay voces.\nToca + para añadir la voz de alguien a quien quieres.';

  @override
  String get voicesAdd => 'Añadir voz';

  @override
  String get voiceStatusReady => 'Voz lista';

  @override
  String get voiceStatusNotCreated => 'Voz aún no creada';

  @override
  String samplesCountAndSecs(String recordings, String duration) {
    return '$recordings · $duration';
  }

  @override
  String get importVoiceTitleNew => 'Nueva voz';

  @override
  String get importVoiceTitleEdit => 'Voz';

  @override
  String get voiceNameHint => 'Nombre (p. ej., Abuela Lucía)';

  @override
  String get voiceNameRequired => 'Escribe el nombre.';

  @override
  String get voiceConsentHint => 'Quién autorizó el uso de esta voz';

  @override
  String get voiceConsentHelp =>
      'Obligatorio: usa una voz solo con el permiso de la persona.';

  @override
  String get voiceConsentRequired =>
      'Escribe quién autorizó el uso de esta voz.';

  @override
  String get voiceLanguageHint => 'Idioma';

  @override
  String get voiceListenerHint => 'Cómo te llama esta voz';

  @override
  String get voiceListenerHelp =>
      'Opcional, p. ej., un apodo. Vacío: tu nombre de Ajustes.';

  @override
  String get voiceRegistered => 'Voz guardada. Ahora añade grabaciones.';

  @override
  String get voiceSaved => 'Guardado.';

  @override
  String get voiceCreated =>
      '¡Voz creada! Ya está lista para tus recordatorios.';

  @override
  String get samplesTitle => 'Grabaciones';

  @override
  String get samplesAddFiles => 'Añadir archivos de audio';

  @override
  String get samplesRecord => 'Grabar';

  @override
  String get samplesStop => 'Detener grabación';

  @override
  String samplesAdded(int count) {
    return 'Grabaciones añadidas: $count';
  }

  @override
  String sampleTitle(int number) {
    return 'Grabación $number';
  }

  @override
  String sampleDuration(String secs) {
    return '$secs s';
  }

  @override
  String recordingStatus(String elapsed, String limit) {
    return 'Grabando… $elapsed de $limit';
  }

  @override
  String get recordPermissionDenied =>
      'Permite el micrófono para grabar una voz.';

  @override
  String get actionCancel => 'Cancelar';

  @override
  String get deleteVoice => 'Eliminar voz';

  @override
  String get deleteVoiceTitle => '¿Eliminar esta voz?';

  @override
  String get deleteVoiceMessage =>
      'Sus grabaciones y la voz creada se borran de este móvil y del servidor de voz. Los recordatorios que la usan se desactivan y se conservan, para que elijas otra voz.';

  @override
  String get deleteVoiceConfirm => 'Eliminar';

  @override
  String get voiceDeleted => 'Voz eliminada.';

  @override
  String get createVoice => 'Crear voz';

  @override
  String get createVoiceRunning => 'Creando la voz…';

  @override
  String get createVoiceHint =>
      'Añade al menos 10 segundos de grabaciones (WAV, MP3 o FLAC, de 2 s a 3 min cada una) y luego crea la voz.';

  @override
  String get createVoiceHintReady =>
      'La voz está lista. ¿Añadiste más grabaciones? Créala de nuevo para mejorarla.';

  @override
  String get settingsTitle => 'Ajustes';

  @override
  String get settingsListenerHint => 'Tu nombre';

  @override
  String get settingsListenerHelp =>
      'Se usa en los mensajes cuando la voz no tiene un apodo para ti.';

  @override
  String get settingsEngineHint => 'URL del servidor de voz';

  @override
  String get settingsSaved => 'Ajustes guardados.';

  @override
  String get settingsLanguageHint => 'Idioma de la app';

  @override
  String get languageSystemDefault => 'Predeterminado del sistema';

  @override
  String get channelName => 'Mensajes programados';

  @override
  String get notificationTitle => 'Un mensaje para ti';

  @override
  String notificationBody(String text, String tone) {
    return '$text\n(tono: $tone)';
  }

  @override
  String pitchHigher(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: 'Altura: $count semitonos más aguda',
      many: 'Altura: $count semitonos más aguda',
      one: 'Altura: $count semitono más aguda',
    );
    return '$_temp0';
  }

  @override
  String pitchLower(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: 'Altura: $count semitonos más grave',
      many: 'Altura: $count semitonos más grave',
      one: 'Altura: $count semitono más grave',
    );
    return '$_temp0';
  }

  @override
  String repeatLabelEvery(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: 'Cada $count min',
      many: 'Cada $count min',
      one: 'Cada minuto',
    );
    return '$_temp0';
  }

  @override
  String samplesCount(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: '$count grabaciones',
      many: '$count grabaciones',
      one: '$count grabación',
    );
    return '$_temp0';
  }

  @override
  String seconds(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: '$count segundos',
      many: '$count segundos',
      one: '$count segundo',
    );
    return '$_temp0';
  }

  @override
  String get deleteReminder => 'Borrar recordatorio';

  @override
  String get samplesFillFirst =>
      'Escribe el nombre y quién lo autorizó, y luego añade grabaciones.';
}

/// The translations for Spanish Castilian, as used in Argentina (`es_AR`).
class AppLocalizationsEsAr extends AppLocalizationsEs {
  AppLocalizationsEsAr() : super('es_AR');

  @override
  String get errorGeneric => 'Algo salió mal. Probá de nuevo.';

  @override
  String get errorNoAudioTrack =>
      'Este archivo no tiene audio. Usá un archivo WAV, MP3 o FLAC.';

  @override
  String get errorAudioFormat =>
      'Este formato de audio no es compatible. Convertí el archivo a WAV.';

  @override
  String get errorAudioTooLong =>
      'Esta grabación dura más de 3 minutos: dividila en partes.';

  @override
  String get errorRecordingFailed => 'La grabación falló. Probá de nuevo.';

  @override
  String get tabSettings => 'Configuración';

  @override
  String get reminderVoiceUnavailable =>
      'Voz no disponible — tocá para elegir otra';

  @override
  String get reminderNoVoices =>
      'Todavía no hay ninguna voz lista. Creá una primero en la pestaña Voces.';

  @override
  String get repeatPickDate => 'Elegí una fecha';

  @override
  String get formErrorNoVoice => 'Primero elegí una voz.';

  @override
  String get formErrorNoTheme => 'Elegí un tema.';

  @override
  String get formErrorNoDays => 'Elegí al menos un día.';

  @override
  String get formErrorInterval =>
      'Indicá cuántos minutos entre mensajes (1 o más).';

  @override
  String get formErrorNoDate => 'Elegí la fecha.';

  @override
  String get voicesEmpty =>
      'Todavía no hay voces.\nTocá + para agregar la voz de alguien a quien querés.';

  @override
  String get voicesAdd => 'Agregar voz';

  @override
  String get voiceNameRequired => 'Escribí el nombre.';

  @override
  String get voiceConsentHelp =>
      'Obligatorio: usá una voz solo con el permiso de la persona.';

  @override
  String get voiceConsentRequired =>
      'Escribí quién autorizó el uso de esta voz.';

  @override
  String get voiceListenerHelp =>
      'Opcional, p. ej., un apodo. Vacío: tu nombre de Configuración.';

  @override
  String get voiceRegistered => 'Voz guardada. Ahora agregá grabaciones.';

  @override
  String get samplesAddFiles => 'Agregar archivos de audio';

  @override
  String samplesAdded(int count) {
    return 'Grabaciones agregadas: $count';
  }

  @override
  String get recordPermissionDenied =>
      'Permití el micrófono para grabar una voz.';

  @override
  String get createVoiceHint =>
      'Agregá al menos 10 segundos de grabaciones (WAV, MP3 o FLAC, de 2 s a 3 min cada una) y después creá la voz.';

  @override
  String get createVoiceHintReady =>
      'La voz está lista. ¿Agregaste más grabaciones? Creala de nuevo para mejorarla.';

  @override
  String get settingsTitle => 'Configuración';

  @override
  String get settingsSaved => 'Configuración guardada.';
}

/// The translations for Spanish Castilian, as used in Mexico (`es_MX`).
class AppLocalizationsEsMx extends AppLocalizationsEs {
  AppLocalizationsEsMx() : super('es_MX');

  @override
  String get tabSettings => 'Configuración';

  @override
  String get voicesEmpty =>
      'Aún no hay voces.\nToca + para agregar la voz de alguien a quien quieres.';

  @override
  String get voicesAdd => 'Agregar voz';

  @override
  String get voiceListenerHelp =>
      'Opcional, p. ej., un apodo. Vacío: tu nombre de Configuración.';

  @override
  String get voiceRegistered => 'Voz guardada. Ahora agrega grabaciones.';

  @override
  String get samplesAddFiles => 'Agregar archivos de audio';

  @override
  String samplesAdded(int count) {
    return 'Grabaciones agregadas: $count';
  }

  @override
  String get createVoiceHint =>
      'Agrega al menos 10 segundos de grabaciones (WAV, MP3 o FLAC, de 2 s a 3 min cada una) y luego crea la voz.';

  @override
  String get createVoiceHintReady =>
      'La voz está lista. ¿Agregaste más grabaciones? Créala de nuevo para mejorarla.';

  @override
  String get settingsTitle => 'Configuración';

  @override
  String get settingsSaved => 'Configuración guardada.';
}
