//! send 保证数据能在线程中转移所有权
//! sync 保证数据能在线程中共享(通过引用)
//! 编译期初始化的全局变量，const创建常量，static创建静态变量，Atomic创建原子类型
//! 运行期初始化的全局变量，lazy_static用于懒初始化，Box::leak利用内存泄漏将一个变量的生命周期变为'static

/**
lazy_static
一个全局的动态配置，它在程序开始后，才加载数据进行初始化，最终可以让各个线程直接访问使用
```
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref HASHMAP: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();
        m.insert(0, "foo");
        m.insert(1, "bar");
        m.insert(2, "baz");
        m
    };
}

fn main() {
    // 首次访问`HASHMAP`的同时对其进行初始化
    println!("The entry for `0` is \"{}\".", HASHMAP.get(&0).unwrap());

    // 后续的访问仅仅获取值，再不会进行任何初始化操作
    println!("The entry for `1` is \"{}\".", HASHMAP.get(&1).unwrap());
}
```
*/
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::OnceLock;

static MAX_ID: u64 = u64::MAX / 2;
static GEN: AtomicU64 = AtomicU64::new(0);

/// ID生成原子类使用
pub fn id_generate() -> u64 {
    let id = GEN.load(Ordering::Relaxed);
    if id > MAX_ID {
        panic!("max num")
    }
    let nex_id = GEN.fetch_add(1, Ordering::Relaxed);
    if nex_id > MAX_ID {
        panic!("max num")
    }
    return nex_id;
}

#[derive(Debug)]
struct MyU8(*const u8);

unsafe impl Send for MyU8 {}
pub fn test_self_send_sync() {
    // 裸指针不能直接在多线程中传递
    /*
    let x = 8 as *const u8;
    let mux = std::sync::Mutex::new(x);
    let p = std::thread::spawn(move ||{
        let x = mux.lock().unwrap();
    })
    */
    use std::sync::{Arc, Mutex};

    let x = 8 as *const u8;

    let my_u8 = Arc::new(Mutex::new(MyU8(x)));

    let my_u8_clone = Arc::clone(&my_u8);

    let p = std::thread::spawn(move || {
        let mut x = my_u8_clone.lock().unwrap();
        x.0 = 9 as *const u8;
    });

    p.join().unwrap();

    std::thread::sleep(std::time::Duration::from_secs(5));

    let x = my_u8.lock().unwrap();
    println!("{:?}  {}", x, x.0 as u8);
}
#[derive(Debug)]
struct Logger;
static LOGGER: OnceLock<Logger> = OnceLock::new();
impl Logger {
    fn global() -> &'static Self {
        LOGGER.get_or_init(|| {
            println!("Logger init");
            Logger
        })
    }
    fn log(&self, message: String) {
        println!("{}", message)
    }
}

/// 全局初始化一次
pub fn once_cell_test() {
    let handle = std::thread::spawn(|| {
        let logger = Logger::global();
        logger.log("thread message".to_string());
    });

    let logger = Logger::global();
    logger.log("some message".to_string());
    let logger2 = Logger::global();
    logger2.log("other message".to_string());

    handle.join().unwrap();
}
