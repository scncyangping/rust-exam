//! SSH SERVER

use std::{
    borrow::Cow,
    collections::binary_heap::Drain,
    net::SocketAddr,
    sync::Arc,
    time::{self, Duration},
};

use super::{
    common::{DirectTCPIPParams, PtyRequest, ServerChannelId, X11Request},
    secret::Secret,
};
use anyhow::anyhow;
use async_trait::async_trait;
use bytes::Bytes;
use chrono::Utc;
use openssl::rsa::Rsa;
use russh::{
    keys::{
        decode_secret_key,
        key::{KeyPair, PublicKey},
    },
    server::{Auth, Handle, Msg, Session},
    Channel, ChannelId, ChannelMsg, MethodSet, Preferred, Pty, Sig,
};
use std::fmt::Debug;
use tokio::{
    io::{AsyncRead, AsyncWrite},
    net::TcpListener,
    sync::{
        mpsc::{unbounded_channel, UnboundedSender},
        oneshot,
    },
};
use tracing::{debug, info};

fn generate_rsa_keypair() -> Result<KeyPair, Box<dyn std::error::Error>> {
    let rsa = Rsa::generate(2048)?.private_key_to_pem()?;
    // 将 PEM 格式私钥解析为 Russh 的 KeyPair
    let private_key = decode_secret_key(std::str::from_utf8(&rsa)?, None)?;
    Ok(private_key)
}

pub async fn run_server(address: SocketAddr) -> anyhow::Result<()> {
    let russh_config = {
        russh::server::Config {
            auth_rejection_time: Duration::from_secs(1),
            auth_rejection_time_initial: Some(Duration::from_secs(0)),
            //inactivity_timeout: Some(Duration::from_secs(3600)),
            // 多久没收到消息则关闭通道(仅针对通道)
            inactivity_timeout: None,
            // 多少秒没收到消息则探活
            keepalive_interval: Some(Duration::from_secs(10)),
            // 达到3次没收到响应,则退出
            keepalive_max: 3,
            methods: MethodSet::PUBLICKEY | MethodSet::PASSWORD | MethodSet::KEYBOARD_INTERACTIVE,
            keys: vec![generate_rsa_keypair().map_err(|e| anyhow!(e.to_string()))?],
            event_buffer_size: 100,
            preferred: Preferred {
                key: Cow::Borrowed(&[
                    russh::keys::key::ED25519,
                    russh::keys::key::RSA_SHA2_256,
                    russh::keys::key::RSA_SHA2_512,
                    russh::keys::key::SSH_RSA,
                ]),
                ..<_>::default()
            },
            ..<_>::default()
        }
    };

    let russh_config = Arc::new(russh_config);

    let socket = TcpListener::bind(&address).await?;
    info!(?address, "Listening");
    while let Ok((socket, remote_address)) = socket.accept().await {
        let russh_config = russh_config.clone();
        let (event_tx, event_rx) = unbounded_channel();
        let handler = ServerHandler {
            handle: None,
            event_tx,
        };
        tokio::spawn(_run_stream(russh_config, socket, handler));
    }
    Ok(())
}

async fn _run_stream<R>(
    config: Arc<russh::server::Config>,
    socket: R,
    handler: ServerHandler,
) -> anyhow::Result<()>
where
    R: AsyncRead + AsyncWrite + Unpin + Send + 'static,
{
    let session: russh::server::RunningSession<ServerHandler> =
        russh::server::run_stream(config, socket, handler).await?;
    session.await?;
    Ok(())
}

pub struct HandleWrapper(pub Handle);

impl Debug for HandleWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HandleWrapper")
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ServerHandlerError {
    #[error("channel disconnected")]
    ChannelSend,
}

pub struct ServerHandler {
    pub handle: Option<Handle>,
    pub event_tx: UnboundedSender<ServerHandlerEvent>,
}

impl ServerHandler {
    fn send_event(&self, event: ServerHandlerEvent) -> Result<(), ServerHandlerError> {
        self.event_tx
            .send(event)
            .map_err(|_| ServerHandlerError::ChannelSend)
    }
    pub async fn loop_handle(&mut self, handle: Handle) {}
}

#[async_trait]
impl russh::server::Handler for ServerHandler {
    type Error = anyhow::Error;
    async fn auth_succeeded(&mut self, session: &mut Session) -> Result<(), Self::Error> {
        self.handle = Some(session.handle());
        println!("auth_succeed: {}", Utc::now().to_string());
        Ok(())
    }

    async fn auth_publickey(
        &mut self,
        user: &str,
        key: &russh::keys::key::PublicKey,
    ) -> Result<Auth, Self::Error> {
        let user = Secret::new(user.to_string());
        let (tx, rx) = oneshot::channel();

        self.send_event(ServerHandlerEvent::AuthPublicKey(user, key.clone(), tx))?;

        let result = rx.await.unwrap_or(Auth::UnsupportedMethod);
        Ok(result)
    }

    async fn auth_password(&mut self, user: &str, password: &str) -> Result<Auth, Self::Error> {
        println!(
            "date:{}, user: {}, password: {}",
            Utc::now().to_string(),
            user,
            password
        );
        Ok(Auth::Accept)
    }

    async fn data(
        &mut self,
        channel: ChannelId,
        data: &[u8],
        _session: &mut Session,
    ) -> Result<(), Self::Error> {
        println!(
            "data: {}, channelId: {}, data: {}",
            Utc::now().to_string(),
            channel,
            String::from_utf8(data.to_vec())?
        );
        Ok(())
    }

    async fn extended_data(
        &mut self,
        channel: ChannelId,
        code: u32,
        data: &[u8],
        _session: &mut Session,
    ) -> Result<(), Self::Error> {
        println!(
            "extended_data: {}, channelId: {}, data: {}",
            Utc::now().to_string(),
            channel,
            std::str::from_utf8(data)?
        );
        Ok(())
    }

    async fn channel_open_session(
        &mut self,
        channel: Channel<Msg>,
        _session: &mut Session,
    ) -> Result<bool, Self::Error> {
        let mut new_chan = channel;
        println!(
            "channel_open_session: {}, channelId: {}",
            Utc::now().to_string(),
            new_chan.id()
        );
        tokio::spawn(async move {
            loop {
                match new_chan.wait().await {
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
                        tokio::time::sleep(Duration::from_secs(2)).await;
                        new_chan
                            .extended_data(3, "SSSSSSSSSSSSSSS".as_bytes())
                            .await
                            .unwrap();
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
                        println!(
                            "receve none: {}, channelId: {}",
                            Utc::now().to_string(),
                            new_chan.id()
                        );
                        break;
                    }
                    msg => {
                        println!("msg = {:?}", msg);
                    }
                }
            }
        });
        Ok(true)
    }

    async fn channel_close(
        &mut self,
        channel: ChannelId,
        _session: &mut Session,
    ) -> Result<(), Self::Error> {
        println!(
            "channel_close: {}, channelId: {}",
            Utc::now().to_string(),
            channel
        );
        Ok(())
    }
    async fn channel_eof(
        &mut self,
        channel: ChannelId,
        _session: &mut Session,
    ) -> Result<(), Self::Error> {
        println!(
            "channel_eof: {}, channelId: {}",
            Utc::now().to_string(),
            channel
        );
        Ok(())
    }

    async fn signal(
        &mut self,
        channel: ChannelId,
        signal_name: russh::Sig,
        _session: &mut Session,
    ) -> Result<(), Self::Error> {
        println!("signal: {}, channelId: {}", Utc::now().to_string(), channel);
        Ok(())
    }

    // async fn auth_publickey(
    //     &mut self,
    //     user: &str,
    //     key: &russh::keys::key::PublicKey,
    // ) -> Result<Auth, Self::Error> {
    //     let user = Secret::new(user.to_string());
    //     let (tx, rx) = oneshot::channel();

    //     self.send_event(ServerHandlerEvent::AuthPublicKey(user, key.clone(), tx))?;

    //     let result = rx.await.unwrap_or(Auth::UnsupportedMethod);
    //     Ok(result)
    // }

    // async fn auth_password(&mut self, user: &str, password: &str) -> Result<Auth, Self::Error> {
    //     let user = Secret::new(user.to_string());
    //     let password = Secret::new(password.to_string());

    //     let (tx, rx) = oneshot::channel();

    //     self.send_event(ServerHandlerEvent::AuthPassword(user, password, tx))?;

    //     let result = rx.await.unwrap_or(Auth::UnsupportedMethod);
    //     Ok(result)
    // }

    // async fn data(
    //     &mut self,
    //     channel: ChannelId,
    //     data: &[u8],
    //     _session: &mut Session,
    // ) -> Result<(), Self::Error> {
    //     let channel = ServerChannelId(channel);
    //     let data = Bytes::from(data.to_vec());

    //     let (tx, rx) = oneshot::channel();

    //     self.send_event(ServerHandlerEvent::Data(channel, data, tx))?;

    //     let _ = rx.await;
    //     Ok(())
    // }

    // async fn extended_data(
    //     &mut self,
    //     channel: ChannelId,
    //     code: u32,
    //     data: &[u8],
    //     _session: &mut Session,
    // ) -> Result<(), Self::Error> {
    //     let channel = ServerChannelId(channel);
    //     let data = Bytes::from(data.to_vec());
    //     let (tx, rx) = oneshot::channel();

    //     self.send_event(ServerHandlerEvent::ExtendedData(channel, data, code, tx))?;
    //     let _ = rx.await;
    //     Ok(())
    // }

    // async fn channel_open_session(
    //     &mut self,
    //     channel: Channel<Msg>,
    //     _session: &mut Session,
    // ) -> Result<bool, Self::Error> {
    //     let (tx, rx) = oneshot::channel();

    //     self.send_event(ServerHandlerEvent::ChannelOpenSession(
    //         ServerChannelId(channel.id()),
    //         tx,
    //     ))?;

    //     let allowed = rx.await.unwrap_or(false);
    //     Ok(allowed)
    // }

    // async fn channel_close(
    //     &mut self,
    //     channel: ChannelId,
    //     _session: &mut Session,
    // ) -> Result<(), Self::Error> {
    //     let channel = ServerChannelId(channel);
    //     let (tx, rx) = oneshot::channel();
    //     self.send_event(ServerHandlerEvent::ChannelClose(channel, tx))?;
    //     let _ = rx.await;
    //     Ok(())
    // }
    // async fn channel_eof(
    //     &mut self,
    //     channel: ChannelId,
    //     _session: &mut Session,
    // ) -> Result<(), Self::Error> {
    //     let channel = ServerChannelId(channel);
    //     let (tx, rx) = oneshot::channel();

    //     self.event_tx
    //         .send(ServerHandlerEvent::ChannelEof(channel, tx))
    //         .map_err(|_| ServerHandlerError::ChannelSend)?;

    //     let _ = rx.await;
    //     Ok(())
    // }

    // async fn signal(
    //     &mut self,
    //     channel: ChannelId,
    //     signal_name: russh::Sig,
    //     _session: &mut Session,
    // ) -> Result<(), Self::Error> {
    //     let (tx, rx) = oneshot::channel();
    //     self.send_event(ServerHandlerEvent::Signal(
    //         ServerChannelId(channel),
    //         signal_name,
    //         tx,
    //     ))?;
    //     let _ = rx.await;
    //     Ok(())
    // }

    // async fn auth_succeeded(&mut self, session: &mut Session) -> Result<(), Self::Error> {
    //     let handle = session.handle();
    //     self.send_event(ServerHandlerEvent::Authenticated(HandleWrapper(handle)))?;
    //     Ok(())
    // }

    async fn subsystem_request(
        &mut self,
        channel: ChannelId,
        name: &str,
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        let name = name.to_string();
        let (tx, rx) = oneshot::channel();

        self.send_event(ServerHandlerEvent::SubsystemRequest(
            ServerChannelId(channel),
            name,
            tx,
        ))?;

        if rx.await.unwrap_or(false) {
            session.channel_success(channel)
        } else {
            session.channel_failure(channel)
        }

        Ok(())
    }

    async fn pty_request(
        &mut self,
        channel: ChannelId,
        term: &str,
        col_width: u32,
        row_height: u32,
        pix_width: u32,
        pix_height: u32,
        modes: &[(Pty, u32)],
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        let term = term.to_string();
        let modes = modes
            .iter()
            .take_while(|x| (x.0 as u8) > 0 && (x.0 as u8) < 160)
            .map(Clone::clone)
            .collect();

        let (tx, rx) = oneshot::channel();

        self.send_event(ServerHandlerEvent::PtyRequest(
            ServerChannelId(channel),
            PtyRequest {
                term,
                col_width,
                row_height,
                pix_width,
                pix_height,
                modes,
            },
            tx,
        ))?;

        let _ = rx.await;
        session.channel_success(channel);
        Ok(())
    }

    async fn shell_request(
        &mut self,
        channel: ChannelId,
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        let (tx, rx) = oneshot::channel();

        self.send_event(ServerHandlerEvent::ShellRequest(
            ServerChannelId(channel),
            tx,
        ))?;

        if rx.await.unwrap_or(false) {
            session.channel_success(channel)
        } else {
            session.channel_failure(channel)
        }

        Ok(())
    }

    async fn auth_publickey_offered(
        &mut self,
        user: &str,
        key: &russh::keys::key::PublicKey,
    ) -> Result<Auth, Self::Error> {
        let user = Secret::new(user.to_string());
        let (tx, rx) = oneshot::channel();

        self.send_event(ServerHandlerEvent::AuthPublicKeyOffer(
            user,
            key.clone(),
            tx,
        ))?;

        Ok(rx.await.unwrap_or(Auth::Reject {
            proceed_with_methods: None,
        }))
    }

    async fn auth_keyboard_interactive(
        &mut self,
        user: &str,
        _submethods: &str,
        response: Option<russh::server::Response<'async_trait>>,
    ) -> Result<Auth, Self::Error> {
        let user = Secret::new(user.to_string());
        let response = response
            .and_then(|mut r| r.next())
            .and_then(|b| String::from_utf8(b.to_vec()).ok())
            .map(Secret::new);

        let (tx, rx) = oneshot::channel();

        self.send_event(ServerHandlerEvent::AuthKeyboardInteractive(
            user, response, tx,
        ))?;

        let result = rx.await.unwrap_or(Auth::UnsupportedMethod);
        Ok(result)
    }

    async fn window_change_request(
        &mut self,
        channel: ChannelId,
        col_width: u32,
        row_height: u32,
        pix_width: u32,
        pix_height: u32,
        _session: &mut Session,
    ) -> Result<(), Self::Error> {
        let (tx, rx) = oneshot::channel();
        self.send_event(ServerHandlerEvent::WindowChangeRequest(
            ServerChannelId(channel),
            PtyRequest {
                term: "".to_string(),
                col_width,
                row_height,
                pix_width,
                pix_height,
                modes: vec![],
            },
            tx,
        ))?;
        let _ = rx.await;
        Ok(())
    }

    async fn exec_request(
        &mut self,
        channel: ChannelId,
        data: &[u8],
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        let data = Bytes::from(data.to_vec());
        let (tx, rx) = oneshot::channel();
        self.send_event(ServerHandlerEvent::ExecRequest(
            ServerChannelId(channel),
            data,
            tx,
        ))?;

        if rx.await.unwrap_or(false) {
            session.channel_success(channel)
        } else {
            session.channel_failure(channel)
        }

        Ok(())
    }

    async fn env_request(
        &mut self,
        channel: ChannelId,
        variable_name: &str,
        variable_value: &str,
        _session: &mut Session,
    ) -> Result<(), Self::Error> {
        let variable_name = variable_name.to_string();
        let variable_value = variable_value.to_string();
        let (tx, rx) = oneshot::channel();
        self.send_event(ServerHandlerEvent::EnvRequest(
            ServerChannelId(channel),
            variable_name,
            variable_value,
            tx,
        ))?;
        let _ = rx.await;
        Ok(())
    }

    async fn channel_open_direct_tcpip(
        &mut self,
        channel: Channel<Msg>,
        host_to_connect: &str,
        port_to_connect: u32,
        originator_address: &str,
        originator_port: u32,
        _session: &mut Session,
    ) -> Result<bool, Self::Error> {
        let host_to_connect = host_to_connect.to_string();
        let originator_address = originator_address.to_string();
        let (tx, rx) = oneshot::channel();
        self.send_event(ServerHandlerEvent::ChannelOpenDirectTcpIp(
            ServerChannelId(channel.id()),
            DirectTCPIPParams {
                host_to_connect,
                port_to_connect,
                originator_address,
                originator_port,
            },
            tx,
        ))?;
        let allowed = rx.await.unwrap_or(false);
        Ok(allowed)
    }

    async fn x11_request(
        &mut self,
        channel: ChannelId,
        single_conection: bool,
        x11_auth_protocol: &str,
        x11_auth_cookie: &str,
        x11_screen_number: u32,
        _session: &mut Session,
    ) -> Result<(), Self::Error> {
        let x11_auth_protocol = x11_auth_protocol.to_string();
        let x11_auth_cookie = x11_auth_cookie.to_string();
        let (tx, rx) = oneshot::channel();
        self.send_event(ServerHandlerEvent::X11Request(
            ServerChannelId(channel),
            X11Request {
                single_conection,
                x11_auth_protocol,
                x11_auth_cookie,
                x11_screen_number,
            },
            tx,
        ))?;
        let _ = rx.await;
        Ok(())
    }

    async fn tcpip_forward(
        &mut self,
        address: &str,
        port: &mut u32,
        session: &mut Session,
    ) -> Result<bool, Self::Error> {
        let address = address.to_string();
        let port = *port;
        let (tx, rx) = oneshot::channel();
        self.send_event(ServerHandlerEvent::TcpIpForward(address, port, tx))?;
        let allowed = rx.await.unwrap_or(false);
        if allowed {
            session.request_success()
        } else {
            session.request_failure()
        }
        Ok(allowed)
    }

    async fn cancel_tcpip_forward(
        &mut self,
        address: &str,
        port: u32,
        session: &mut Session,
    ) -> Result<bool, Self::Error> {
        let address = address.to_string();
        let (tx, rx) = oneshot::channel();
        self.send_event(ServerHandlerEvent::CancelTcpIpForward(address, port, tx))?;
        let allowed = rx.await.unwrap_or(false);
        if allowed {
            session.request_success()
        } else {
            session.request_failure()
        }
        Ok(allowed)
    }
}

impl Drop for ServerHandler {
    fn drop(&mut self) {
        debug!("Dropped");
        let _ = self.event_tx.send(ServerHandlerEvent::Disconnect);
    }
}

impl Debug for ServerHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ServerHandler")
    }
}

#[derive(Debug)]
pub enum ServerHandlerEvent {
    Authenticated(HandleWrapper),
    ChannelOpenSession(ServerChannelId, oneshot::Sender<bool>),
    SubsystemRequest(ServerChannelId, String, oneshot::Sender<bool>),
    PtyRequest(ServerChannelId, PtyRequest, oneshot::Sender<()>),
    ShellRequest(ServerChannelId, oneshot::Sender<bool>),
    AuthPublicKey(Secret<String>, PublicKey, oneshot::Sender<Auth>),
    AuthPublicKeyOffer(Secret<String>, PublicKey, oneshot::Sender<Auth>),
    AuthPassword(Secret<String>, Secret<String>, oneshot::Sender<Auth>),
    AuthKeyboardInteractive(
        Secret<String>,
        Option<Secret<String>>,
        oneshot::Sender<Auth>,
    ),
    Data(ServerChannelId, Bytes, oneshot::Sender<()>),
    ExtendedData(ServerChannelId, Bytes, u32, oneshot::Sender<()>),
    ChannelClose(ServerChannelId, oneshot::Sender<()>),
    ChannelEof(ServerChannelId, oneshot::Sender<()>),
    WindowChangeRequest(ServerChannelId, PtyRequest, oneshot::Sender<()>),
    Signal(ServerChannelId, Sig, oneshot::Sender<()>),
    ExecRequest(ServerChannelId, Bytes, oneshot::Sender<bool>),
    ChannelOpenDirectTcpIp(ServerChannelId, DirectTCPIPParams, oneshot::Sender<bool>),
    EnvRequest(ServerChannelId, String, String, oneshot::Sender<()>),
    X11Request(ServerChannelId, X11Request, oneshot::Sender<()>),
    TcpIpForward(String, u32, oneshot::Sender<bool>),
    CancelTcpIpForward(String, u32, oneshot::Sender<bool>),
    Disconnect,
}
