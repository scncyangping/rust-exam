//! send 保证数据能在线程中转移所有权
//! sync 保证数据能在线程中共享(通过引用)
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
