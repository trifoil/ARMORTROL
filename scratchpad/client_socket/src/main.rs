use std::io::Write;
use std::net::TcpStream;

pub fn run(host: &String, port: &u16) -> Result<(), String> {
    let addr = format!("{}:{}", host, port);
    //let mut client = TcpStream::connect(addr.as_str()).map_err(|_| format!("failed to connect to {}", addr))?;

    //client.write("hello, TCP".as_bytes()).map_err(|_| format!("failed to send"))?;
    
    let mut i = 0;
    let mut client = TcpStream::connect(addr.as_str()).map_err(|_| format!("failed to connect to {}", addr))?;
    client.write("hello, TCP\n".as_bytes()).map_err(|_| format!("failed to send"))?;

    while i < 3 {
        let mut client = TcpStream::connect(addr.as_str()).map_err(|_| format!("failed to connect to {}", addr))?;
        client.write("hello, TCP\n".as_bytes()).map_err(|_| format!("failed to send"))?;
        i+=1;
    }
    Ok(())
}

fn main() {
    println!("Hello, world!");
    let mut host= String::new();
    host = "192.168.1.160".to_string();
    let port:u16 = 8069; 
    run(&host, &port);
}
