// use std::io::{Read, stdout, Write};
// use std::net::TcpListener;
// use std::time::Duration;


// pub fn run(bind_host: &String, port: &u16) -> Result<(), String> {
//     let addr = format!("{}:{}", bind_host, port);
//     let listener = TcpListener::bind(addr.clone()).map_err(|_| format!("Failed to bind to {}", addr))?;

//     for stream in listener.incoming() {
//         match stream {
//             Ok(mut s) => {
//                 println!("Connection accepted");
                
//                 s.set_read_timeout(Some(Duration::new(5,0))).map_err(|_| "failed to set read timeout")?;

//                 let mut buf = [0; 128];
//                 let mut read_bytes = 0;
//                 while read_bytes == 0 {
//                     read_bytes = s.read(&mut buf).map_err(|_| "failed to read from socket")?;
//                     println!("received bytes {}", read_bytes);
//                 }
//                 stdout().write(&buf[0..read_bytes]).map_err(|_| "failed to write to stdout")?;
//                 stdout().flush().unwrap();
//             }
//             Err(e) => {
//                 println!("Error while accepting incoming connection - {}", e);
//             }
//         }
//     }

//     Ok(())
// }

// fn main() {
//     println!("Hello, world!");
//     let host = "192.168.1.160".to_string();
//     let port:u16 = 8069;
//     run(&host, &port);
// }


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
                println!("Connection closed by client.");
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
