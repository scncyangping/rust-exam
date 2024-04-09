use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
    thread,
    time::Duration,
};

/// 自定义一个TimerFuture
pub struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

struct SharedState {
    completed: bool,
    // 引入Waker 用于通知excutor继续执行
    waker: Option<Waker>,
}

/// 实现Future
///
/// 实现其trait用于异步任务判断业务是否准备好执行
impl Future for TimerFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // 检查状态是否完成
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.completed {
            Poll::Ready(())
        } else {
            // 按理说只需要复制一次,但是这儿每次poll都会clone一次
            // 因为`TimerFuture`可以在执行器的不同任务间移动，如果只克隆一次，
            // 那么获取到的`waker`可能已经被篡改并指向了其它任务，最终导致执行器运行了错误的任务
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

/// 构建定时器和启动计时器线程
impl TimerFuture {
    pub fn new(duration: Duration) -> TimerFuture {
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
        }));
        let thread_shaared_state = shared_state.clone();
        thread::spawn(move || {
            // 睡眠一会儿
            thread::sleep(duration);
            // 获取到waker
            let mut shared_state = thread_shaared_state.lock().unwrap();
            shared_state.completed = true;
            // 若时间到了,此waker也被设置,则调用waker唤醒
            if let Some(waker) = shared_state.waker.take() {
                // 调用唤醒方法
                waker.wake()
            }
        });
        TimerFuture { shared_state }
    }
}
