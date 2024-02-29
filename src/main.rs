use redis_starter_rust::{Config, MetaData, Request, Response, Storage};
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

    if let Some((base, port)) = config.master() {
        connect_to_master(format!("{}:{}", base, port), config.address().port());
    }
    for stream in listener.incoming() {
        let db = Arc::new(Storage::new());
        if let Some(master) = config.master() {
            db.meta.set(MetaData::Master((master.0, master.1)));
        }
        // TODO: generate replication ID
        db.meta.set(MetaData::ReplicationId(
            "8371b4fb1155b71f4a04d3e1bc3e18c4a990aeeb".to_string(),
        ));
        db.meta.set(MetaData::ReplicationOffset(0));
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

fn connect_to_master(address: String, my_port: u16) {
    match TcpStream::connect(address) {
        Ok(mut stream) => {
            let ping = "*1\r\n$4\r\nPING\r\n".as_bytes();
            stream.write_all(ping).unwrap();
            let repl_conf = format!(
                "*3\r\n$8\r\nREPLCONF\r\n$14\r\nlistening-port\r\n$4\r\n{}\r\n",
                my_port
            );
            stream = ad_hoc_response(stream, "+PONG\r\n", repl_conf.as_str());
            stream = ad_hoc_response(
                stream,
                "+OK\r\n",
                "*3\r\n$8\r\nREPLCONF\r\n$4\r\ncapa\r\n$6\r\npsync2\r\n",
            );
            ad_hoc_response(
                stream,
                "+OK\r\n",
                "*3\r\n$5\r\nPSYNC\r\n$1\r\n?\r\n$2\r\n-1\r\n",
            );
        }
        Err(error) => {
            eprintln!("Failure on connecting to master: {}", error);
        }
    }
}

fn ad_hoc_response(mut stream: TcpStream, respond_to: &str, msg: &str) -> TcpStream {
    let mut buf = [0; 512];

    stream.read(&mut buf).expect("Client failure");

    let mut v: Vec<u8> = vec![];
    let zero: u8 = 0;
    for b in buf.into_iter() {
        if b != zero {
            v.push(b);
        }
    }
    let s = String::from_utf8(v).unwrap();
    if s.as_str() == respond_to {
        stream.write_all(msg.as_bytes()).unwrap();
    }
    stream
}
