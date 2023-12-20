use std::env;
use tokio::process::Command;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>>{
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8888".to_string());
    println!("Listening on : {}",addr);
    let listener = TcpListener::bind(&addr).await?;
    loop {
        let (mut socket,_)  = listener.accept().await?;
        tokio::spawn(async move {
            let mut buf = [0;1024];
            let mut offset = 0;
            loop {
                let n = socket
                    .read(&mut buf[offset..])
                    .await
                    .expect("failed to read");
                if n == 0 {
                    return ;
                }
                println!("offset: {offset},n {n}");
                let end = offset +n;
                if let Ok(directive) = std::str::from_utf8(&buf[..end]){
                    println!("{directive}");
                    let output = process(directive).await;
                    println!("{output}");
                    socket.write_all(
                        &output.as_bytes()
                    ).await
                        .expect("failed")
                }else{
                    offset = end;
                }
            }
        });
    }
}

async fn  process(directive: &str)->String{
    if directive == "gettime"{
        let output = Command::new("date").output()
            .await.unwrap();
        String::from_utf8(output.stdout).unwrap()
    }else {
        "invalid command".to_owned()
    }
}