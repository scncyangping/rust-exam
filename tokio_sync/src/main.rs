use std::hash::Hasher;
use std::sync::atomic::{AtomicI32, AtomicU32, AtomicU8, Ordering};
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
#[tokio::main]
async fn main() {
    let lock = RwLock::new(5);
    {
        let r1 = lock.read().await;
        let r2 = lock.read().await;
        assert_eq!(*r1, 5);
        assert_eq!(*r2, 5);
    }
    {
        let mut w = lock.write().await;
        *w += 1;
        assert_eq!(*w, 6);
    }
}
#[tokio::main]
async fn main_back() {
    let db = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let arc_db = Arc::new(Mutex::new(db));
    let arc_db2 = arc_db.clone();
    let arc_db3 = arc_db.clone();

    let task_a = tokio::spawn(async move {
        let mut db = arc_db.lock().await;
        db[4] = 50;
        assert_eq!(db[4], 50);
    });
    let task_b = tokio::spawn(async move {
        let mut db = arc_db2.lock().await;
        db[4] = 100;
        assert_eq!(db[4], 100);
    });
    task_a.await.unwrap();
    task_b.await.unwrap();
    println!("{:?}", arc_db3.lock().await);
}
