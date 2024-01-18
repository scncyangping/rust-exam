mod self_mod;
#[tokio::main]
async fn main() {
    self_mod::barrier::thread_semaphore().await;
}
