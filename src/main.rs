use std::net::TcpListener;
use std::net::TcpStream;
use std::io::Read;
use std::io::Write;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    match stream.read(&mut buffer) {
        Ok(size) => {
            let request = String::from_utf8_lossy(&buffer[..size]);
            println!("Request: {}", request);

            stream.write(b"Message received").expect("Failed to send message.");

        }
        Err(e) => {
            eprintln!("Connection failed: {}", e);
        }
    }

}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:5000")
        .expect("Failed to bind to address.");
    println!("Server is listening on 127.0.0.1:5000");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                eprintln!("Failed to establish connection");
            }
        }
    }

}