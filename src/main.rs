use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str::from_utf8;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:25565").unwrap();
    println!("listening started, ready to accept");
    for stream in listener.incoming() {
        thread::spawn(|| {
            handle_client(stream.unwrap());
        });
    }
}

fn handle_client(stream: TcpStream) {
    let mut stream = stream;
    stream.write(b"Hello World\r\n").unwrap();
    let mut buffer = [0; 1024];
    sleep(Duration::from_secs(5));
    let _ = stream.read(&mut buffer);
    match from_utf8(&buffer) {
        Ok(s) => {
            println!("Read: {s}");
        }
        Err(_) => {}
    }
}