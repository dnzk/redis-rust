use redis_starter_rust::{Config, KvStore, MetaData, Request, Response, Storage};
use std::env;
use std::sync::Arc;
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::from(&args);
    let address = config.address();
    let listener = TcpListener::bind(address).unwrap();

    for stream in listener.incoming() {
        let db = Arc::new(Storage::new());
        if let Some(master) = config.master() {
            db.meta.set(MetaData::Master((master.0, master.1)));
        }
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

fn handle_client(mut stream: TcpStream, db: Arc<Storage>) {
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
