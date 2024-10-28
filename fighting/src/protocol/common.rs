use std::{
    fmt::{Display, Formatter},
    sync::Arc,
};

use russh::{ChannelId, Pty};
use tokio::sync::{mpsc::UnboundedSender, Mutex, MutexGuard};
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

// ************ EventHub and EventSender start ************

type SubscriptionStoreInner<E> = Vec<(Box<dyn Fn(&E) -> bool + Send>, UnboundedSender<E>)>;

type SubscriptionStore<E> = Arc<Mutex<SubscriptionStoreInner<E>>>;

pub struct EventSender<E> {
    subscriptions: SubscriptionStore<E>,
}

impl<E> Clone for EventSender<E> {
    fn clone(&self) -> Self {
        Self {
            subscriptions: self.subscriptions.clone(),
        }
    }
}

impl<E> EventSender<E> {
    async fn cleanup_subscriptions(&self) -> MutexGuard<'_, SubscriptionStoreInner<E>> {
        let mut subs = self.subscriptions.lock().await;
        subs.retain(|(_, ref s)| !s.is_closed());
        subs
    }
}
// ************ EventHub and EventSender end ************
