use std::env;
use std::future::IntoFuture;
use std::sync::Arc;
use std::time::Duration;

use anyhow::anyhow;
use anyhow::Result;
use async_trait::async_trait;
use chrono::Utc;
use clap::builder::Str;
use client::Msg;
use client::Session;
use keys::key;
use russh::*;
use termion::raw::IntoRawMode;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::ToSocketAddrs;
use tokio::sync::RwLock;
use tokio::time::sleep;
use tracing::info;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry().init();
    let mut cli = Cli::default();
    cli.host = "0.0.0.0".to_string();
    cli.port = 9099;
    // cli.host = "10.0.1.52".to_string();
    // cli.port = 22;
    cli.password = "1qaz2wsx".to_string();
    cli.username = Some("root".to_string());
    cli.command = vec!["xxxx".to_string()]; // 交互式命令，如 bash 或 sh
    info!("Connecting to {}:{}", cli.host, cli.port);

    // 使用用户名和密码进行 SSH 连接
    let mut ssh = ClientSession::connect(
        cli.username.unwrap_or("root".to_string()),
        cli.password,
        (cli.host, cli.port),
    )
    .await?;
    Ok(())
}

struct Client {}

#[async_trait]
impl client::Handler for Client {
    type Error = russh::Error;

    async fn check_server_key(
        &mut self,
        _server_public_key: &key::PublicKey,
    ) -> Result<bool, Self::Error> {
        Ok(true)
    }
    async fn channel_success(
        &mut self,
        channel: ChannelId,
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        println!(
            "channel_success: {}, channelId: {}",
            Utc::now().to_string(),
            channel
        );
        Ok(())
    }
    async fn channel_open_failure(
        &mut self,
        channel: ChannelId,
        reason: ChannelOpenFailure,
        description: &str,
        language: &str,
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        println!(
            "channel_open_failure: {}, channelId: {},reason: {:?}",
            Utc::now().to_string(),
            channel,
            reason
        );
        Ok(())
    }

    async fn server_channel_open_session(
        &mut self,
        channel: ChannelId,
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        println!(
            "channel_open_failure: {}, channelId: {}",
            Utc::now().to_string(),
            channel
        );
        Ok(())
    }
}

pub struct ClientSession {
    session: client::Handle<Client>,
}

impl ClientSession {
    async fn connect<A: ToSocketAddrs>(
        user: impl Into<String>,
        password: String,
        addrs: A,
    ) -> Result<Self> {
        let config = client::Config {
            // 多久没收到消息则关闭通道(仅针对通道)
            inactivity_timeout: None,
            // 多少秒没收到消息则探活
            keepalive_interval: Some(Duration::from_secs(10)),
            // 达到3次没收到响应,则退出
            keepalive_max: 3,
            ..<_>::default()
        };

        let config = Arc::new(config);
        let sh = Client {};
        let user = user.into();

        // --------------------------- start ---------------------------
        // connect first
        let mut first = client::connect(config.clone(), &addrs, Client {}).await?;
        // 使用用户名和密码进行身份验证
        let auth_res = first.authenticate_password(&user, &password).await?;

        if !auth_res {
            anyhow::bail!("Authentication01 (with password) failed");
        }
        println!(
            "start first channel time:{}",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
        );
        let x = first.channel_open_session().await.unwrap();
        println!(
            "first receive channel id: {},time: {}",
            x.id(),
            Utc::now().to_string()
        );
        let x: Arc<RwLock<Channel<Msg>>> = Arc::new(RwLock::new(x));
        let read_x: Arc<RwLock<Channel<Msg>>> = x.clone();

        tokio::spawn(async move {
            // 等待5s过后,发起一个额外数据请求
            loop {
                sleep(Duration::from_secs(5)).await;
                let x = read_x.read().await;
                x.extended_data(4, "FFFFFFFFFFFFFFF".as_bytes())
                    .await
                    .unwrap();
            }
        });
        tokio::spawn({
            let write_x = x.clone();
            async move {
                loop {
                    tokio::select! {
                        _ = tokio::time::sleep(Duration::from_secs(10)) => {
                        println!("first sleep time now:{}",Utc::now().to_string());
                    },
                    msg = async {
                        let mut lock = write_x.write().await;
                        lock.wait().await
                    } => {
                        match msg {
                            Some(ChannelMsg::Open {
                                id,
                                max_packet_size,
                                window_size,
                            }) => {
                                println!("first receve open: {}", id);
                            }

                            Some(ChannelMsg::Data { data }) => {
                                let bytes: &[u8] = &data;
                                println!("first revece data{}", String::from_utf8_lossy(&bytes));
                            }

                            Some(ChannelMsg::ExtendedData { data, ext }) => {
                                let bytes: &[u8] = &data;
                                println!(
                                    "first revece ext: {},extendeData{}",
                                    ext,
                                    String::from_utf8_lossy(&bytes)
                                );
                            }

                            Some(ChannelMsg::Eof) => {
                                println!("first receive eof");
                                break;
                            }

                            Some(ChannelMsg::Close) => {
                                println!("first receive close");
                                break;
                            }

                            Some(ChannelMsg::OpenFailure(reason)) => {
                                println!("first open error:{:?}", reason)
                            }
                            None => {
                                println!("first receve none");
                                println!("first receve none: {}", Utc::now().to_string());
                                break;
                            }
                            msg => {
                                println!("first msg = {:?}", msg);
                            }
                        }
                       }
                    }
                }
            }
        });
        // --------------------------- end ---------------------------

        let mut session = client::connect(config, addrs, sh).await?;
        // 使用用户名和密码进行身份验证
        let auth_res = session.authenticate_password(user, password).await?;

        if !auth_res {
            anyhow::bail!("Authentication (with password) failed");
        }
        let global_channel = session.channel_open_session().await?;
        let global_channel_id = global_channel.id();
        println!(
            "receive channel id: {},time: {}",
            global_channel_id,
            Utc::now().to_string()
        );

        let g_c = Arc::new(RwLock::new(global_channel));
        let c_g_c = g_c.clone();

        tokio::spawn(async move {
            // 等待5s过后,发起一个额外数据请求
            loop {
                sleep(Duration::from_secs(5)).await;
                let x = c_g_c.read().await;
                x.extended_data(2, "QQQQQQQQQQQQQQQQQQ".as_bytes())
                    .await
                    .unwrap();
            }
        });

        let c_g_c_s = g_c.clone();
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = tokio::time::sleep(Duration::from_secs(10)) => {
                        println!("sleep time now:{}",Utc::now().to_string());
                    },
                    //data = c_g_c_s.lock().await.wait() => {
                    data = async {
                        let mut guard = c_g_c_s.write().await;  // 获取锁
                        guard.wait().await                     // 等待消息
                    } => {
                        match data {
                            Some(ChannelMsg::Open {
                                id,
                                max_packet_size,
                                window_size,
                            }) => {
                                println!("receve open: {}", id);
                            }

                            Some(ChannelMsg::Data { data }) => {
                                let bytes: &[u8] = &data;
                                println!("revece data{}", String::from_utf8_lossy(&bytes));
                            }

                            Some(ChannelMsg::ExtendedData { data, ext }) => {
                                let bytes: &[u8] = &data;
                                println!(
                                    "revece ext: {},extendeData{}",
                                    ext,
                                    String::from_utf8_lossy(&bytes)
                                );
                            }

                            Some(ChannelMsg::Eof) => {
                                println!("receive eof");
                                break;
                            }

                            Some(ChannelMsg::Close) => {
                                println!("receive close");
                                break;
                            }

                            Some(ChannelMsg::OpenFailure(reason)) => {
                                println!("open error:{:?}", reason)
                            }
                            None => {
                                println!("receve none");
                                println!("receve none: {}", Utc::now().to_string());
                                break;
                            }
                            msg => {
                                println!("msg = {:?}", msg);
                            }
                        }
                    },
                }
            }
        });
        println!("sleep 36000s");
        sleep(Duration::from_secs(36000)).await;
        println!("end sleep 36000s");
        Ok(Self { session })
    }

    async fn connect_and_run<A: ToSocketAddrs>(
        user: impl Into<String>,
        password: String,
        addrs: A,
    ) -> Result<Self> {
        let config = client::Config {
            inactivity_timeout: Some(Duration::from_secs(10)),
            ..<_>::default()
        };

        let config = Arc::new(config);
        let sh = Client {};

        let mut session = client::connect(config, addrs, sh).await?;

        // 使用用户名和密码进行身份验证
        let auth_res = session.authenticate_password(user, password).await?;

        if !auth_res {
            anyhow::bail!("Authentication (with password) failed");
        }

        info!("Connected");
        let mut ssh = Self { session };
        // 执行交互式命令
        let code = {
            let _raw_term = std::io::stdout().into_raw_mode()?; // 将终端设置为 raw 模式
            ssh.call().await.map_err(|e| anyhow!("cuowula: {:?}", e))?;
        };

        println!("Exitcode: {:?}", code);
        ssh.close().await?;
        Ok(ssh)
    }

    async fn call(&mut self) -> Result<u32> {
        let mut channel = self.session.channel_open_session().await?;
        //channel.extended_data(ext, data)
        let (w, h) = termion::terminal_size()?;
        // 请求一个交互式 PTY
        channel
            .request_pty(
                true, // 交互式 PTY
                &env::var("TERM").unwrap_or("xterm".into()),
                w as u32,
                h as u32,
                0,
                0,
                &[],
            )
            .await?;
        //channel.exec(true, command).await?; // 执行命令
        channel.request_shell(false).await?; // 请求启动用户的默认 shell

        let code;
        let mut stdin = tokio::io::stdin();
        let mut stdout = tokio::io::stdout();
        let mut buf = vec![0; 1024];
        let mut stdin_closed = false;

        loop {
            tokio::select! {
                // 读取用户输入并发送到服务器
                r = stdin.read(&mut buf), if !stdin_closed => {
                    match r {
                        Ok(0) => {
                            stdin_closed = true;
                            channel.eof().await?;
                        },
                        Ok(n) => channel.data(&buf[..n]).await?,
                        Err(e) => return Err(e.into()),
                    };
                },
                // 读取服务器输出并打印到终端
                Some(msg) = channel.wait() => {
                    match msg {
                        ChannelMsg::Data { ref data } => {
                            stdout.write_all(data).await?;
                            stdout.flush().await?;
                        }
                        ChannelMsg::ExitStatus { exit_status } => {
                            code = exit_status;
                            if !stdin_closed {
                                channel.eof().await?;
                            }
                            break;
                        }
                        _ => {}
                    }
                },
            }
        }
        Ok(code)
    }

    async fn close(&mut self) -> Result<()> {
        self.session
            .disconnect(Disconnect::ByApplication, "", "English")
            .await?;
        Ok(())
    }
}

#[derive(Default)]
pub struct Cli {
    host: String,

    port: u16,

    username: Option<String>,

    password: String,

    command: Vec<String>,
}
