//! Functions exposed to Dart by flutter_rust_bridge. Like the CLI, this is a
//! presentation layer: it only translates inputs/outputs and delegates to the
//! use cases wired by `friendly-voice-bootstrap`.
pub mod catalog;
pub mod failure;
pub mod messages;
pub mod session;
pub mod speakers;
pub mod voice;

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}
