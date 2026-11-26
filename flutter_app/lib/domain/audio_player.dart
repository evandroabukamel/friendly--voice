/// Plays a generated message (a WAV file written by the core).
abstract interface class AudioPlayer {
  Future<void> play(String path);
}
