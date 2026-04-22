use my_library::config::{LogLevel, LogOutput, Logging};

#[test]
fn test_logging_constructor() {
    let logging: Logging = Logging::new();
    let aux: Logging = Logging { enabled: false, level: LogLevel::Info, destination: LogOutput::Stdout };
    assert_eq!(logging.enabled, false);
    assert_eq!(logging.level, LogLevel::Info);
    assert_eq!(logging.destination, LogOutput::Stdout);
    assert_eq!(logging, aux);
}

#[test]
fn test_logging_enabled() {
    let mut logging: Logging = Logging::new();
    assert_ne!(logging.enabled, true);
    logging.enable();
    assert_eq!(logging.enabled, true);
}