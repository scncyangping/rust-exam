use std::env;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8888".to_string());

    let mut stream = TcpStream::connect(addr).await?;

    stream.write_all(b"getTime").await?;
    let mut buf = Vec::with_capacity(8128);
    let mut resp = [0u8; 2048];
    loop {
        let n = stream.read(&mut resp).await?;
        buf.extend_from_slice(&resp[..n]);
        if n == 0 {
            panic!("EOF")
        } else if buf.len() > 28 {
            break;
        } else {
            continue;
        }
    }
    let time_info = String::from_utf8(buf)?;
    println!("{}", time_info);
    Ok(())
}
