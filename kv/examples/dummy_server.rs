use anyhow::Result;
use async_prost::AsyncProstStream;
use futures::{SinkExt, StreamExt};
use kv::{CommandRequest, CommandResponse};
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let addr = "127.0.0.1:9527";
    let listener = TcpListener::bind(addr).await?;
    info!("Start listening on {}", addr);

    loop {
        let (stream, addr) = listener.accept().await?;

        info!("client connected {}", addr);

        tokio::spawn(async move {
            let mut stream =
                AsyncProstStream::<_, CommandRequest, CommandResponse, _>::from(stream).for_async();

            while let Some(Ok(message)) = stream.next().await {
                info!("Got a new command: {:?}", message);
                // 创建一个404 response
                let mut response = CommandResponse::default();
                response.status = 404;
                response.message = "Not Found".to_string();
                stream.send(response).await.unwrap();
            }
            info!("client {:?} disconnected", addr)
        });
    }
}
