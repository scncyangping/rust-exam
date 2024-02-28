use crate::api::ApiError;
use crate::AppState;
use askama::filters::format;
use axum::extract::ws::{Message, WebSocket};
use axum::extract::{State, WebSocketUpgrade};
use axum::response::IntoResponse;
use futures::{SinkExt, StreamExt};
use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::{broadcast, Mutex};

pub struct ChatState {
    // 存储用户名
    pub user_set: Mutex<HashSet<String>>,
    // 广播
    pub tx: broadcast::Sender<String>,
}

impl ChatState {
    pub fn new(num: usize) -> Self {
        let (tx, _) = broadcast::channel(num);
        ChatState {
            tx,
            user_set: Mutex::new(HashSet::new()),
        }
    }
}

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| websocket(socket, state))
}

async fn websocket(stream: WebSocket, state: Arc<AppState>) {
    let (mut sender, mut receiver) = stream.split();
    // Username gets set in the receive loop, if it's valid.
    let mut username = String::new();

    while let Some(Ok(msg)) = receiver.next().await {
        if let Message::Text(name) = msg {
            check_username(&state, &mut username, &name).await;
            if !username.is_empty() {
                break;
            } else {
                let _ = sender
                    .send(Message::Text(String::from("Username already taken.")))
                    .await;
                return;
            }
        }
    }

    let mut rx = state.chat_state.tx.subscribe();
    let msg = format!("{username} joined.");
    tracing::debug!("{msg}");
    let _ = state.chat_state.tx.send(msg);

    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            // In any websocket error, break loop.
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    let tx = state.chat_state.tx.clone();
    let name = username.clone();

    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            // Add username before message.
            let _ = tx.send(format!("{name}: {text}"));
        }
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };

    // Send "user left" message (similar to "joined" above).
    let msg = format!("{username} left.");
    tracing::debug!("{msg}");
    state.chat_state.user_set.lock().await.remove(&username);
    let _ = state.chat_state.tx.send(msg);

    // Remove username from map so new clients can take it again.
    state.chat_state.user_set.lock().await.remove(&username);
    ()
}

async fn check_username(state: &AppState, string: &mut String, name: &str) {
    let mut user_set = state.chat_state.user_set.lock().await;

    if !user_set.contains(name) {
        user_set.insert(name.to_owned());
        string.push_str(name);
    }
}
