//! 全局变量

use std::sync::OnceLock;

#[derive(Debug)]
pub struct Logger;

static LOGGER: OnceLock<Logger> = OnceLock::new();

impl Logger {
    pub fn global() -> &'static Logger {
        LOGGER.get_or_init(|| {
            println!("Logger is being created...");
            Logger
        })
    }
    pub fn log(&self, message: String) {
        println!("{}", message);
    }
}

#[cfg(test)]
mod tests {
    use std::thread;

    use super::Logger;

    #[test]
    fn test_logger() {
        let handle = thread::spawn(|| {
            let loger = Logger::global();
            loger.log("thread message ".to_string());
        });

        let logger = Logger::global();
        logger.log("main message".to_string());

        let logger2 = Logger::global();
        logger2.log("ohter message".to_string());
        handle.join().unwrap()
    }
}
