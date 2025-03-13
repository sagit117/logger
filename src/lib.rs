#![crate_type = "lib"]
#![crate_name = "logger"]

use std::fmt::{self, write};

pub enum LogLevel {
    DEBUG = 10,
    INFO = 20,
    WARNING = 30,
    ERROR = 40,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            LogLevel::DEBUG => write(f, format_args!("DEBUG")),
            LogLevel::INFO => write(f, format_args!("INFO")),
            LogLevel::WARNING => write(f, format_args!("WARNING")),
            LogLevel::ERROR => write(f, format_args!("ERROR")),
        }
    }
}

pub struct Logger<'a> {
    pub log_level: &'a LogLevel,
    pub formatter: fn (&str, level: &LogLevel) -> String,
}

impl Logger<'_>  {
    pub fn log(&self, message: &str, level: &LogLevel) {
        if get_log_level_number(level) >= get_log_level_number(self.log_level) {
            let format_message = (self.formatter)(message, level);
            println!("{}", format_message);
        }
    }

    pub fn info(&self, message: &str) {
        self.log(message, &LogLevel::INFO);
    }

    pub fn debug(&self, message: &str) {
        self.log(message, &LogLevel::DEBUG);
    }

    pub fn warning(&self, message: &str) {
        self.log(message, &LogLevel::WARNING);
    }

    pub fn error(&self, message: &str) {
        self.log(message, &LogLevel::ERROR);
    }
} 

fn get_log_level_number(log_level: &LogLevel) -> u8 {
    match log_level {
        LogLevel::DEBUG => 10,
        LogLevel::INFO => 20,
        LogLevel::WARNING => 30,
        LogLevel::ERROR => 40,
    }
}

pub fn formatter(message: &str, level: &LogLevel) -> String {
    format!("[{: <7}] {}", level.to_string(), message)
}