use std::{io::Write, net::TcpListener};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");
                let buf = b"+PONG\r\n";
                stream.write_all(buf).expect("Failed to write to client.");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
