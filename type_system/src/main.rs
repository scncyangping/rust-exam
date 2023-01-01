use std::io::{Write};
use std::net::TcpStream;

#[derive(Debug)]
struct MyWriter<W> {
    writer: W,
}

impl<W: Write> MyWriter<W> {
    pub fn new(w: W) -> Self {
        Self { writer: w }
    }

    pub fn write(&mut self, buf: &str) -> std::io::Result<()> {
        self.writer.write_all(buf.as_bytes())
    }
}

fn main() {
    let stream = TcpStream::connect("127.0.0.1:9090").unwrap();
    let mut writer = MyWriter::new(stream);
    match writer.write("hello world!") {
        Ok(_) => println!("ok"),
        Err(e) => println!("error: {}", e),
    }
}
