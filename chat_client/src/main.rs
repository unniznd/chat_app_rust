use std::io::{self, Write, Read};
use std::net::TcpStream;
use std::env;
use std::thread;
use colored::Colorize;

// Function to handle reading from the server
fn read_from_server(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    
    loop {
        let bytes_read = stream.read(&mut buffer).unwrap_or_else(|e| {
            eprintln!("{}", format!("Failed to read: {}", e).red());
            std::process::exit(1);
        });
        
        if bytes_read == 0 {
            println!("{}", "Connection closed by server".yellow());
            break;
        }
        
        let received_message = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
        let _ = received_message.trim();
        println!("{received_message}");
    }
}

// Function to handle writing to the server
fn write_to_server(mut stream: TcpStream) {
    let stdin = io::stdin();
    
    loop {
        let mut input = String::new();
        stdin.read_line(&mut input).expect("Failed to read from stdin");
        let _ = input.trim();
        if input.is_empty() {
            continue;
        }
        
        stream.write_all(input.as_bytes()).unwrap_or_else(|e| {
            eprintln!("{}", format!("Failed to write: {}", e).red());
            std::process::exit(1);
        });
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("{}", "Expect hostname and port number as args".red());
        return;
    }
    let addr = &args[1];

    let stream = TcpStream::connect(addr).unwrap_or_else(|e| {
        eprintln!("{}", format!("Failed to connect: {}", e).red());
        std::process::exit(1);
    });

    // Clone the TcpStream for use in each thread
    let read_stream = stream.try_clone().expect("Failed to clone stream for reading");
    let write_stream = stream.try_clone().expect("Failed to clone stream for writing");

    // Spawn a thread for reading
    thread::spawn(move || {
        read_from_server(read_stream);
    });

    // Main thread for writing
    write_to_server(write_stream);
}
