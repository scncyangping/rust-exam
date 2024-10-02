use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use anyhow::{Ok, Result};
use fighting::protocol::server;
#[tokio::main]
async fn main() -> Result<()> {
    let v4 = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 9099);
    server::run_server(v4).await?;
    Ok(())
}
