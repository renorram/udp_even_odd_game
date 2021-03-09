use crate::server::DEFAULT_SERVER_ADDRESS;
use std::net::{UdpSocket, SocketAddr};
use std::process::exit;
use std::str::FromStr;

pub fn run_client(server_address: Option<String>) -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:0")?;
    let server_address: SocketAddr = match server_address {
        Some(address) => SocketAddr::from_str(address.as_str()).unwrap(),
        None => SocketAddr::from_str(DEFAULT_SERVER_ADDRESS).unwrap()
    };

    socket.connect(server_address)?;

    let mut input_buffer = String::new();
    let mut buffer = [0; 1024];

    loop {
        println!("Type the input:");
        if let Err(err) = std::io::stdin().read_line(&mut input_buffer) {
            println!("The following error happened while reading from input: {}", err);
            exit(1);
        }

        socket.send(input_buffer.as_bytes())?;
        input_buffer.clear();


        let bytes_received = socket.recv(&mut buffer)?;
        match String::from_utf8(buffer[..bytes_received].to_vec()) {
            Ok(value) => {
                println!("Server response: {}", value);
            }
            Err(e) => println!("Error parsing: {}", e)
        }
    }
}