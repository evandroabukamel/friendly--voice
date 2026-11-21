import 'dart:async';

import 'package:flutter_local_notifications/flutter_local_notifications.dart';
import 'package:friendly_voice/domain/alarms.dart';
import 'package:friendly_voice/domain/voice_core.dart';
import 'package:timezone/timezone.dart' as tz;

/// [AlarmScheduler] on flutter_local_notifications: exact alarms when the
/// user allowed them (Android 14+: SCHEDULE_EXACT_ALARM), otherwise
/// inexact while idle (may be a few minutes late).
final class LocalNotificationsAlarmScheduler implements AlarmScheduler {
  final _plugin = FlutterLocalNotificationsPlugin();
  final _taps = StreamController<void>.broadcast();
  Future<void>? _initialized;

  static const _details = NotificationDetails(
    android: AndroidNotificationDetails(
      'scheduled_messages',
      'Scheduled messages',
      importance: Importance.high,
    ),
    iOS: DarwinNotificationDetails(),
  );

  Future<void> _init() => _initialized ??= _plugin
      .initialize(
        settings: const InitializationSettings(
          android: AndroidInitializationSettings('@mipmap/ic_launcher'),
          iOS: DarwinInitializationSettings(
            requestAlertPermission: false,
            requestBadgePermission: false,
            requestSoundPermission: false,
          ),
        ),
        onDidReceiveNotificationResponse: (_) => _taps.add(null),
      )
      .then((_) {});

  AndroidFlutterLocalNotificationsPlugin? get _android => _plugin
      .resolvePlatformSpecificImplementation<
        AndroidFlutterLocalNotificationsPlugin
      >();

  @override
  Stream<void> get taps => _taps.stream;

  @override
  Future<void> requestPermissions() async {
    await _init();
    await _android?.requestNotificationsPermission();
    await _android?.requestExactAlarmsPermission();
    await _plugin
        .resolvePlatformSpecificImplementation<
          IOSFlutterLocalNotificationsPlugin
        >()
        ?.requestPermissions(alert: true, sound: true);
  }

  @override
  Future<void> replaceAll(List<Alarm> alarms) async {
    await _init();
    await _plugin.cancelAll();
    final exact = await _android?.canScheduleExactNotifications() ?? true;
    for (final (i, alarm) in alarms.indexed) {
      await _plugin.zonedSchedule(
        id: i,
        scheduledDate: tz.TZDateTime.fromMillisecondsSinceEpoch(
          tz.UTC,
          alarm.atEpochMs,
        ),
        notificationDetails: _details,
        androidScheduleMode: exact
            ? AndroidScheduleMode.exactAllowWhileIdle
            : AndroidScheduleMode.inexactAllowWhileIdle,
        title: 'A message for you 💛',
        body: 'Tap to hear it.',
        payload: alarm.scheduleId,
      );
    }
  }
}
