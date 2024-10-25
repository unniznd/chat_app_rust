mod connection;

use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::env;
use std::sync::{Arc, Mutex};
use std::thread;

use colored::Colorize;

use crate::connection::handle_connection;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("{}","Expect hostname and port number as args".red());
        return;
    }
    let addr = &args[1];

    let listener = TcpListener::bind(addr).unwrap_or_else(|e| {
        eprintln!("{}", format!("Failed to bind: {}", e).red());
        std::process::exit(1);
    });

    println!("Listening on {}", addr);

    // Wrap streams in Arc and Mutex for safe sharing across threads
    let streams: Arc<Mutex<Vec<TcpStream>>> = Arc::new(Mutex::new(Vec::new()));
    
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established!");

        // Clone the Arc to allow passing to threads
        let streams = Arc::clone(&streams);

        {
            // Lock and modify the streams vector
            let mut locked_streams = streams.lock().expect("Failed to lock streams");

            if !locked_streams.is_empty() {
                for tcpstream in &mut *locked_streams {
                    let new_user = String::from("New User Added");
                    let _ = tcpstream.write(new_user.as_bytes());
                    let _ = tcpstream.flush();
                }
            }
            
            // Add the new stream to the list
            locked_streams.push(stream.try_clone().expect("Failed to clone stream"));
        }

        // Spawn a new thread for each connection
        thread::spawn(move || {
            handle_connection(stream, streams);
        });
    }
}
