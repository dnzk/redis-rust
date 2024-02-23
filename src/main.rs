use redis_starter_rust::{KvStore, Request, Response};
use std::sync::Arc;
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        let db = Arc::new(KvStore::new());
        thread::spawn(move || match stream {
            Ok(stream) => {
                handle_client(stream, Arc::clone(&db));
            }
            Err(e) => {
                println!("error: {}", e);
            }
        });
    }
}

fn handle_client(mut stream: TcpStream, db: Arc<KvStore>) {
    let mut buf = [0; 512];

    loop {
        let bytes_read = stream.read(&mut buf).expect("Failed to read from client.");

        if bytes_read == 0 {
            break;
        }
        let request = Request::from(&buf, &db);
        let command = request.command();
        let response = Response::from(&command);
        stream
            .write_all(&response.buf())
            .expect("Failed to write to client");
    }
}
