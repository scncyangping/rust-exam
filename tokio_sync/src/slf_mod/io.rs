//! 介绍tokio io相关操作

use std::ptr::read;
use std::thread;
use std::time::Duration;
use tokio::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

/// 测试 tokio::io::copy
pub async fn test_io_copy(addr: &str) -> io::Result<()> {
    let listener = TcpListener::bind(addr).await?;
    let (stream, _) = listener.accept().await?;
    //let stream = TcpStream::connect(addr).await?;
    let (mut rd, mut wd) = io::split(stream);
    tokio::spawn(async move {
        println!("write01");
        wd.write_all(b"hello").await?;
        println!("write02");
        wd.write_all(b"word").await?;
        Ok::<_, io::Error>(())
    });
    let mut buf = vec![0; 2048];
    loop {
        let n = rd.read(&mut buf).await?;
        if n == 0 {
            break;
        }
        println!("Got {:?}", String::from_utf8(buf[..n].to_vec()));
    }
    Ok(())
}
pub async fn client(addr: &str) -> io::Result<()> {
    let socket = TcpStream::connect(addr).await?;
    let (mut rd, mut wr) = io::split(socket);

    // 创建异步任务，在后台写入数据
    tokio::spawn(async move {
        wr.write_all(b"hello\r\n").await?;
        wr.write_all(b"world\r\n").await?;

        // 有时，我们需要给予 Rust 一些类型暗示，它才能正确的推导出类型
        Ok::<_, io::Error>(())
    });

    let mut buf = vec![0; 128];

    loop {
        let n = rd.read(&mut buf).await?;
        if n == 0 {
            break;
        }
        println!("GOT {:?}", &buf[..n]);
    }

    Ok(())
}

pub async fn test_manu_copy(addr: &str) -> io::Result<()> {
    let listener = TcpListener::bind(addr).await?;
    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut buf = vec![0; 1024];
            loop {
                match socket.read(&mut buf).await {
                    Ok(0) => return,
                    Ok(n) => {
                        if socket.write_all(&buf[..n]).await.is_err() {
                            return;
                        }
                    }
                    Err(_) => return,
                }
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::slf_mod::io::client;

    #[test]
    fn test_client() {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        let num = rt.block_on(client("127.0.0.1:9898"));
        println!("{:?}", num);
        ()
    }
}
