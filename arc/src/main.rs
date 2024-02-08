use std::{
    sync::{Arc, Mutex},
    thread,
};

fn arc_mutex_is_send_sync() {
    let a = Arc::new(Mutex::new(1));
    let b = a.clone();
    let c = a.clone();

    let handle = thread::spawn(move || {
        println!("c is execute");
        let mut g = c.lock().unwrap();
        *g += 1;
    });
    {
        println!("b is execute");
        let mut g = b.lock().unwrap();
        *g += 1;
    }

    handle.join().unwrap();
    println!("a= {:?}", a);
}

fn main() {
    arc_mutex_is_send_sync();

    let x = "Hello world";
    print(x);
    let y = "Hello world".to_string();
    print(y);
    let z = SelfStruct {
        name: String::from("123"),
    };
    print(z);
}

struct SelfStruct {
    name: String,
}

impl AsRef<str> for SelfStruct {
    fn as_ref(&self) -> &str {
        self.name.as_ref()
    }
}

fn print(v: impl AsRef<str>) {
    println!("{}", v.as_ref());
}
