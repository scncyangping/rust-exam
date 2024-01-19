//! atomic 源子类测试
use std::ops::Sub;
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread as _thread;
use std::time::Instant;

/// 测试10个线程每个线程新增1000000次
/// Ordering::Relaxed
/// 用于控制原子操作使用的内存顺序
pub fn atomic_add() {
    const N_TIMES: u64 = 1000000;
    const N_THREADS: usize = 10;
    static R: AtomicU64 = AtomicU64::new(0);
    fn add_n_times(n: u64) -> std::thread::JoinHandle<()> {
        _thread::spawn(move || {
            for _ in 0..n {
                R.fetch_add(1, Ordering::Relaxed);
            }
        })
    }

    let s = Instant::now();
    let mut threads = Vec::with_capacity(N_THREADS);
    for _ in 0..N_THREADS {
        threads.push(add_n_times(N_TIMES))
    }
    for thread in threads {
        thread.join().unwrap();
    }
    println!("{}, {:?}", R.load(Ordering::Relaxed), Instant::now().sub(s))
}
