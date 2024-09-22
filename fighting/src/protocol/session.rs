//! SSH SESSIOn

use std::net::SocketAddr;

use serde::{Deserialize, Serialize};
use tokio::sync::{broadcast, mpsc};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SessionHandleCommand {
    Close,
}

pub struct SSHSessionHandle {
    sender: mpsc::UnboundedSender<SessionHandleCommand>,
}

impl SSHSessionHandle {
    pub fn new() -> (Self, mpsc::UnboundedReceiver<SessionHandleCommand>) {
        let (sender, receiver) = mpsc::unbounded_channel();
        (SSHSessionHandle { sender }, receiver)
    }
}

impl SessionHandle for SSHSessionHandle {
    fn close(&mut self) {
        let _ = self.sender.send(SessionHandleCommand::Close);
    }
}

pub trait SessionHandle {
    fn close(&mut self);
}

pub struct SessionState {
    pub remote_address: Option<SocketAddr>,
    pub username: Option<String>,
    pub target: Option<Target>,
    pub handle: Box<dyn SessionHandle + Send>,
    change_sender: broadcast::Sender<()>,
}

pub struct SessionStateInit {
    pub remote_address: Option<SocketAddr>,
    pub handle: Box<dyn SessionHandle + Send>,
}

impl SessionState {
    fn new(init: SessionStateInit, change_sender: broadcast::Sender<()>) -> Self {
        SessionState {
            remote_address: init.remote_address,
            username: None,
            target: None,
            handle: init.handle,
            change_sender,
        }
    }

    pub fn emit_change(&self) {
        let _ = self.change_sender.send(());
    }
}

pub(crate) fn _default_empty_vec<T>() -> Vec<T> {
    vec![]
}
pub(crate) const fn _default_ssh_port() -> u16 {
    22
}

#[inline]
pub(crate) fn _default_username() -> String {
    "root".to_owned()
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Target {
    #[serde(default)]
    pub id: Uuid,
    pub name: String,
    #[serde(default = "_default_empty_vec")]
    pub allow_roles: Vec<String>,
    #[serde(flatten)]
    pub options: TargetOptions,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum TargetOptions {
    #[serde(rename = "ssh")]
    Ssh(TargetSSHOptions),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TargetSSHOptions {
    pub host: String,
    #[serde(default = "_default_ssh_port")]
    pub port: u16,
    #[serde(default = "_default_username")]
    pub username: String,
    #[serde(default)]
    pub allow_insecure_algos: Option<bool>,
    #[serde(default)]
    pub auth: SSHTargetAuth,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum SSHTargetAuth {
    #[serde(rename = "password")]
    Password(SshTargetPasswordAuth),
    #[serde(rename = "publickey")]
    PublicKey(SshTargetPublicKeyAuth),
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct SshTargetPasswordAuth {
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
pub struct SshTargetPublicKeyAuth {}

impl Default for SSHTargetAuth {
    fn default() -> Self {
        SSHTargetAuth::PublicKey(SshTargetPublicKeyAuth::default())
    }
}
