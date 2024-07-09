use std::io::Write;
use std::net::TcpStream;

pub fn run(host: &String, port: &u16) -> Result<(), String> {
    let addr = format!("{}:{}", host, port);

    loop {
        let mut client = TcpStream::connect(addr.as_str()).map_err(|_| format!("failed to connect to {}", addr))?;
        client.write("blah".as_bytes()).map_err(|_| format!("failed to send"))?;
    }
    Ok(())
}

fn main() {
    println!("Hello, world!");
    let host = "192.168.1.160".to_string();
    let port:u16 = 8069; 
    run(&host, &port);
}
