//! # Friendly Voice — Mobile
//!
//! Camada de apresentação para o app Android. Assim como a CLI, ela só traduz
//! entradas/saídas e delega aos casos de uso (montados pelo `bootstrap`).
//!
//! * [`api`] — operações com entrada simples e saída JSON
//!   (`{"ok":true,"data":...}` ou `{"ok":false,"error":"..."}`), testáveis
//!   no host, sem Android.
//! * [`jni_bridge`] — funções `extern "system"` chamadas pelo Kotlin
//!   (`app.friendlyvoice.NativeBridge`).
pub mod api;
mod jni_bridge;
