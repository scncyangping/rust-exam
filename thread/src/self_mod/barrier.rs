//! barrier
//! 让多个线程执行到某个点后
//! 才继续一起执行

use std::cell::RefCell;
use std::sync::{Arc, Barrier, Condvar, Mutex};
use std::time::Duration;
use std::{sync, thread as _thread};
use tokio::sync::Semaphore;

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

/// 线程本地变量
pub fn thread_local_param() {
    thread_local! (static FOO: RefCell<u32> = RefCell::new(1));
    FOO.with(|f| {
        assert_eq!(*f.borrow(), 1);
        *f.borrow_mut() = 2;
    });
    let t = _thread::spawn(move || {
        FOO.with(|f| {
            assert_eq!(*f.borrow(), 1);
            *f.borrow_mut() = 3;
        })
    });
    t.join().unwrap();
    FOO.with(|f| {
        assert_eq!(*f.borrow(), 2);
    });
}

/// condvar 条件判断
/// # 例
/// ```
/// thread::self_mod::barrier::thread_condvar()
/// ```
pub fn thread_condvar() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();

    _thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut s = lock.lock().unwrap();
        *s = true;
        cvar.notify_one();
    });

    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        started = cvar.wait(started).unwrap();
    }
    println!("started changed");
}

/// sync::Once
/// 只调用一次
/// # 例
/// ```
/// let once = sync::Once::new()
/// once.call_once(||{})
/// ```
pub fn thread_once() {
    static mut VAL: usize = 0;
    static INIT: sync::Once = sync::Once::new();

    let handle1 = _thread::spawn(move || {
        INIT.call_once(|| unsafe {
            VAL = 1;
        });
    });

    let handle2 = _thread::spawn(move || {
        INIT.call_once(|| unsafe {
            VAL = 2;
        });
    });
    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("{}", unsafe { VAL })
}

/// 同步发送
/// 若容量为0,则会阻塞知道接收端接收才发送
/// 若容量大于0,则会直接发送,知道容量被占满
/// # 例
/// ```
/// let (tx, rx) = sync::mpsc::sync_channel(1);
/// ```
pub fn async_send() {
    let (tx, rx) = sync::mpsc::sync_channel(1);

    let handle = _thread::spawn(move || {
        println!("发送之前");
        tx.send(1).unwrap();
        println!("发送之后");
        println!("发送之前2");
        tx.send(1).unwrap();
        println!("发送之后2");
    });

    println!("睡眠之前");
    _thread::sleep(std::time::Duration::from_secs(3));
    println!("睡眠之后");

    println!("receive {}", rx.recv().unwrap());
    handle.join().unwrap();
}

/// semaphore 信号量
/// 推荐使用tokio::sync::Semaphore
pub async fn thread_semaphore() {
    let semaphore = Arc::new(Semaphore::new(3));
    let mut join_handles = Vec::new();
    for _ in 0..5 {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        join_handles.push(tokio::spawn(async move {
            println!("execute");
            tokio::time::sleep(Duration::from_secs(1)).await;
            drop(permit)
        }))
    }
    for handle in join_handles {
        handle.await.unwrap();
    }
}
