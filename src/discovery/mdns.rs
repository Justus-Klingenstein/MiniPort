use crate::device::Identity;
use anyhow::Result;
use mdns_sd::{ServiceDaemon, ServiceEvent, ServiceInfo};
use tokio::time::{sleep, Duration};

const SERVICE_TYPE: &str = "_miniport._tcp.local.";
const MINI_PORT: u16 = 39400;

pub async fn start(identity: &Identity) -> Result<()> {
    println!();
    println!("Discovery");
    println!("---------");

    let mdns = ServiceDaemon::new()?;

    let properties = [
        ("id", identity.id.to_string()),
        ("name", identity.name.clone()),
        ("version", env!("CARGO_PKG_VERSION").to_string()),
    ];

    let service = ServiceInfo::new(
        SERVICE_TYPE,
        &identity.id.to_string(),
        &format!("{}.local.", identity.name),
        "",
        MINI_PORT,
        &properties,
    )?
    .enable_addr_auto();

    mdns.register(service)?;

    println!("Service registered:");
    println!("  Type:    {SERVICE_TYPE}");
    println!("  Port:    {MINI_PORT}");
    println!("  Name:    {}", identity.name);
    println!("  ID:      {}", identity.id);

    let receiver = mdns.browse(SERVICE_TYPE)?;

    println!();
    println!("Searching for MiniPort! devices...");
    println!("Press Ctrl+C to stop.");
    println!();

    loop {
        match receiver.recv_async().await {
            Ok(ServiceEvent::ServiceResolved(info)) => {
                let instance = info.get_fullname();

                // Don't report ourselves.
                let own_id = identity.id.to_string();

                let discovered_id = info
                    .get_property_val_str("id")
                    .unwrap_or("");

                if discovered_id == own_id {
                    continue;
                }

                let name = info
                    .get_property_val_str("name")
                    .unwrap_or("Unknown device");

                println!("Found MiniPort! device:");
                println!("  Name:    {name}");
                println!("  ID:      {discovered_id}");
                println!("  Host:    {}", info.get_hostname());
                println!("  Port:    {}", info.get_port());

                for address in info.get_addresses() {
                    println!("  Address: {address}");
                }

                println!("  Service: {instance}");
                println!();
            }

            Ok(ServiceEvent::ServiceRemoved(_, fullname)) => {
                println!("MiniPort! device disappeared:");
                println!("  {fullname}");
                println!();
            }

            Ok(_) => {
                // Other mDNS events are not interesting to us yet.
            }

            Err(error) => {
                eprintln!("Discovery error: {error}");
                break;
            }
        }

        // Prevent an extremely tight loop in case the channel closes.
        sleep(Duration::from_millis(10)).await;
    }
}