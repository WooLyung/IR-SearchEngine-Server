mod index;
mod retrieve;

use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::process::Command;
use std::str::from_utf8;
use subprocess::Exec;
use index::Indexer;
use crate::retrieve::Retriever;

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 50];
    while match stream.read(&mut data) {
        Ok(size) => {
            println!("{}", from_utf8(&data[0..size]).unwrap());
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
    let mut indexer = Indexer::new();

    indexer.read("corpus/nor_corpus.txt")
        .index()
        .sort()
        .tfidf()
        .normalize();

    let mut retriever = Retriever::new(&indexer);

    let output = Command::new("python")
        .args(["./corpus/normalizer.py", "자유 소프트웨어 운동과 대통령"])
        .output()
        .expect("error!!");
    let query = String::from_utf8(output.stdout).unwrap();
    println!("{}", &query);

    let result = retriever.retrieve(query);
    for num in result {
        println!("{}", num);
    }

    // let listener = TcpListener::bind("127.0.0.1:3997")?;
    // for stream in listener.incoming() {
    //     handle_client(stream?);
    // }
    Ok(())
}