//! A session owns the wired use cases for one data folder and engine. Dart
//! keeps the session object; there is no global mutable state.
use super::failure::AppFailure;
use flutter_rust_bridge::frb;
use friendly_voice_bootstrap::Container;
use std::path::Path;

#[frb(opaque)]
pub struct Session {
    pub(crate) container: Container,
}

/// Blank optional text means "not provided".
pub(crate) fn non_empty(s: Option<String>) -> Option<String> {
    s.map(|s| s.trim().to_string()).filter(|s| !s.is_empty())
}

impl Session {
    /// Opens the app's data folder (e.g. the platform's application support
    /// directory) with the neural engine at `engine_url`, or the local
    /// sketch engine when no URL is given.
    #[frb(sync)]
    pub fn open(data_dir: String, engine_url: Option<String>) -> Result<Session, AppFailure> {
        let url = non_empty(engine_url);
        let container =
            Container::new(Path::new(&data_dir), url.as_deref(), None).map_err(AppFailure::new)?;
        Ok(Session { container })
    }

    #[frb(sync, getter)]
    pub fn engine_name(&self) -> String {
        self.container.engine_name.to_string()
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use std::path::PathBuf;

    /// A fresh data folder per test: sessions never share state.
    pub(crate) fn temp_dir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "fv-bridge-{name}-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        let _ = std::fs::remove_dir_all(&dir);
        dir
    }

    pub(crate) fn local_session(name: &str) -> Session {
        Session::open(temp_dir(name).display().to_string(), None).unwrap()
    }

    #[test]
    fn opens_with_the_local_engine_when_no_url_is_given() {
        let s = local_session("local");
        assert_eq!(s.engine_name(), "local (prosody sketch)");
        let blank = Session::open(temp_dir("blank").display().to_string(), Some("  ".into()));
        assert_eq!(blank.unwrap().engine_name(), "local (prosody sketch)");
    }

    #[test]
    fn opens_with_the_neural_engine_when_a_url_is_given() {
        let s = Session::open(
            temp_dir("http").display().to_string(),
            Some("http://127.0.0.1:8020".into()),
        )
        .unwrap();
        assert_eq!(s.engine_name(), "neural server (HTTP)");
    }
}
