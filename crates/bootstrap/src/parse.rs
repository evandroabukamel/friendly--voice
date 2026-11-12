//! Conversion of textual arguments into domain values.
use chrono::{NaiveDateTime, NaiveTime, Weekday};
use friendly_voice_domain::schedule::TimeWindow;

pub fn time(s: &str) -> Result<NaiveTime, String> {
    NaiveTime::parse_from_str(s.trim(), "%H:%M")
        .map_err(|_| format!("horário inválido '{s}' (use HH:MM)"))
}

pub fn window(s: &str) -> Result<TimeWindow, String> {
    let (a, b) = s
        .split_once('-')
        .ok_or_else(|| format!("janela inválida '{s}' (use HH:MM-HH:MM)"))?;
    TimeWindow::new(time(a)?, time(b)?).map_err(|e| e.to_string())
}

pub fn weekday(s: &str) -> Result<Weekday, String> {
    let d = match s.trim().to_lowercase().as_str() {
        "seg" | "segunda" => Weekday::Mon,
        "ter" | "terca" | "terça" => Weekday::Tue,
        "qua" | "quarta" => Weekday::Wed,
        "qui" | "quinta" => Weekday::Thu,
        "sex" | "sexta" => Weekday::Fri,
        "sab" | "sáb" | "sabado" | "sábado" => Weekday::Sat,
        "dom" | "domingo" => Weekday::Sun,
        other => {
            return Err(format!(
                "dia inválido '{other}' (use seg, ter, qua, qui, sex, sab, dom)"
            ))
        }
    };
    Ok(d)
}

/// "seg,qua,sex@07:30"
pub fn weekly(s: &str) -> Result<(Vec<Weekday>, NaiveTime), String> {
    let (days, at) = s
        .split_once('@')
        .ok_or_else(|| format!("use dias@HH:MM, ex.: seg,qua@07:30 (recebi '{s}')"))?;
    let days = days
        .split(',')
        .map(weekday)
        .collect::<Result<Vec<_>, _>>()?;
    Ok((days, time(at)?))
}

pub fn datetime(s: &str) -> Result<NaiveDateTime, String> {
    NaiveDateTime::parse_from_str(s.trim(), "%Y-%m-%d %H:%M")
        .map_err(|_| format!("data inválida '{s}' (use AAAA-MM-DD HH:MM)"))
}

pub fn weekday_pt(d: Weekday) -> &'static str {
    match d {
        Weekday::Mon => "seg",
        Weekday::Tue => "ter",
        Weekday::Wed => "qua",
        Weekday::Thu => "qui",
        Weekday::Fri => "sex",
        Weekday::Sat => "sáb",
        Weekday::Sun => "dom",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_rules() {
        let (days, at) = weekly("seg,QUA,sexta@07:30").unwrap();
        assert_eq!(days, vec![Weekday::Mon, Weekday::Wed, Weekday::Fri]);
        assert_eq!(at, time("07:30").unwrap());
        assert!(window("22:00-07:00")
            .unwrap()
            .contains(time("23:00").unwrap()));
        assert!(weekly("seg").is_err());
        assert!(datetime("2026-11-20 09:00").is_ok());
        assert!(time("25:00").is_err());
    }
}
