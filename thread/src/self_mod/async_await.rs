//! async/await
//! 通过async标记的语法块儿会被转换成实现了Future特征的状态机
//! 当Future执行遇到阻塞时，它会让出当前线程的控制权，这样其他的Future就
//! 可以在该线程中执行，这种方式完全不会导致当前线程阻塞

use futures::{join, select, try_join, FutureExt, TryFutureExt};
use std::time;

pub async fn do_something() {
    println!("go go !")
}

/// 交替打印数据以说明当遇到await时，此线程会去执行
/// 其他future
pub async fn print_num() {
    async fn print_1() {
        for i in 0..10 {
            println!("print_1: {} start", i);
            tokio::time::sleep(time::Duration::from_secs(1)).await;
            println!("print_1: {} stop", i);
        }
    }
    async fn print_2() {
        for i in 0..10 {
            println!("print_2: {} start", i);
            tokio::time::sleep(time::Duration::from_secs(1)).await;
            println!("print_2: {} stop", i);
        }
    }
    let f1 = print_1();
    let f2 = print_2();
    join!(f1, f2);
}

use std::sync::mpsc::{Receiver, SyncSender};
use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
    thread,
    time::Duration,
};

pub struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

/// 在Future和等待的线程间共享状态
struct SharedState {
    /// 定时(睡眠)是否结束
    completed: bool,

    /// 当睡眠结束后，线程可以用`waker`通知`TimerFuture`来唤醒任务
    waker: Option<Waker>,
}
/// 实现通用方法
impl Future for TimerFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.completed {
            Poll::Ready(())
        } else {
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}
/// 构建Future
impl TimerFuture {
    pub fn new(duration: Duration) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
        }));

        let shared_state_clone = shared_state.clone();

        thread::spawn(move || {
            thread::sleep(Duration::from(duration));
            let mut lock = shared_state_clone.lock().unwrap();
            lock.completed = true;
            if let Some(waker) = lock.waker.take() {
                waker.wake()
            }
        });
        TimerFuture { shared_state }
    }
}

/// 任务执行器,负责从通道中接收任务然后执行
pub struct Executor {
    ready_queue: Receiver<Arc<Task>>,
}

/// `Spawner` 负责创建新的 `Future` 然后将它发送到任务通道中
pub struct Spawner {
    task_sender: SyncSender<Arc<Task>>,
}

pub struct Task {
    future: Mutex<Option<futures::future::BoxFuture<'static, ()>>>,
    /// 可以将该任务自身放回到任务通道中，等待执行器的poll
    task_sender: SyncSender<Arc<Task>>,
}

pub fn new_executor_and_spawner() -> (Executor, Spawner) {
    // 任务通道允许的最大缓冲数(任务队列的最大长度)
    // 当前的实现仅仅是为了简单，在实际的执行中，并不会这么使用
    const MAX_QUEUED_TASKS: usize = 10_000;
    let (task_sender, ready_queue) = std::sync::mpsc::sync_channel(MAX_QUEUED_TASKS);
    (Executor { ready_queue }, Spawner { task_sender })
}

impl Spawner {
    pub fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        let future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });
        self.task_sender.send(task).expect("queue max");
    }
}

impl futures::task::ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        // 通过发送任务到任务管道的方式来实现`wake`，这样`wake`后，任务就能被执行器`poll`
        let cloned = arc_self.clone();
        arc_self.task_sender.send(cloned).expect("任务队列已满");
    }
}

impl Executor {
    pub fn run(&self) {
        while let Ok(task) = self.ready_queue.recv() {
            // 获取一个future，若它还没有完成(仍然是Some，不是None)，则对它进行一次poll并尝试完成它
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take() {
                // 基于任务自身创建一个 `LocalWaker`
                let waker = futures::task::waker_ref(&task);
                let context = &mut Context::from_waker(&*waker);
                // `BoxFuture<T>`是`Pin<Box<dyn Future<Output = T> + Send + 'static>>`的类型别名
                // 通过调用`as_mut`方法，可以将上面的类型转换成`Pin<&mut dyn Future + Send + 'static>`
                if future.as_mut().poll(context).is_pending() {
                    // Future还没执行完，因此将它放回任务中，等待下次被poll
                    *future_slot = Some(future);
                }
            }
        }
    }
}

async fn future_1() -> Result<(), ()> {
    Ok(())
}

async fn future_2() -> Result<(), String> {
    Err("future_2 error".to_string())
}

async fn join_future() -> Result<((), ()), String> {
    let f1 = future_1().map_err(|()| "123".to_string());
    let f2 = future_2();

    try_join!(f1, f2)
}

pub async fn select_future() {
    let t1 = future_1().fuse();
    let t2 = future_2().fuse();

    futures::pin_mut!(t1, t2);

    select! {
        r1 = t1 => println!("任务1率先完成 {:?}",r1),
        r2 = t2 => println!("任务2率先完成 {:?}",r2),
    }
}
