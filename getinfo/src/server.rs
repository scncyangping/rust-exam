mod client;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::process::Command;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8888".to_string());

    let listener = TcpListener::bind(&addr).await?;

    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut buf = [0; 1024];
            let mut offset = 0;
            let n = socket
                .read(&mut buf[offset..])
                .await
                .expect("failed to read data from socket");
            if n == 0 {
                return;
            }
            println!("offset : {offset}, n:{n}");
            let end = offset + n;
            if let Ok(directive) = std::str::from_utf8(&buf[..end]) {
                println!("{directive}");
                let output = process(directive).await;
                println!("{output}");
                socket
                    .write_all(&output.as_bytes())
                    .await
                    .expect("failed to write data to socket")
            } else {
                offset = end;
            }
        });
    }
}

async fn process(directive: &str) -> String {
    if directive == "getTime" {
        let output = Command::new("date").output().await.unwrap();
        String::from_utf8(output.stdout).unwrap()
    } else {
        "invalid command".to_owned()
    }
}
