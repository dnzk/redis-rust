use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut message = String::new();
                stream
                    .read_to_string(&mut message)
                    .expect("Failed to read from client.");
                pong(&mut stream, ping_count(&message));
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn ping_count(message: &str) -> usize {
    let by_newline: Vec<&str> = message.trim().split('\n').collect();
    by_newline.len()
}

fn pong(stream: &mut TcpStream, count: usize) {
    let buf = b"+PONG\r\n";
    for _i in 0..count {
        stream.write_all(buf).expect("Failed to write to client.");
    }
}
