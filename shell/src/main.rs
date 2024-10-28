use std::env;
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::thread;

use ssh2::Session;
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use tungstenite::Message;

#[tokio::main]
async fn main() {
    // 监听 WebSocket 连接
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Listening on: {}", addr);

    loop {
        let (socket, _) = listener.accept().await.unwrap();

        // 使用 tokio-tungstenite 接受 WebSocket 连接
        let ws_stream = accept_async(socket).await.unwrap();

        // 为每个连接启动新的线程进行处理
        tokio::spawn(async move {
            if let Err(err) = handle_websocket(ws_stream).await {
                eprintln!("WebSocket error: {}", err);
            }
        });
    }
}

async fn handle_websocket(ws_stream: tokio_tungstenite::WebSocketStream<TcpStream>) -> Result<(), Box<dyn std::error::Error>> {
    // 建立 SSH 连接
    let tcp = TcpStream::connect("example.com:22")?;
    let mut sess = Session::new()?;
    sess.set_tcp_stream(tcp);
    sess.handshake()?;

    // SSH 用户认证
    let username = env::var("SSH_USERNAME").unwrap();
    let password = env::var("SSH_PASSWORD").unwrap();
    sess.userauth_password(&username, &password)?;

    // 打开一个通道
    let mut channel = sess.channel_session()?;
    channel.request_pty("xterm", None, None)?;

    // 启动 shell
    channel.shell()?;

    // 读取 WebSocket 消息并发送到 SSH 通道
    let (mut write, mut read) = ws_stream.split();
    let mut stdin = channel.stdin().unwrap();
    let mut stdout = channel.stdout().unwrap();

    let stdin_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = read.next().await {
            if let Message::Text(data) = msg {
                if let Err(err) = stdin.write_all(data.as_bytes()).await {
                    eprintln!("Failed to send to SSH channel: {}", err);
                    break;
                }
            }
        }
        stdin.shutdown().await.unwrap();
    });

    let stdout_task = tokio::spawn(async move {
        let mut buf = [0; 4096];
        loop {
            let size = stdout.read(&mut buf).unwrap();
            if size == 0 {
                break;
            }
            let msg = Message::Text(String::from_utf8_lossy(&buf[..size]).to_string());
            if let Err(err) = write.send(msg).await {
                eprintln!("Failed to send WebSocket message: {}", err);
                break;
            }
        }
    });

    // 等待任务完成
    stdin_task.await?;
    stdout_task.await?;

    Ok(())
}
