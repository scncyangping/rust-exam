use std::fmt::{Display, Formatter};

use russh::{ChannelId, Pty};

#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
pub struct ServerChannelId(pub ChannelId);

impl Display for ServerChannelId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Debug)]
pub struct PtyRequest {
    pub term: String,
    pub col_width: u32,
    pub row_height: u32,
    pub pix_width: u32,
    pub pix_height: u32,
    pub modes: Vec<(Pty, u32)>,
}

#[derive(Clone, Debug)]
pub struct DirectTCPIPParams {
    pub host_to_connect: String,
    pub port_to_connect: u32,
    pub originator_address: String,
    pub originator_port: u32,
}

#[derive(Clone, Debug)]
pub struct X11Request {
    pub single_conection: bool,
    pub x11_auth_protocol: String,
    pub x11_auth_cookie: String,
    pub x11_screen_number: u32,
}
