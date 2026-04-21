//! This module contains the configuration options for the application.
//! # Examples:
//! ```
//! use my_library::config::Logging;
//! let config = Logging::new();
//! ```
//! 
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

pub enum LogOutput {
    Stdout,
    Stderr,
    File(String),
}

/// This struct contains configuration options for the application.
/// # Examples:
/// ```
/// use my_library::config::Logging;
/// let config = Logging::new();
/// ```
/// 
/// Creating a new instance of the Logging struct:
/// ```
/// use my_library::config::{Logging, LogLevel, LogOutput};
/// let config = Logging{ enabled: true, level: LogLevel::Info, destination: LogOutput::Stdout };
/// ```
/// 
/// Modifying the Logging struct:
/// ```
/// use my_library::config::{Logging, LogLevel, LogOutput};
/// let mut config = Logging::new();
/// config.enabled = true;
/// config.level = LogLevel::Debug;
/// config.destination = LogOutput::File(String::from("log.txt"));
/// ```

pub struct Logging {
    pub enabled: bool,
    pub level: LogLevel,
    pub destination: LogOutput,   
}

/// This implementation block provides a constructor for the Logging struct.
/// # Examples:
/// ```
/// use my_library::config::Logging;
/// let config = Logging::new();
/// ```
impl Logging {
    pub fn new() -> Self {
        Self {
            enabled: false,
            level: LogLevel::Info,
            destination: LogOutput::Stdout,
        }
    }
}