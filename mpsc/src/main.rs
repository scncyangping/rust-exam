use std::time::Duration;
use tokio::sync::{mpsc, oneshot};

async fn test() -> u8 {
    println!("test executes");
    1
}
fn main() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let num = rt.block_on(test());
    println!("{num}");
    let num = rt.block_on(async { test().await });
    println!("{num}");
}

#[tokio::main]
async fn tokio_select() {
    let task_a = tokio::spawn(async {
        println!("task_a");
        tokio::time::sleep(Duration::from_secs(3)).await;
        1
    });
    let task_b = tokio::spawn(async {
        println!("task_b");
        2
    });
    let task_c = tokio::spawn(async {
        println!("task_c");
        3
    });

    let ret = tokio::select! {
        r = task_a => r.unwrap(),
        r = task_b => r.unwrap(),
        r = task_c => r.unwrap(),
    };
    println!("{}", ret);
}
#[tokio::main]
async fn tokio_join() {
    let task_a = tokio::spawn(async {
        println!("task_a");
        1
    });
    let task_b = tokio::spawn(async {
        println!("task_b");
        tokio::time::sleep(Duration::from_secs(5)).await;
        2
    });
    let task_c = tokio::spawn(async {
        println!("task_c");
        3
    });

    let (r1, r2, r3) = tokio::join!(task_a, task_b, task_c);
    println!("{} {} {}", r1.unwrap(), r2.unwrap(), r3.unwrap());
}

#[tokio::main]
async fn tokio_mpsc_oneshot() {
    let mut db = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    // MPSC 的特点就是可以有多个生产者，但只有一个消费者。
    // 因此，tx 可以被随意 clone 多份，但是 rx 只能有一个
    let (tx, mut rx) = mpsc::channel::<(u32, oneshot::Sender<bool>)>(100);
    // mpsc::unbounded_channel() 可以创建无容量限制的channel

    let tx1 = tx.clone();
    let tx2 = tx.clone();
    //tokio::task::spawn() 这个 API 有个特点，就是通过它创建的异步任务，一旦创建好
    //就会立即扔到 tokio runtime 里执行，不需要对其返回的 JoinHandler 进行 await 才驱动执行
    let task_a = tokio::task::spawn(async move {
        // tokio::time::sleep(Duration::from_secs(3)).await;
        let (sender, receiver) = oneshot::channel();
        if let Err(_) = tx1.send((50, sender)).await {
            println!("task_a receive dropped");
        }
        match receiver.await {
            Ok(r) => println!("task_a oneshot receiver: {}", r),
            Err(e) => println!("error: {}", e),
        }
        println!("task_a receive end");
    });
    let task_b = tokio::spawn(async move {
        let (sender, receiver) = oneshot::channel();
        if let Err(_) = tx2.send((100, sender)).await {
            println!("receiver dropped");
        }
        if let Ok(ret) = receiver.await {
            if ret {
                println!("task_b receive success");
            } else {
                println!("task_b receive success");
            }
        }
        println!("task_b receive end");
    });

    let task_c = tokio::spawn(async move {
        while let Some((i, sender)) = rx.recv().await {
            println!("got  =  {}", i);
            db[4] = 1;
            println!("{:?}", db);
            if i == 50 {
                sender.send(false).unwrap();
            } else if i == 100 {
                sender.send(true).unwrap();
            }
        }
    });
    task_a.await.unwrap();
    task_c.await.unwrap();
    task_b.await.unwrap();
}
