use crate::device::Identity;
use anyhow::Result;

pub async fn start(identity: &Identity) -> Result<()> {
    println!();
    println!("Discovery");
    println!("---------");
    println!("Starting discovery for {}", identity.name);

    Ok(())
}