use std::ops::Deref;

fn main() {
    // 由于 String 实现了 Deref<Target=str>
    let owned = "Hello".to_string();
    // 因此下面的函数可以正常运行：
    foo(&owned);
}

fn foo(s: &str) {
    println!("{}", s);
}
