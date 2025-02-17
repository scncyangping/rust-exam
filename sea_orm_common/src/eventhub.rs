use std::sync::Arc;

use tokio::sync::mpsc::error::SendError;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tokio::sync::{Mutex, MutexGuard};

pub struct EventSender<E> {
    subscriptions: SubscriptionStore<E>,
}

impl<E> Clone for EventSender<E> {
    fn clone(&self) -> Self {
        EventSender {
            subscriptions: self.subscriptions.clone(),
        }
    }
}

impl<E> EventSender<E> {
    async fn cleanup_subscriptions(&self) -> MutexGuard<'_, SubscriptionStoreInner<E>> {
        let mut subscriptions = self.subscriptions.lock().await;
        subscriptions.retain(|(_, ref s)| !s.is_closed());
        subscriptions
    }
}

impl<'h, E: Clone + 'h> EventSender<E> {
    pub async fn send_all(&'h self, event: E) -> Result<(), SendError<E>> {
        let mut subscriptions = self.cleanup_subscriptions().await;

        for (ref f, ref mut s) in subscriptions.iter_mut().rev() {
            if f(&event) {
                let _ = s.send(event.clone());
            }
        }
        if subscriptions.is_empty() {
            Err(SendError(event))
        } else {
            Ok(())
        }
    }
}

impl<'h, E: 'h> EventSender<E> {
    pub async fn send_once(&'h self, event: E) -> Result<(), SendError<E>> {
        let mut subscriptions = self.cleanup_subscriptions().await;

        for (ref f, ref mut s) in subscriptions.iter_mut().rev() {
            if f(&event) {
                return s.send(event);
            }
        }

        Err(SendError(event))
    }
}

pub struct EventSubscription<E>(UnboundedReceiver<E>);

impl<E> EventSubscription<E> {
    pub fn unbox(self) -> UnboundedReceiver<E> {
        self.0
    }
    pub async fn recv(&mut self) -> Option<E> {
        self.0.recv().await
    }
}

type SubscriptionStoreInner<E> = Vec<(Box<dyn Fn(&E) -> bool + Send>, UnboundedSender<E>)>;
type SubscriptionStore<E> = Arc<Mutex<SubscriptionStoreInner<E>>>;

pub struct EventHub<E: Send> {
    subscriptions: SubscriptionStore<E>,
}

impl<E: Send> EventHub<E> {
    pub fn setup() -> (Self, EventSender<E>) {
        let subscriptions = Arc::new(Mutex::new(vec![]));
        (
            Self {
                subscriptions: subscriptions.clone(),
            },
            EventSender { subscriptions },
        )
    }

    pub async fn subscribe<F: Fn(&E) -> bool + Send + 'static>(
        &self,
        filter: F,
    ) -> EventSubscription<E> {
        let (sender, receiver) = unbounded_channel();
        let mut subscriptions = self.subscriptions.lock().await;
        subscriptions.push((Box::new(filter), sender));
        EventSubscription(receiver)
    }
}
