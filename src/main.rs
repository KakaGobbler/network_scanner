use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};
use std::net::SocketAddr;
use futures::future::join_all;

#[tokio::main]
async fn main() {
    let ip_range = "192.168.1."; // You can adjust this for your own network
    let start_port = 20;
    let end_port = 1024; // Define the range of ports you want to scan

    // Scan all addresses in the range
    let tasks: Vec<_> = (start_port..=end_port).map(|port| {
        let ip = format!("{}{}", ip_range, "1"); // Replace "1" with other IPs in range if necessary
        let addr = format!("{}:{}", ip, port);
        let socket_addr: SocketAddr = addr.parse().unwrap();

        tokio::spawn(async move {
            scan_port(socket_addr).await
        })
    }).collect();

    // Wait for all tasks to finish
    let _results = join_all(tasks).await;
}

// Function to scan individual port
async fn scan_port(addr: SocketAddr) {
    let result = timeout(Duration::from_secs(1), TcpStream::connect(&addr)).await;

    match result {
        Ok(Ok(_stream)) => {
            println!("Port {} is open", addr.port());
        }
        Ok(Err(_)) => {
            // Port is closed, no output or could print if desired
        }
        Err(_) => {
            // Timeout, port may be closed or not responding
        }
    }
}
