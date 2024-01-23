use futures::executor::block_on;
use std::marker::PhantomPinned;
use std::pin::Pin;
use thread::self_mod::async_await::TimerFuture;

mod self_mod;

fn main() {
    let x = self_mod::async_await::select_future();

    block_on(x)
}
// #[derive(Debug)]
// struct Test {
//     a: String,
//     b: *const String,
//     _marker: PhantomPinned,
// }
//
// impl Test {
//     fn new(txt: &str) -> Pin<Box<Self>> {
//         let t = Test {
//             a: String::from(txt),
//             b: std::ptr::null(),
//             _marker: PhantomPinned,
//         };
//         let mut boxed = Box::pin(t);
//         let self_ptr: *const String = &boxed.as_ref().a;
//         unsafe { boxed.as_mut().get_unchecked_mut().b = self_ptr };
//
//         boxed
//     }
//
//     fn a(self: Pin<&Self>) -> &str {
//         &self.get_ref().a
//     }
//
//     fn b(self: Pin<&Self>) -> &String {
//         unsafe { &*(self.b) }
//     }
// }
//
// pub fn main() {
//     let mut test1 = Test::new("test1");
//     let mut test2 = Test::new("test2");
//
//     println!("a: {}, b: {}", test1.as_ref().a(), test1.as_ref().b());
//     std::mem::swap(&mut test1, &mut test2);
//     println!("a: {}, b: {}", test2.as_ref().a(), test2.as_ref().b());
// }

// fn main() {
//     let (executor, spawner) = self_mod::async_await::new_executor_and_spawner();
//
//     // 生成一个任务
//     spawner.spawn(async {
//         println!("howdy!");
//         // 创建定时器Future，并等待它完成
//         TimerFuture::new(std::time::Duration::new(2, 0)).await;
//         println!("done!");
//     });
//
//     // drop掉任务，这样执行器就知道任务已经完成，不会再有新的任务进来
//     drop(spawner);
//
//     // 运行执行器直到任务队列为空
//     // 任务运行后，会先打印`howdy!`, 暂停2秒，接着打印 `done!`
//     executor.run();
// }

//
// #[derive(Debug)]
// struct Config {
//     a: String,
//     b: String,
// }
// static mut CONFIG: Option<&mut Config> = None;
//
// fn main() {
//     let c = Box::new(Config {
//         a: "A".to_string(),
//         b: "B".to_string(),
//     });
//
//     unsafe {
//         // 将`c`从内存中泄漏，变成`'static`生命周期
//         CONFIG = Some(Box::leak(c));
//         println!("{:?}", CONFIG);
//     }
//
//     if let Some(t) = unsafe { &CONFIG }{
//         println!("a:{} b:{}",t.a,t.b)
//     }
//
//     println!("{:?}", unsafe { &CONFIG });
// }

// #[tokio::main]
// async fn main() {
//     self_mod::barrier::thread_semaphore().await;
// }

// fn main() {
//     self_mod::send_sync::test_self_send_sync();
//     let mut threads = Vec::with_capacity(100);
//     for _ in 0..100 {
//         threads.push(spawn(|| {
//             println!("id:{}", self_mod::send_sync::id_generate())
//         }))
//     }
//     for hand in threads {
//         hand.join().unwrap()
//     }
//     println!("end")
// }
