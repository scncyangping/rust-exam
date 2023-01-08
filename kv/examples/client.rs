use anyhow::Result;
use async_prost::AsyncProstStream;
use futures::{SinkExt, StreamExt};
use kv::{CommandRequest, CommandResponse};
use tokio::net::TcpStream;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let addr = "127.0.0.1:9527";
    // 连接服务器
    let stream = TcpStream::connect(addr).await?;
    // 连接到服务器
    let mut client =
        AsyncProstStream::<_, CommandResponse, CommandRequest, _>::from(stream).for_async();
    // 生成一个 HSET命令
    let cmd = CommandRequest::new_hset("table1", "hello", "world".into());
    // 发送 HSET命令
    client.send(cmd).await?;

    let cmd = CommandRequest::new_hget("table1", "hello");
    // 发送 HSET命令
    client.send(cmd).await?;

    if let Some(Ok(data)) = client.next().await {
        info!("Got response {:?}", data);
    }
    Ok(())
}
