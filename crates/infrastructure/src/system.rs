use chrono::NaiveDateTime;
use friendly_voice_application::ports::{Clock, IdGenerator};
use std::sync::atomic::{AtomicU32, Ordering};

#[derive(Debug, Clone, Copy, Default)]
pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> NaiveDateTime {
        chrono::Local::now().naive_local()
    }
}

/// Ids curtos, ordenáveis por tempo: `<prefixo>-<millis base36><contador>`.
#[derive(Debug, Default)]
pub struct TimeIdGenerator {
    counter: AtomicU32,
}

fn base36(mut n: u128) -> String {
    const DIGITS: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyz";
    let mut s = Vec::new();
    loop {
        s.push(DIGITS[(n % 36) as usize]);
        n /= 36;
        if n == 0 {
            break;
        }
    }
    s.reverse();
    String::from_utf8(s).expect("ascii")
}

impl IdGenerator for TimeIdGenerator {
    fn next_id(&self, prefix: &str) -> String {
        let millis = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis())
            .unwrap_or_default();
        let c = self.counter.fetch_add(1, Ordering::Relaxed) % 1296;
        format!("{prefix}-{}{:0>2}", base36(millis), base36(c as u128))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids_are_unique() {
        let g = TimeIdGenerator::default();
        let a = g.next_id("x");
        let b = g.next_id("x");
        assert_ne!(a, b);
        assert!(a.starts_with("x-"));
    }
}
