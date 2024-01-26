use std::io;
use std::time::{Duration, Instant};
use tokio::io::AsyncReadExt;
mod slf_mod;
#[tokio::main]
async fn main() {
    let future = slf_mod::future::Delay {
        when: Instant::now() + Duration::from_millis(10),
    };

    let out = future.await;

    assert_eq!(out, "done");
}
