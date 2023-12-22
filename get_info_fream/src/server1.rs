use bytes::Bytes;
use futures::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio::process::Command;
use tokio_util::codec::{Framed, LengthDelimitedCodec};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8887".to_string());

    println!("get addr {addr}");
    let listener = TcpListener::bind(addr).await?;

    loop {
        let (stream, _) = listener.accept().await?;
        let mut framed_stream = Framed::new(stream, LengthDelimitedCodec::new());
        tokio::spawn(async move {
            while let Some(msg) = framed_stream.next().await {
                match msg {
                    Ok(msg) => {
                        let directive = String::from_utf8(msg.to_vec())
                            .expect("error when converting to string");
                        println!("{directive}");
                        let output = process(&directive).await;
                        println!("{output}");
                        _ = framed_stream.send(Bytes::from(output)).await;
                    }
                    Err(e) => {
                        println!("{e:?}");
                    }
                }
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
