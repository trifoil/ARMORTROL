use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                // Connection was closed by the client
                //println!("Connection closed by client.");
                break;
            }
            Ok(bytes_read) => {
                println!("Received: {}", String::from_utf8_lossy(&buffer[..bytes_read]));
                stream.write_all(b"Message received\n").unwrap();
                stream.flush().unwrap();
            }
            Err(e) => {
                eprintln!("Failed to read from socket: {}", e);
                break;
            }
        }
    }
}

pub fn run_server(host: &str, port: u16) -> std::io::Result<()> {
    let addr = format!("{}:{}", host, port);
    let listener = TcpListener::bind(addr)?;

    println!("Server listening on {}:{}", host, port);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Failed to accept connection: {}", e);
            }
        }
    }

    Ok(())
}

fn main() {
    let host = "192.168.1.160";
    let port: u16 = 8069;
    if let Err(e) = run_server(host, port) {
        eprintln!("Error: {}", e);
    }
}
