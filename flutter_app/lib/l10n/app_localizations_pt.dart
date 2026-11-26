// ignore: unused_import
import 'package:intl/intl.dart' as intl;

import 'app_localizations.dart';

// ignore_for_file: type=lint

/// The translations for Portuguese (`pt`).
class AppLocalizationsPt extends AppLocalizations {
  AppLocalizationsPt([String locale = 'pt']) : super(locale);

  @override
  String get appName => 'Friendly Voice';

  @override
  String get actionBack => 'Voltar';

  @override
  String get errorGeneric => 'Algo deu errado. Tente de novo.';

  @override
  String get errorPlayback => 'Não foi possível tocar a mensagem.';

  @override
  String get errorFileOpen => 'Não foi possível abrir o arquivo.';

  @override
  String get errorNoAudioTrack =>
      'Este arquivo não tem áudio. Use um arquivo WAV, MP3 ou FLAC.';

  @override
  String get errorAudioFormat =>
      'Este formato de áudio não é compatível. Converta o arquivo para WAV.';

  @override
  String get errorAudioTooLong =>
      'Esta gravação passa de 3 minutos: divida em partes.';

  @override
  String get errorMicUnavailable => 'O microfone não está disponível.';

  @override
  String get errorRecordingFailed => 'A gravação falhou. Tente de novo.';

  @override
  String get rangeSeparator => '–';

  @override
  String clockFormat(int minutes, String seconds) {
    return '$minutes:$seconds';
  }

  @override
  String get tabReminders => 'Lembretes';

  @override
  String get tabVoices => 'Vozes';

  @override
  String get tabSettings => 'Ajustes';

  @override
  String get remindersTitle => 'Lembretes';

  @override
  String get remindersEmpty =>
      'Nenhum lembrete ainda.\nToque em + para ouvir quem você ama na hora certa.';

  @override
  String get remindersAdd => 'Novo lembrete';

  @override
  String get reminderSwitchDescription => 'Lembrete ligado ou desligado';

  @override
  String reminderTimeInterval(int minutes) {
    return '$minutes min';
  }

  @override
  String reminderThemeLine(String theme, String tone) {
    return '$theme · $tone';
  }

  @override
  String get reminderOwnText => 'Seu próprio texto';

  @override
  String get reminderVoiceUnavailable =>
      'Voz indisponível — toque para escolher outra';

  @override
  String get editReminderTitleNew => 'Novo lembrete';

  @override
  String get editReminderTitleEdit => 'Editar lembrete';

  @override
  String get reminderVoiceHint => 'Voz';

  @override
  String get reminderNoVoices =>
      'Nenhuma voz pronta ainda. Crie uma na aba Vozes primeiro.';

  @override
  String get reminderGoToVoices => 'Ir para Vozes';

  @override
  String get reminderThemeHint => 'Tema';

  @override
  String get reminderToneHint => 'Tom de voz';

  @override
  String get toneSuggested => 'Sugerido pelo tema';

  @override
  String toneSuggestedWith(String tone) {
    return 'Sugerido pelo tema ($tone)';
  }

  @override
  String toneOption(String tone, String description) {
    return '$tone — $description';
  }

  @override
  String get pitchNatural => 'Altura: natural';

  @override
  String get pitchDescription => 'Ajuste de altura em semitons';

  @override
  String get reminderPlayNow => 'Ouvir agora';

  @override
  String get reminderPlaying => 'Preparando a mensagem…';

  @override
  String get repeatTitle => 'Repetir';

  @override
  String get repeatKindDaily => 'Todo dia';

  @override
  String get repeatKindWeekly => 'Dias escolhidos';

  @override
  String get repeatKindInterval => 'A cada N minutos';

  @override
  String get repeatKindOnce => 'Uma vez';

  @override
  String get repeatTimeLabel => 'Às';

  @override
  String get repeatDateLabel => 'Em';

  @override
  String get repeatPickDate => 'Escolha uma data';

  @override
  String repeatDateValue(String day, String month) {
    return '$day/$month';
  }

  @override
  String get repeatEveryHint => 'A cada';

  @override
  String get repeatEverySuffix => 'minutos';

  @override
  String get repeatWindowLabel => 'Entre';

  @override
  String get repeatLabelDaily => 'Todo dia';

  @override
  String get repeatLabelWeekdays => 'Dias úteis';

  @override
  String get repeatLabelWeekends => 'Fins de semana';

  @override
  String repeatLabelWithin(String repeat, String from, String to) {
    return '$repeat · $from–$to';
  }

  @override
  String repeatLabelOnce(String date) {
    return 'Uma vez · $date';
  }

  @override
  String get dayMonShort => 'S';

  @override
  String get dayTueShort => 'T';

  @override
  String get dayWedShort => 'Q';

  @override
  String get dayThuShort => 'Q';

  @override
  String get dayFriShort => 'S';

  @override
  String get daySatShort => 'S';

  @override
  String get daySunShort => 'D';

  @override
  String get dayMon => 'Segunda-feira';

  @override
  String get dayTue => 'Terça-feira';

  @override
  String get dayWed => 'Quarta-feira';

  @override
  String get dayThu => 'Quinta-feira';

  @override
  String get dayFri => 'Sexta-feira';

  @override
  String get daySat => 'Sábado';

  @override
  String get daySun => 'Domingo';

  @override
  String get dayMonAbbr => 'Seg';

  @override
  String get dayTueAbbr => 'Ter';

  @override
  String get dayWedAbbr => 'Qua';

  @override
  String get dayThuAbbr => 'Qui';

  @override
  String get dayFriAbbr => 'Sex';

  @override
  String get daySatAbbr => 'Sáb';

  @override
  String get daySunAbbr => 'Dom';

  @override
  String get quietTitle => 'Horário silencioso';

  @override
  String get quietHelp => 'Nenhuma mensagem entre estes horários.';

  @override
  String get formErrorNoVoice => 'Escolha uma voz primeiro.';

  @override
  String get formErrorNoTheme => 'Escolha um tema.';

  @override
  String get formErrorNoDays => 'Escolha pelo menos um dia.';

  @override
  String get formErrorInterval =>
      'Informe quantos minutos entre as mensagens (1 ou mais).';

  @override
  String get formErrorNoDate => 'Escolha a data.';

  @override
  String get voicesTitle => 'Vozes';

  @override
  String get voicesEmpty =>
      'Nenhuma voz ainda.\nToque em + para adicionar a voz de quem você ama.';

  @override
  String get voicesAdd => 'Nova voz';

  @override
  String get voiceStatusReady => 'Voz pronta';

  @override
  String get voiceStatusNotCreated => 'Voz ainda não criada';

  @override
  String samplesCountAndSecs(String recordings, String duration) {
    return '$recordings · $duration';
  }

  @override
  String get importVoiceTitleNew => 'Nova voz';

  @override
  String get importVoiceTitleEdit => 'Voz';

  @override
  String get voiceNameHint => 'Nome (ex.: Vovó Lúcia)';

  @override
  String get voiceNameRequired => 'Informe o nome.';

  @override
  String get voiceConsentHint => 'Quem autorizou o uso desta voz';

  @override
  String get voiceConsentHelp =>
      'Obrigatório: só use uma voz com a permissão da pessoa.';

  @override
  String get voiceConsentRequired => 'Informe quem autorizou o uso desta voz.';

  @override
  String get voiceLanguageHint => 'Idioma';

  @override
  String get voiceListenerHint => 'Como esta voz chama você';

  @override
  String get voiceListenerHelp =>
      'Opcional, ex.: um apelido. Vazio: seu nome dos Ajustes.';

  @override
  String get voiceRegistered => 'Voz salva. Agora adicione gravações.';

  @override
  String get voiceSaved => 'Salvo.';

  @override
  String get voiceCreated => 'Voz criada! Já está pronta para seus lembretes.';

  @override
  String get samplesTitle => 'Gravações';

  @override
  String get samplesAddFiles => 'Adicionar áudios';

  @override
  String get samplesRecord => 'Gravar';

  @override
  String get samplesStop => 'Parar de gravar';

  @override
  String samplesAdded(int count) {
    return 'Gravações adicionadas: $count';
  }

  @override
  String sampleTitle(int number) {
    return 'Gravação $number';
  }

  @override
  String sampleDuration(String secs) {
    return '$secs s';
  }

  @override
  String recordingStatus(String elapsed, String limit) {
    return 'Gravando… $elapsed de $limit';
  }

  @override
  String get recordPermissionDenied =>
      'Permita o microfone para gravar uma voz.';

  @override
  String get actionCancel => 'Cancelar';

  @override
  String get deleteVoice => 'Excluir voz';

  @override
  String get deleteVoiceTitle => 'Excluir esta voz?';

  @override
  String get deleteVoiceMessage =>
      'As gravações e a voz criada são apagadas deste celular e do servidor de voz. Os lembretes que a usam ficam desligados e guardados, para você escolher outra voz.';

  @override
  String get deleteVoiceConfirm => 'Excluir';

  @override
  String get voiceDeleted => 'Voz excluída.';

  @override
  String get createVoice => 'Criar voz';

  @override
  String get createVoiceRunning => 'Criando a voz…';

  @override
  String get createVoiceHint =>
      'Adicione pelo menos 10 segundos de gravações (WAV, MP3 ou FLAC, de 2 s a 3 min cada) e depois crie a voz.';

  @override
  String get createVoiceHintReady =>
      'A voz está pronta. Adicionou mais gravações? Crie de novo para melhorá-la.';

  @override
  String get settingsTitle => 'Ajustes';

  @override
  String get settingsListenerHint => 'Seu nome';

  @override
  String get settingsListenerHelp =>
      'Usado nas mensagens quando a voz não tem um apelido para você.';

  @override
  String get settingsEngineHint => 'Endereço do servidor de voz';

  @override
  String get settingsSaved => 'Ajustes salvos.';

  @override
  String get settingsLanguageHint => 'Idioma do app';

  @override
  String get languageSystemDefault => 'Padrão do sistema';

  @override
  String get channelName => 'Mensagens agendadas';

  @override
  String get notificationTitle => 'Uma mensagem para você';

  @override
  String notificationBody(String text, String tone) {
    return '$text\n(tom: $tone)';
  }

  @override
  String pitchHigher(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: 'Altura: $count semitons mais aguda',
      many: 'Altura: $count semitons mais aguda',
      one: 'Altura: $count semitom mais aguda',
    );
    return '$_temp0';
  }

  @override
  String pitchLower(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: 'Altura: $count semitons mais grave',
      many: 'Altura: $count semitons mais grave',
      one: 'Altura: $count semitom mais grave',
    );
    return '$_temp0';
  }

  @override
  String repeatLabelEvery(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: 'A cada $count min',
      many: 'A cada $count min',
      one: 'A cada minuto',
    );
    return '$_temp0';
  }

  @override
  String samplesCount(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: '$count gravações',
      many: '$count gravações',
      one: '$count gravação',
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
  String get deleteReminder => 'Apagar lembrete';

  @override
  String get samplesFillFirst =>
      'Preencha o nome e quem autorizou, depois adicione gravações.';
}

/// The translations for Portuguese, as used in Portugal (`pt_PT`).
class AppLocalizationsPtPt extends AppLocalizationsPt {
  AppLocalizationsPtPt() : super('pt_PT');

  @override
  String get errorGeneric => 'Algo correu mal. Tente novamente.';

  @override
  String get errorPlayback => 'Não foi possível reproduzir a mensagem.';

  @override
  String get errorFileOpen => 'Não foi possível abrir o ficheiro.';

  @override
  String get errorNoAudioTrack =>
      'Este ficheiro não tem áudio. Use um ficheiro WAV, MP3 ou FLAC.';

  @override
  String get errorAudioFormat =>
      'Este formato de áudio não é suportado. Converta o ficheiro para WAV.';

  @override
  String get errorAudioTooLong =>
      'Esta gravação tem mais de 3 minutos: divida-a em partes.';

  @override
  String get errorRecordingFailed => 'A gravação falhou. Tente novamente.';

  @override
  String get tabSettings => 'Definições';

  @override
  String get reminderNoVoices =>
      'Ainda não há vozes prontas. Crie uma primeiro no separador Vozes.';

  @override
  String get pitchNatural => 'Tom: natural';

  @override
  String get pitchDescription => 'Ajuste de tom em semitons';

  @override
  String get reminderPlayNow => 'Ouvir já';

  @override
  String get reminderPlaying => 'A preparar a mensagem…';

  @override
  String get repeatKindDaily => 'Todos os dias';

  @override
  String get repeatPickDate => 'Escolha uma data';

  @override
  String get repeatLabelDaily => 'Todos os dias';

  @override
  String get voicesEmpty =>
      'Ainda não há vozes.\nToque em + para adicionar a voz de quem ama.';

  @override
  String get voiceNameHint => 'Nome (ex.: Avó Lúcia)';

  @override
  String get voiceListenerHint => 'Como esta voz o trata';

  @override
  String get voiceListenerHelp =>
      'Opcional, ex.: uma alcunha. Vazio: o seu nome das Definições.';

  @override
  String get voiceRegistered => 'Voz guardada. Agora adicione gravações.';

  @override
  String get voiceSaved => 'Guardado.';

  @override
  String get samplesAddFiles => 'Adicionar ficheiros de áudio';

  @override
  String recordingStatus(String elapsed, String limit) {
    return 'A gravar… $elapsed de $limit';
  }

  @override
  String get recordPermissionDenied =>
      'Permita o acesso ao microfone para gravar uma voz.';

  @override
  String get createVoiceRunning => 'A criar a voz…';

  @override
  String get settingsTitle => 'Definições';

  @override
  String get settingsListenerHint => 'O seu nome';

  @override
  String get settingsListenerHelp =>
      'Usado nas mensagens quando a voz não tem uma alcunha para si.';

  @override
  String get settingsSaved => 'Definições guardadas.';

  @override
  String get settingsLanguageHint => 'Idioma da aplicação';

  @override
  String get languageSystemDefault => 'Predefinição do sistema';

  @override
  String get notificationTitle => 'Uma mensagem para si';

  @override
  String pitchHigher(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: 'Tom: $count semitons mais agudo',
      many: 'Tom: $count semitons mais agudo',
      one: 'Tom: $count semitom mais agudo',
    );
    return '$_temp0';
  }

  @override
  String pitchLower(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: 'Tom: $count semitons mais grave',
      many: 'Tom: $count semitons mais grave',
      one: 'Tom: $count semitom mais grave',
    );
    return '$_temp0';
  }
}
