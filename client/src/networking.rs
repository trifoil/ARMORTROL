use std::net::TcpStream;
use std::io::{Read, stdout, Write};
use std::net::TcpListener;


pub fn run_client(host: &String, port: &u16) -> Result<(), String> {
    let addr = format!("{}:{}", host, port);
    let mut client = TcpStream::connect(addr.as_str()).map_err(|_| format!("failed to connect to {}", addr))?;

    client.write("hello, TCP".as_bytes()).map_err(|_| format!("failed to send"))?;

    Ok(())
}

pub fn run_serve(bind_host: &String, port: &u16) -> Result<(), String> {
    let addr = format!("{}:{}", bind_host, port);
    let listener = TcpListener::bind(addr.clone()).map_err(|_| format!("Failed to bind to {}", addr))?;

    for stream in listener.incoming() {
        match stream {
            Ok(mut s) => {
                println!("Connection accepted");

                let mut buf = [0; 128];
                let mut read_bytes = 0;
                while read_bytes == 0 {
                    read_bytes = s.read(&mut buf).map_err(|_| "failed to read from socket")?;
                    println!("received bytes {}", read_bytes);
                }
                stdout().write(&buf[0..read_bytes]).map_err(|_| "failed to write to stdout")?;
                stdout().flush().unwrap();
            }
            Err(e) => {
                println!("Error while accepting incoming connection - {}", e);
            }
        }
    }

    Ok(())
}