use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};
use futures::future::join_all;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let base_ip = "192.168.1."; // Base part of the IP address
    let start_port = 20;         // Starting port
    let end_port = 1024;         // Ending port
    let timeout_duration = Duration::from_secs(1); // Timeout for each connection attempt

    // Generate tasks to scan each IP from 192.168.1.1 to 192.168.1.255
    let tasks: Vec<_> = (1..=255).flat_map(|i| {
        (start_port..=end_port).map(move |port| {
            let ip = format!("{}{}", base_ip, i); // Build the full IP address (192.168.1.1 to 192.168.1.255)
            let addr = format!("{}:{}", ip, port);
            let socket_addr: SocketAddr = addr.parse().unwrap();
            tokio::spawn(async move {
                scan_port(socket_addr, timeout_duration).await
            })
        })
    }).collect();

    // Wait for all tasks to finish
    let _results = join_all(tasks).await;
}

// Function to scan a single port
async fn scan_port(addr: SocketAddr, timeout_duration: Duration) {
    let result = timeout(timeout_duration, TcpStream::connect(&addr)).await;

    match result {
        Ok(Ok(_stream)) => {
            println!("Port {} is open on {}", addr.port(), addr.ip());
        }
        Ok(Err(_)) => {
            // Port is closed, no output
        }
        Err(_) => {
            // Timeout, port might be closed or unreachable
        }
    }
}
