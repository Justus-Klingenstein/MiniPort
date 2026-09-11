mod config;
mod device;
mod discovery;
mod jobs;
mod network;
mod protocol;
mod security;
mod storage;
mod transfer;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("MiniPort! 0.1.0");
    println!("================");

    let config = config::Config::load()?;
    let identity = device::Identity::load_or_create(&config)?;

    println!("Device name: {}", identity.name);
    println!("Device ID:   {}", identity.id);

    discovery::start(&identity).await?;

    Ok(())
}