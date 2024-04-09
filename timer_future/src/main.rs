use std::{
    sync::{
        mpsc::{sync_channel, Receiver, SyncSender},
        Arc, Mutex,
    },
    task::Context,
    time::Duration,
};

use futures::FutureExt;
use futures::{
    future::BoxFuture,
    task::{waker_ref, ArcWake},
};
use std::future::Future;
use timer_future::TimerFuture;

/// 构建任务执行器,负责从通道中接收任务然后执行
struct Executor {
    ready_queue: Receiver<Arc<Task>>,
}
/// 一个Future,它可以调度自己(将自己放入任务通道中),然后等待执行器poll
/// 表示了一个具体的任务类型,BoxFuture是一个Future的包装,用于在线程中传递
struct Task {
    /// 进行中的Future,在未来的某个时间点会被完成
    future: Mutex<Option<BoxFuture<'static, ()>>>,
    task_sender: SyncSender<Arc<Task>>,
}

///`Spawner`负责创建新的`Future`然后将它发送到任务通道中
/// 将传入的一个Future包装未一个Task,然后发送到队列里面,供
/// 执行器消费
#[derive(Clone)]
struct Spawner {
    task_sender: SyncSender<Arc<Task>>,
}

///创建一个future
impl Spawner {
    fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        // 要使用boxed的方法,需要引入FutureExt这个库
        // 这里面会对Future的trait进行扩展
        let future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });
        self.task_sender.send(task).expect("任务队列已满");
    }
}

/// 实现这个方法,当执行wake时调用这个方法
/// 将任务再次放入队列里面等待执行
impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let cloned = arc_self.clone();
        arc_self.task_sender.send(cloned).expect("队列已满")
    }
}

impl Executor {
    fn run(&self) {
        while let Ok(task) = self.ready_queue.recv() {
            // 获取一个future，若它还没有完成(仍然是Some，不是None)，则对它进行一次poll并尝试完成它
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take() {
                let waker = waker_ref(&task);
                let context = &mut Context::from_waker(&waker);
                // 执行一次poll
                // 若未执行完成,则重新赋值到task
                if future.as_mut().poll(context).is_pending() {
                    *future_slot = Some(future);
                }
            }
        }
    }
}

fn new_executor_and_spawner() -> (Executor, Spawner) {
    // 任务通道允许的最大缓冲数(任务队列的最大长度)
    const MAX_QUEUED_TASKS: usize = 10_000;
    let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);
    (Executor { ready_queue }, Spawner { task_sender })
}

fn main() {
    let (executor, spawner) = new_executor_and_spawner();

    // 生成一个任务
    spawner.spawn(async {
        println!("howdy!");
        // 创建定时器Future，并等待它完成
        TimerFuture::new(Duration::new(2, 0)).await;
        println!("done!");
    });
    // drop掉任务，这样执行器就知道任务已经完成，不会再有新的任务进来
    drop(spawner);
    // 运行执行器直到任务队列为空
    // 任务运行后，会先打印`howdy!`, 暂停2秒，接着打印 `done!`
    executor.run();
}
