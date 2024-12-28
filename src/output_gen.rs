use std::fs::OpenOptions;
use std::io::{self, Write};

pub fn write_output_to_file(file_path: &str, content: String, write_to_file: bool) -> io::Result<()> {
    if write_to_file {
        // Open the file in append mode, create if it doesn't exist
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_path)?;
        
        writeln!(file, "{}", content)?;
    } else {
        // Print to the console instead
        println!("{}", content);
    }
    Ok(())
}