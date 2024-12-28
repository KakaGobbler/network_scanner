use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};
use futures::future::join_all;
use std::net::SocketAddr;
use indicatif::{ProgressBar, ProgressStyle};
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    let base_ip = "192.168.1."; // Base part of the IP address
    let start_port = 20;         // Starting port
    let end_port = 1024;         // Ending port
    let timeout_duration = Duration::from_secs(1); // Timeout for each connection attempt
    let total_ips = 255; // We are scanning from .1 to .255, so there are 255 IPs
    let total_ports = end_port - start_port + 1; // Total number of ports to scan for each IP

    // Create a shared progress bar
    let pb = Arc::new(Mutex::new(ProgressBar::new(total_ips as u64 * total_ports as u64)));  // Total progress based on both IPs and ports

    // Set the style for the progress bar
    pb.lock().unwrap().set_style(ProgressStyle::default_bar()
        .template("{bar:40} {percent}% {pos}/{len} Ports scanned")
        .unwrap() // Unwrap the Result to get the ProgressStyle
        .progress_chars("=>-")); // Correctly calling progress_chars on ProgressStyle

    // Generate tasks to scan each IP from 192.168.1.1 to 192.168.1.255
    let mut tasks = Vec::new();
    
    for i in 1..=total_ips {
        for port in start_port..=end_port {
            let ip = format!("{}{}", base_ip, i); // Build the full IP address (192.168.1.1 to 192.168.1.255)
            let addr = format!("{}:{}", ip, port);
            let socket_addr: SocketAddr = addr.parse().unwrap();

            // Clone the Arc before moving it into the closure
            let pb_clone = Arc::clone(&pb); // Clone the Arc for use in the task
            
            // Spawn the async task using the cloned Arc
            let task = tokio::spawn(async move {
                scan_port(socket_addr, timeout_duration, pb_clone).await;
            });

            tasks.push(task);
        }
    }

    // Wait for all tasks to finish
    let _results = join_all(tasks).await;

    pb.lock().unwrap().finish_with_message("Scan complete!");
}

// Function to scan a single port
async fn scan_port(addr: SocketAddr, timeout_duration: Duration, pb: Arc<Mutex<ProgressBar>>) {
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

    // Increment the progress bar when the port scan is completed
    let pb = pb.lock().unwrap();
    pb.inc(1); // Increment progress by 1
}
