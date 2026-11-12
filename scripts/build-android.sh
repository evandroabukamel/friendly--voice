#!/usr/bin/env bash
# Builds the Android app:
#   1. the Rust library (crates/mobile) for each ABI, with cargo-ndk;
#   2. the APK with Gradle (android/).
#
# Usage: scripts/build-android.sh [--rust-only] [--release] [--no-http-engine]
# Requires: ANDROID_NDK_HOME, cargo-ndk (cargo install cargo-ndk), rustup targets
#           aarch64-linux-android, armv7-linux-androideabi, x86_64-linux-android;
#           for the APK: Android SDK (ANDROID_HOME) and Gradle 8.7+.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ABIS=(arm64-v8a armeabi-v7a x86_64)
MIN_SDK=26
RUST_ONLY=false
GRADLE_TASK=assembleDebug
FEATURES=(--features http-engine)

for arg in "$@"; do
  case "$arg" in
    --rust-only) RUST_ONLY=true ;;
    --release) GRADLE_TASK=assembleRelease ;;
    --no-http-engine) FEATURES=() ;;
    *) echo "unknown argument: $arg" >&2; exit 2 ;;
  esac
done

: "${ANDROID_NDK_HOME:?set ANDROID_NDK_HOME to the Android NDK path}"
command -v cargo-ndk >/dev/null || { echo "install it: cargo install cargo-ndk" >&2; exit 1; }

targets=()
for abi in "${ABIS[@]}"; do targets+=(-t "$abi"); done

echo "==> Native library (${ABIS[*]})"
(cd "$ROOT" && cargo ndk "${targets[@]}" --platform "$MIN_SDK" \
  -o android/app/src/main/jniLibs \
  build --release -p friendly-voice-mobile "${FEATURES[@]}")

$RUST_ONLY && exit 0

echo "==> APK ($GRADLE_TASK)"
if [[ -x "$ROOT/android/gradlew" ]]; then
  (cd "$ROOT/android" && ./gradlew "$GRADLE_TASK")
else
  gradle -p "$ROOT/android" "$GRADLE_TASK"
fi
find "$ROOT/android/app/build/outputs/apk" -name '*.apk' -print
