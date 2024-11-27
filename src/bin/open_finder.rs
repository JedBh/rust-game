use std::io;
use std::process::Command;

fn main() {
    let mut input = String::new();

    println!("Open Finder? y/n");

    io::stdin()
        .read_line(&mut input) // read the line for y or n
        .expect("Failed to read line.");

    let input = input.trim();

    if input == "y" {
        open_finder();
    } else {
        println!("Finder was not opened.");
    }
}

fn open_finder() {
    // Use the `open` command to launch Finder
    let result = Command::new("open")
        .arg("-a") // Specify the application
        .arg("Finder") // The name of the application
        .output();

    match result {
        Ok(output) => {
            if output.status.success() {
                println!("Finder opened successfully.");
            } else {
                eprintln!(
                    "Failed to open Finder: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
            }
        }
        Err(e) => {
            eprintln!("Error executing command: {}", e);
        }
    }
}
