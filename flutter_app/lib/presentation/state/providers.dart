import 'dart:async';

import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:friendly_voice/domain/alarms.dart';
import 'package:friendly_voice/domain/app_settings.dart';
import 'package:friendly_voice/domain/audio_files.dart';
import 'package:friendly_voice/domain/audio_player.dart';
import 'package:friendly_voice/domain/voice_core.dart';
import 'package:friendly_voice/infrastructure/file_picker_audio_picker.dart';
import 'package:friendly_voice/infrastructure/local_notifications_alarm_scheduler.dart';
import 'package:friendly_voice/infrastructure/preferences_settings_store.dart';
import 'package:friendly_voice/infrastructure/rust_voice_core.dart';
import 'package:friendly_voice/infrastructure/soloud_audio_player.dart';
import 'package:friendly_voice/presentation/design/daylight.dart';
import 'package:path_provider/path_provider.dart';

/// Composition root of the app: the only place that names the concrete
/// adapters. Tests override these providers with fakes.
final settingsStoreProvider = Provider<SettingsStore>(
  (ref) => PreferencesSettingsStore(),
);

/// The voice server compiled into this build (`null` = local sketch engine).
final engineUrlProvider = Provider<String?>(
  (ref) => engineUrlFromBuild(
    const String.fromEnvironment('FRIENDLY_VOICE_ENGINE_URL'),
  ),
);

final voiceCoreFactoryProvider = Provider<VoiceCoreFactory>(
  (ref) => RustVoiceCoreFactory(dataDir: getApplicationSupportDirectory),
);

final audioPickerProvider = Provider<AudioFilePicker>(
  (ref) => FilePickerAudioPicker(),
);

final audioPlayerProvider = Provider<AudioPlayer>((ref) => SoloudAudioPlayer());

final alarmSchedulerProvider = Provider<AlarmScheduler>(
  (ref) => LocalNotificationsAlarmScheduler(),
);

/// How often due messages are checked while the app is open (`null` =
/// never; the OS alarms still fire).
final deliveryIntervalProvider = Provider<Duration?>(
  (ref) => const Duration(minutes: 1),
);

/// The wall clock (tests pin it).
final clockProvider = Provider<DateTime Function()>((ref) => DateTime.now);

/// How often the sky is repainted for the time of day (`null` = never).
final daylightTickProvider = Provider<Duration?>(
  (ref) => const Duration(minutes: 1),
);

/// The part of the day the screens are painted in, following the clock.
final daylightProvider = NotifierProvider<DaylightNotifier, Daylight>(
  DaylightNotifier.new,
);

class DaylightNotifier extends Notifier<Daylight> {
  @override
  Daylight build() {
    final now = ref.watch(clockProvider);
    final tick = ref.watch(daylightTickProvider);
    if (tick != null) {
      final timer = Timer.periodic(
        tick,
        (_) => state = Daylight.at(now().hour),
      );
      ref.onDispose(timer.cancel);
    }
    return Daylight.at(now().hour);
  }
}

/// The core (opened on the build's voice server) and the user's settings.
final class Connection {
  const new({required this.settings, required this.core});

  final AppSettings settings;
  final VoiceCore core;
}

final connectionProvider =
    AsyncNotifierProvider<ConnectionNotifier, Connection>(
      ConnectionNotifier.new,
    );

class ConnectionNotifier extends AsyncNotifier<Connection> {
  @override
  Future<Connection> build() async => Connection(
    settings: await ref.read(settingsStoreProvider).load(),
    core: await ref
        .read(voiceCoreFactoryProvider)
        .open(engineUrl: ref.read(engineUrlProvider)),
  );

  /// Saves the settings; the core stays open.
  Future<Connection> save(AppSettings settings) async {
    await ref.read(settingsStoreProvider).save(settings);
    final connection = Connection(
      settings: settings,
      core: (await future).core,
    );
    state = AsyncData(connection);
    return connection;
  }
}

/// The connected core; actions before the connection fail clearly.
VoiceCore requireCore(Ref ref) =>
    ref.read(connectionProvider).value?.core ??
    (throw const AppFailure(message: 'the voice engine is not connected yet'));

/// A plain notice for the user (shown without the warning style), e.g.
/// "Choose a voice first.".
final class Notice implements Exception {
  const new(this.message);

  final String message;
}

/// One message for the user, shown as a snackbar. Every message is a new
/// object, so the same text twice is shown twice.
final class UserMessage {
  new(this.text, {this.isError = false});

  final String text;
  final bool isError;
}

/// The last message for the user (`null` = none yet).
final messagesProvider = NotifierProvider<MessagesNotifier, UserMessage?>(
  MessagesNotifier.new,
);

class MessagesNotifier extends Notifier<UserMessage?> {
  @override
  UserMessage? build() => null;

  void show(String text, {bool isError = false}) =>
      state = UserMessage(text, isError: isError);

  /// Runs a user action: shows its result (`null` = nothing to say) or its
  /// failure, worded by the core. Returns whether it succeeded.
  Future<bool> run(Future<String?> Function() action) async {
    try {
      if (await action() case final text?) show(text);
      return true;
    } on Notice catch (n) {
      show(n.message);
    } on AppFailure catch (f) {
      show(f.message, isError: true);
    } on Exception catch (e) {
      show('$e', isError: true);
    }
    return false;
  }
}

enum AppTab { reminders, voices, settings }

/// The tab shown in the navigation bar.
final tabProvider = NotifierProvider<TabNotifier, AppTab>(TabNotifier.new);

class TabNotifier extends Notifier<AppTab> {
  @override
  AppTab build() => AppTab.reminders;

  AppTab get current => state;
  set current(AppTab tab) => state = tab;
}
