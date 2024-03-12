use bytes::Bytes;
use futures::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_util::codec::{Framed, LengthDelimitedCodec};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8887".to_string());

    let stream = TcpStream::connect(&addr).await?;
    let mut framed_stream = Framed::new(stream, LengthDelimitedCodec::new());

    framed_stream.send(Bytes::from("getTime")).await?;

    if let Some(msg) = framed_stream.next().await {
        match msg {
            Ok(msg) => {
                let time_info = String::from_utf8(msg.to_vec())?;
                println!("{}", time_info);
                println!("{}", time_info);
            } 
            Err(e) => {
                println!("{e}");
                println!("{e}");
                return Err(e.into());
            }
        }
    }

    Ok(())
}
