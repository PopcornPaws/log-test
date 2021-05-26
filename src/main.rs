mod logger;
use logger::SimpleLogger;

use log::LevelFilter;
use log::{debug, error, info, trace, warn};

use std::env;

fn main() {
    init_logger();
    trace!("this is a trace message");
    debug!("this is a debug message");
    info!("this is an info message");
    warn!("this is a warn message");
    error!("this is an error message");
}

#[cfg(all(feature = "production", feature = "SGX_MODE_HW"))]
fn init_logger() {
    log::set_logger(&SimpleLogger).unwrap();
    set_log_level_or_default(LevelFilter::Error, LevelFilter::Warn);
}

#[cfg(all(not(feature = "production"), not(feature = "test")))]
fn init_logger() {
    log::set_logger(&SimpleLogger).unwrap();
    set_log_level_or_default(LevelFilter::Trace, LevelFilter::Trace);
}

fn log_level_from_str(env_log_level: &str) -> Option<LevelFilter> {
    match env_log_level {
        "OFF" => Some(LevelFilter::Off),
        "ERROR" => Some(LevelFilter::Error),
        "WARN" => Some(LevelFilter::Warn),
        "INFO" => Some(LevelFilter::Info),
        "DEBUG" => Some(LevelFilter::Debug),
        "TRACE" => Some(LevelFilter::Trace),
        _ => None,
    }
}

fn set_log_level_or_default(default: LevelFilter, max_level: LevelFilter) {
    if default > max_level {
        panic!(
            "Logging configuration is broken, stopping to prevent secret leaking. default: {:?}, max level: {:?}",
            default, max_level
        );
    }

    let mut log_level = default;

    if let Some(env_log_level) = log_level_from_str(&env::var("LOG_LEVEL").unwrap_or_default()) {
        // We want to make sure log level is not higher than WARN in production to prevent accidental secret leakage
        if env_log_level <= max_level {
            log_level = env_log_level;
        }
    }
    log::set_max_level(log_level);
}

/*
#[cfg(feature = "test")]
pub mod logging_tests {
    use crate::{count_failures, set_log_level_or_default};
    use ctor::*;
    use lazy_static::lazy_static;
    use log::*;
    use log::{Metadata, Record};
    use std::sync::SgxMutex;
    use std::{env, panic};

    lazy_static! {
        static ref LOG_BUF: SgxMutex<Vec<String>> = SgxMutex::new(Vec::new());
    }
    pub struct TestLogger;
    impl log::Log for TestLogger {
        fn enabled(&self, _metadata: &Metadata) -> bool {
            true
        }
        fn log(&self, record: &Record) {
            LOG_BUF.lock().unwrap().push(format!(
                "{}  [{}] {}",
                record.level(),
                record.target(),
                record.args()
            ));
        }
        fn flush(&self) {}
    }

    #[ctor]
    fn init_logger_test() {
        log::set_logger(&TestLogger).unwrap();
    }

    pub fn run_tests() {
        println!();
        let mut failures = 0;

        count_failures!(failures, {
            test_log_level();
            test_log_default_greater_than_max();
        });

        if failures != 0 {
            panic!("{}: {} tests failed", file!(), failures);
        }
    }

    fn test_log_level() {
        env::set_var("LOG_LEVEL", "WARN");
        set_log_level_or_default(LevelFilter::Error, LevelFilter::Info);
        assert_eq!(log::max_level(), LevelFilter::Warn);
        info!("Should not process");
        assert!(LOG_BUF.lock().unwrap().is_empty());

        env::set_var("LOG_LEVEL", "TRACE");
        set_log_level_or_default(LevelFilter::Error, LevelFilter::Info);
        assert_eq!(log::max_level(), LevelFilter::Error);
        debug!("Should not process");
        assert!(LOG_BUF.lock().unwrap().is_empty());

        env::set_var("LOG_LEVEL", "WARN");
        set_log_level_or_default(LevelFilter::Warn, LevelFilter::Warn);
        assert_eq!(log::max_level(), LevelFilter::Warn);
        trace!("Should not process");
        assert!(LOG_BUF.lock().unwrap().is_empty());

        warn!("This should process");
        assert_eq!(LOG_BUF.lock().unwrap().len(), 1);
    }

    fn test_log_default_greater_than_max() {
        let result = panic::catch_unwind(|| {
            set_log_level_or_default(LevelFilter::Trace, LevelFilter::Error);
        });
        assert!(result.is_err());
    }
}
*/
