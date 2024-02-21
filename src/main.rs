use redis_starter_rust::{Request, Response};
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        thread::spawn(|| match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        });
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buf = [0; 512];

    loop {
        let bytes_read = stream.read(&mut buf).expect("Failed to read from client.");

        if bytes_read == 0 {
            break;
        }
        let request = Request::from(&buf);
        let command = request.command();
        let response = Response::from(&command);
        stream
            .write_all(&response.buf())
            .expect("Failed to write to client");
    }
}
