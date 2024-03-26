use std::thread;

use stu::bianliang::Logger;

fn main() {
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
