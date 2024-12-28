use std::net::{Ipv4Addr, SocketAddr};
use std::time::Duration;
use ping::{Ping, PingResult};
use futures::stream::FuturesUnordered;
use futures::StreamExt;

async fn scan_ip(ip: Ipv4Addr) -> Option<Ipv4Addr> {
    let ping = Ping::new();
    match ping.ping(ip.to_string().as_str(), 1) {
        Ok(PingResult::Success { .. }) => {
            println!("Host {} is online!", ip);
            Some(ip)
        }
        _ => {
            println!("Host {} is offline!", ip);
            None
        }
    }
}

async fn scan_range(start_ip: Ipv4Addr, end_ip: Ipv4Addr) {
    let mut futures = FuturesUnordered::new();
    
    let start_octets = start_ip.octets();
    let end_octets = end_ip.octets();
    
    // Loop through the range of IPs
    for i in start_octets[3]..=end_octets[3] {
        let current_ip = Ipv4Addr::new(start_octets[0], start_octets[1], start_octets[2], i);
        futures.push(scan_ip(current_ip));
    }

    // Wait for all futures to complete
    while let Some(ip) = futures.next().await {
        if let Some(ip) = ip {
            println!("Active IP: {}", ip);
        }
    }
}

#[tokio::main]
async fn main() {
    // Example input
    let start_ip = Ipv4Addr::new(192, 168, 1, 1);
    let end_ip = Ipv4Addr::new(192, 168, 1, 255);

    // Scan the range
    scan_range(start_ip, end_ip).await;
}
