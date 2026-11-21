import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:friendly_voice/domain/app_settings.dart';
import 'package:friendly_voice/domain/voice_core.dart';
import 'package:friendly_voice/infrastructure/preferences_settings_store.dart';
import 'package:friendly_voice/infrastructure/rust_voice_core.dart';
import 'package:path_provider/path_provider.dart';

/// Composition root of the app: the only place that names the concrete
/// adapters. Tests override these providers with fakes.
final settingsStoreProvider = Provider<SettingsStore>(
  (ref) => PreferencesSettingsStore(),
);

final voiceCoreFactoryProvider = Provider<VoiceCoreFactory>(
  (ref) => RustVoiceCoreFactory(dataDir: getApplicationSupportDirectory),
);

/// The core opened with the current settings.
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
  Future<Connection> build() async {
    final settings = await ref.read(settingsStoreProvider).load();
    return await _open(settings);
  }

  Future<Connection> _open(AppSettings settings) async => Connection(
    settings: settings,
    core: await ref
        .read(voiceCoreFactoryProvider)
        .open(engineUrl: settings.engineUrl),
  );

  /// Saves the settings and reopens the core with them.
  Future<void> save(AppSettings settings) async {
    await ref.read(settingsStoreProvider).save(settings);
    state = AsyncData(await _open(settings));
  }
}

/// Result of the last user action, shown in the status line; `null` shows
/// the connection state instead.
final statusProvider = NotifierProvider<StatusNotifier, String?>(
  StatusNotifier.new,
);

class StatusNotifier extends Notifier<String?> {
  @override
  String? build() => null;

  /// Runs a user action the way the Kotlin app did: "Processing…" while it
  /// runs, then its success message or the failure with a warning sign.
  Future<void> run(Future<String?> Function() action) async {
    state = 'Processing…';
    try {
      state = await action();
    } on AppFailure catch (f) {
      state = '⚠️ ${f.message}';
    } on Exception catch (e) {
      state = '⚠️ $e';
    }
  }
}

/// The status line: last action, or the engine once connected.
final statusLineProvider = Provider<String>((ref) {
  final status = ref.watch(statusProvider);
  if (status != null) return status;
  return switch (ref.watch(connectionProvider)) {
    AsyncData(:final value) => 'Voice engine: ${value.core.engineName}',
    AsyncError(:final error) =>
      '⚠️ ${error is AppFailure ? error.message : error}',
    _ => 'Processing…',
  };
});
