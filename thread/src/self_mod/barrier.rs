//! barrier
//! 让多个线程执行到某个点后
//! 才继续一起执行

use std::sync::{Arc, Barrier};
use std::thread as _thread;

/// 等待所有线程一起执行
/// # Example
/// ```rust
/// thread::self_mod::barrier::wait_all_execute()
/// ```
pub fn wait_all_execute() {
    let mut handles = Vec::with_capacity(6);
    let barrier = Arc::new(Barrier::new(6));
    for _ in 0..6 {
        let b = barrier.clone();
        handles.push(_thread::spawn(move || {
            println!("before wait");
            b.wait();
            println!("after wait");
        }))
    }
    for handle in handles {
        handle.join().unwrap()
    }
}
