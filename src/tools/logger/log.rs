use crate::tools::logger::time::time_now;
use colored::Colorize;

#[allow(dead_code)]
pub enum LogLevel<'a> {
    // blue dimmed
    Info(&'a str),
    // green italic
    Success(&'a str),
    // yellow underline
    Warn(&'a str),
    // red bold
    Error(&'a str),
}

#[macro_export]
macro_rules! debug_info {
    () => {
        std::format!(
            "LINE: [{}] COL: [{}] FILE: [{}]",
            std::line!(),
            std::column!(),
            std::file!()
        )
    };
}

pub fn log(info: LogLevel) {
    let debug = cfg!(debug_assertions);
    let now = time_now();
    let mut error = false;
    let mut result = match info {
        LogLevel::Info(l) => {
            std::format!("{} {} {}", "[INFO]", now, l.replace("\n", "\n[INFO] ")).blue()
        }
        LogLevel::Success(l) => {
            std::format!("{} {} {}", "[SUCC]", now, l.replace("\n", "\n[SUCC] ")).green()
        }
        LogLevel::Warn(l) => std::format!("{} {} {}", "[WARN]", now, l.replace("\n", "\n[WARN] "))
            .yellow()
            .underline(),
        LogLevel::Error(l) => {
            error = true;
            std::format!("{} {} {}", "[ERROR]", now, l.replace("\n", "\n[ERROR] "))
                .red()
                .bold()
        }
    };
    if !debug {
        result = result.clear();
    }
    if error {
        eprintln!("{}", result);
    } else {
        println!("{}", result);
    }
}
