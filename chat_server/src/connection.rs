use std::{io::{Read, Write}, net::TcpStream, sync::{Arc, Mutex}};
use colored::Colorize;

pub fn handle_connection(mut stream: TcpStream, streams: Arc<Mutex<Vec<TcpStream>>>) {
    loop{
        let mut buffer = [0; 512];

        let bytes_read = stream.read(&mut buffer).unwrap_or_else(|e| {
            eprintln!("{}", format!("Failed to read: {}", e).red());
            std::process::exit(1);
        });
        
        if bytes_read == 0 {
            println!("{}", "Connection closed by server".yellow());
            return;
        }
        
        let received_message = String::from_utf8_lossy(&buffer[..bytes_read]);
        let _ = received_message.trim();

        // Lock the streams vector to send message to all other clients
        let mut locked_streams = streams.lock().expect("Failed to lock streams");

        if let Ok(peer_addr) = stream.peer_addr() {
            for tcpstream in &mut *locked_streams {
                // Check if the tcpstream is the same as the current stream by comparing addresses
                if let Ok(addr) = tcpstream.peer_addr() {
                    if addr == peer_addr {
                        continue;
                    }
                }
                
                // Send the message to all other clients
                let _ = tcpstream.write(received_message.as_bytes());
                let _ = tcpstream.flush();
            }
    }
    }
}
