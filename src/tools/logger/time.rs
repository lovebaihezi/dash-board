use chrono::{Datelike, Timelike, Utc};
pub fn time_now() -> String {
    let time = Utc::now();
    std::format!(
        "{}-{:02}-{:02} {:02}:{:02}:{:02}",
        time.year(),
        time.month(),
        time.day(),
        time.hour(),
        time.minute(),
        time.second(),
    )
}
