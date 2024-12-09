use crate::windows::{create_server, send_command};
use std::io::{stdin, BufRead, Write};
use std::process::exit;

mod windows;

fn main() {
    let server = match create_server() {
        Ok(server) => {
            println!("Server created");
            server
        }
        Err(e) => {
            eprintln!("Failed to create server: {:?}", e);
            exit(1);
        }
    };

    loop {
        std::thread::sleep(std::time::Duration::from_secs(5));
        if let Err(e) = send_command(&server, "list") {
            eprintln!("Error: {:?}", e);
        }

    }
}
