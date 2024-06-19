use std::io::{BufRead, BufReader};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:25565").unwrap();
    println!("listening started, ready to accept");
    for stream_result in listener.incoming() {
        match stream_result {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream);
                });
            },
            Err(_) => {
                println!("Failed to accept connection");
            }
        }
    }
}

fn handle_client(stream: TcpStream) {
    let mut buffered_reader = BufReader::new(stream);
    loop {
        let mut string_buffer = &mut Default::default();
        match buffered_reader.read_line(string_buffer) {
            Ok(bytes_read) => {
                println!("Read {bytes_read} bytes");
                println!("Value: {string_buffer}");
            }
            Err(_) => {
                panic!("BAD");
            }
        }
    }
}
