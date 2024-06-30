use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn main() {
    let port: u16 = 9999;

    // startup
    let listener = TcpListener::bind(format!("{}{}", "127.0.0.1:", port)).unwrap();
    println!("Listening for connection on port {}", port);

    if let Ok((stream, _)) = listener.accept() {
        println!("Connection established!");
        connection_handler(stream);
    }

    println!("Connection closed.");

    // camera

    // set up listener for connection

    // loop where camera is sent back via udp

    // listen for packet coming in where 
}

fn connection_handler( mut stream: TcpStream )  {
    let mut buffer = [0; 512];

    loop {
        let bytes_read = stream.read(&mut buffer).unwrap() ;
        if bytes_read == 0 {
            break;
        }

        println!("Received: {}", String::from_utf8_lossy(&buffer[..bytes_read]));

        stream.write_all(b"Message received");
        stream.flush();
    }
}
