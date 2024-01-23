use std::fs;

mod core;
fn main() {
    core::multi_web::run("127.0.0.1:9889");
    // let contents = fs::read_to_string("hello.html").unwrap();
    // println!("{contents}")
}
