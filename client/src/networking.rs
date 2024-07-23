use std::io::Write;
use std::net::TcpStream;
use std::time::Duration;
use std::thread;

pub fn connect_to_server(host: &String, port: &u16) -> Result<TcpStream, String> {
    let addr = format!("{}:{}", host, port);

    loop {
        match TcpStream::connect(&addr) {
            Ok(stream) => return Ok(stream),
            Err(_) => {
                println!("Failed to connect to {}. Retrying in 5 seconds...", addr);
                thread::sleep(Duration::from_secs(5));
            }
        }
    }
}

pub fn send_data(stream: &mut TcpStream, message: &String) -> Result<(), String> {
    stream.write(message.as_bytes()).map_err(|_| format!("failed to send"))?;
    Ok(())
}


// use std::io::Write;
// use std::net::TcpStream;

// pub fn run(host: &String, port: &u16, message: &String) -> Result<(), String> {
//     let addr = format!("{}:{}", host, port);

//     let mut client = TcpStream::connect(addr.as_str()).map_err(|_| format!("failed to connect to {}", addr))?;
//     client.write(message.as_bytes()).map_err(|_| format!("failed to send"))?;
    
//     Ok(())
// }

// pub fn connect(host: &String, port: &u16) -> Result<TcpStream, String> {
//     let addr = format!("{}:{}", host, port);
//     TcpStream::connect(addr.as_str()).map_err(|_| format!("failed to connect to {}", addr))
// }

// pub fn write_message(mut client: TcpStream, message: &String) -> Result<(), String> {
//     client.write(message.as_bytes()).map_err(|_| format!("failed to send"))?;
//     Ok(())
// }

// pub fn run(host: &String, port: &u16, message: &String) -> Result<(), String> {
//     let client = connect(host, port)?;
//     write_message(client, message)
// }