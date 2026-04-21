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
/// Getting the fields
/// ```
/// use my_library::config::Logging;
/// let config = Logging::new();
/// config.is_enabled();
/// config.get_level();
/// config.get_destination();
/// ```
impl Logging {
    pub fn new() -> Self {
        Self {
            enabled: false,
            level: LogLevel::Info,
            destination: LogOutput::Stdout,
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn get_level(&self) -> &LogLevel {
        &self.level
    }

    pub fn get_destination(&self) -> &LogOutput {
        &self.destination
    }

    /// Enable and disable the Logging
    /// # Example:
    /// ```
    /// use my_library::config::Logging;
    /// let mut config = Logging::new();
    /// config.disable();
    /// config.enable();
    /// ```
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Set the Logging level and destination fields
    /// # Example:
    /// ```
    /// use my_library::config::{Logging, LogLevel, LogOutput};
    /// let mut config = Logging::new();
    /// let new_level = LogLevel::Debug;
    /// let new_destination = LogOutput::File("file.txt".to_string());
    /// config.set_level(new_level);
    /// config.set_destination(new_destination);
    /// ```
    pub fn set_level(&mut self, new_level: LogLevel) {
        self.level = new_level;
    }

    pub fn set_destination(&mut self, new_destination: LogOutput) {
        self.destination = new_destination;
    }

}