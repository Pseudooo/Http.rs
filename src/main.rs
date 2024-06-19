use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str::from_utf8;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:25565").unwrap();
    println!("listening started, ready to accept");
    for stream_result in listener.incoming() {
        match stream_result {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream);
                });;
            },
            Err(_) => {
                println!("Failed to accept connection");
            }
        }
    }
}

fn handle_client(stream: TcpStream) {

    // Write to client
    let mut stream = stream;
    stream.write(b"Hello World\r\n").unwrap();

    // Give client 5s to write a response to server
    sleep(Duration::from_secs(5));

    // Read UTF8 string from socket
    let mut buffer = [0; 1024];
    let _ = stream.read(&mut buffer);
    match from_utf8(&buffer) {
        Ok(s) => {
            println!("Read: {s}");
        }
        Err(_) => {}
    }
}
