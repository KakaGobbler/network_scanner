use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};
use futures::future::join_all;
use std::net::SocketAddr;
use indicatif::{ProgressBar, ProgressStyle};
use std::sync::{Arc, Mutex};
use std::io::{self, Write}; // Import necessary libraries for user input
use std::fs; // For checking file existence and renaming

mod output_gen; // Import the output_gen module

#[tokio::main]
async fn main() {
    let base_ip = "192.168.1."; // Base part of the IP address
    let start_port = 20;         // Starting port
    let end_port = 1024;         // Ending port
    let timeout_duration = Duration::from_secs(1); // Timeout for each connection attempt
    let total_ips = 255; // We are scanning from .1 to .255, so there are 255 IPs
    let total_ports = end_port - start_port + 1; // Total number of ports to scan for each IP

    // Prompt the user for input on whether to write output to a file or just print
    print!("Do you want to save the output to a file? (y/n): ");
    let mut input = String::new();
    io::stdout().flush().unwrap(); // Ensure prompt is displayed
    io::stdin().read_line(&mut input).unwrap();
    let write_to_file = input.trim().to_lowercase() == "y"; // Check if user entered 'y'

    let mut output_file = "output.txt".to_string(); // Default file name

    // If the user wants to save to a file, ensure unique filename
    if write_to_file {
        output_file = get_unique_filename(output_file);
        println!("Output will be saved to: {}", output_file);
    }

    // Create a shared progress bar only if the user chooses to write to a file
    let pb = if write_to_file {
        let pb = Arc::new(Mutex::new(ProgressBar::new(total_ips as u64 * total_ports as u64)));  

        // Set the style for the progress bar
        pb.lock().unwrap().set_style(ProgressStyle::default_bar()
            .template("{bar:40} {percent}% {pos}/{len} Ports scanned")
            .unwrap()
            .progress_chars("=>-"));
        Some(pb)
    } else {
        None // No progress bar if not writing to a file
    };

    // Print a message indicating the scan is starting
    println!("Scanning IPs and Ports...\n");

    // Generate tasks to scan each IP from 192.168.1.1 to 192.168.1.255
    let mut tasks = Vec::new();

    for i in 1..=total_ips {
        for port in start_port..=end_port {
            let ip = format!("{}{}", base_ip, i); // Build the full IP address (192.168.1.1 to 192.168.1.255)
            let addr = format!("{}:{}", ip, port);
            let socket_addr: SocketAddr = addr.parse().unwrap();

            // Clone the output_file to avoid moving it
            let output_file_clone = output_file.clone();

            // Clone the Arc before moving it into the closure, if a progress bar exists
            let pb_clone = if let Some(ref pb) = pb {
                Some(Arc::clone(pb)) // Clone the Arc if progress bar is enabled
            } else {
                None // No progress bar to clone if not enabled
            };

            // Spawn the async task using the cloned Arc
            let task = tokio::spawn(async move {
                scan_port(socket_addr, timeout_duration, pb_clone, write_to_file, &output_file_clone).await;
            });

            tasks.push(task);
        }
    }

    // Wait for all tasks to finish
    let _results = join_all(tasks).await;

    // If progress bar was used, finish it and print the completion message
    if let Some(pb) = pb {
        pb.lock().unwrap().finish_with_message("Scan complete!");
    }
}

// Function to scan a single port
async fn scan_port(addr: SocketAddr, timeout_duration: Duration, pb: Option<Arc<Mutex<ProgressBar>>>, write_to_file: bool, output_file: &str) {
    let result = timeout(timeout_duration, TcpStream::connect(&addr)).await;

    let content = match result {
        Ok(Ok(_stream)) => {
            format!("Port {} is open on {}", addr.port(), addr.ip()) // Open port
        }
        Ok(Err(_)) => {
            String::new() // No output if port is closed
        }
        Err(_) => {
            String::new() // Timeout or unreachable port
        }
    };

    // If the content is not empty, write to the file or print it
    if !content.is_empty() {
        output_gen::write_output_to_file(output_file, content, write_to_file).expect("Failed to write output");
    }

    // Increment the progress bar when the port scan is completed, if progress bar exists
    if let Some(pb) = pb {
        let pb = pb.lock().unwrap();
        pb.inc(1); // Increment progress by 1
    }
}

// Function to generate a unique filename
fn get_unique_filename(base_name: String) -> String {
    let mut counter = 1;
    let mut unique_name = base_name.clone();

    // Check if the file already exists, and if so, increment the counter and try again
    while fs::metadata(&unique_name).is_ok() {
        unique_name = format!("output({}).txt", counter);
        counter += 1;
    }

    unique_name
}
