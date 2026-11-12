//! Conversion of textual arguments into domain values.
use chrono::{NaiveDateTime, NaiveTime, Weekday};
use friendly_voice_domain::schedule::TimeWindow;

pub fn time(s: &str) -> Result<NaiveTime, String> {
    NaiveTime::parse_from_str(s.trim(), "%H:%M")
        .map_err(|_| format!("invalid time '{s}' (use HH:MM)"))
}

pub fn window(s: &str) -> Result<TimeWindow, String> {
    let (a, b) = s
        .split_once('-')
        .ok_or_else(|| format!("invalid time window '{s}' (use HH:MM-HH:MM)"))?;
    TimeWindow::new(time(a)?, time(b)?).map_err(|e| e.to_string())
}

pub fn weekday(s: &str) -> Result<Weekday, String> {
    let d = match s.trim().to_lowercase().as_str() {
        "seg" | "segunda" | "mon" | "monday" => Weekday::Mon,
        "ter" | "terca" | "terça" | "tue" | "tuesday" => Weekday::Tue,
        "qua" | "quarta" | "wed" | "wednesday" => Weekday::Wed,
        "qui" | "quinta" | "thu" | "thursday" => Weekday::Thu,
        "sex" | "sexta" | "fri" | "friday" => Weekday::Fri,
        "sab" | "sáb" | "sabado" | "sábado" | "sat" | "saturday" => Weekday::Sat,
        "dom" | "domingo" | "sun" | "sunday" => Weekday::Sun,
        other => {
            return Err(format!(
                "invalid weekday '{other}' (use mon, tue, wed, thu, fri, sat, sun)"
            ))
        }
    };
    Ok(d)
}

/// "seg,qua,sex@07:30"
pub fn weekly(s: &str) -> Result<(Vec<Weekday>, NaiveTime), String> {
    let (days, at) = s
        .split_once('@')
        .ok_or_else(|| format!("use days@HH:MM, e.g. mon,wed@07:30 (got '{s}')"))?;
    let days = days
        .split(',')
        .map(weekday)
        .collect::<Result<Vec<_>, _>>()?;
    Ok((days, time(at)?))
}

pub fn datetime(s: &str) -> Result<NaiveDateTime, String> {
    NaiveDateTime::parse_from_str(s.trim(), "%Y-%m-%d %H:%M")
        .map_err(|_| format!("invalid date '{s}' (use YYYY-MM-DD HH:MM)"))
}

pub fn weekday_short(d: Weekday) -> &'static str {
    match d {
        Weekday::Mon => "mon",
        Weekday::Tue => "tue",
        Weekday::Wed => "wed",
        Weekday::Thu => "thu",
        Weekday::Fri => "fri",
        Weekday::Sat => "sat",
        Weekday::Sun => "sun",
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
        assert_eq!(
            weekly("mon,Fri@08:00").unwrap().0,
            vec![Weekday::Mon, Weekday::Fri]
        );
        assert!(weekly("seg").is_err());
        assert!(datetime("2026-11-20 09:00").is_ok());
        assert!(time("25:00").is_err());
    }
}
