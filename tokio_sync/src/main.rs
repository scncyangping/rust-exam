use std::io;
use std::time::Duration;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
mod slf_mod;
#[tokio::main]
async fn main() -> io::Result<()> {
    slf_mod::io::test_manu_copy("127.0.0.1:9898").await?;
    Ok(())
}
