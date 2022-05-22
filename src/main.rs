mod index;
mod retrieve;

use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use index::Indexer;

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 50];
    while match stream.read(&mut data) {
        Ok(size) => {
            println!("{}", std::str::from_utf8(&data[0..size]).unwrap());
            stream.write(&data[0..size]).unwrap();
            size != 0
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() -> std::io::Result<()> {
    let indexer = Indexer::new();
    indexer.read("hello")
        .index()
        .save();

    let listener = TcpListener::bind("127.0.0.1:3997")?;
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}