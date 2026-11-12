//! # Friendly Voice — Mobile
//!
//! Presentation layer for the Android app. Like the CLI, it only translates
//! inputs/outputs and delegates to the use cases (wired by `bootstrap`).
//!
//! * [`api`] — operations with simple inputs and JSON output
//!   (`{"ok":true,"data":...}` or `{"ok":false,"error":"..."}`), testable
//!   on the host, without Android.
//! * [`jni_bridge`] — `extern "system"` functions called from Kotlin
//!   (`app.friendlyvoice.NativeBridge`).
pub mod api;
mod jni_bridge;
