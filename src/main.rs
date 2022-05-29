mod index;
mod retrieve;
mod document;

use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::process::Command;
use std::str::from_utf8;
use subprocess::Exec;
use index::Indexer;
use crate::document::Document;
use crate::retrieve::Retriever;

fn handle_client(mut stream: TcpStream, indexer: &Indexer, docs: &Document) {
    let mut data = [0 as u8; 50];

    while match stream.read(&mut data) {
        Ok(size) => {
            let mut retriever = Retriever::new(&indexer);

            let output = Command::new("python")
                .args(["./corpus/normalizer.py", from_utf8(&data[0..size]).unwrap()])
                .output()
                .expect("error!!");
            let query = String::from_utf8(output.stdout).unwrap();

            let result = retriever.retrieve(query);
            for num in result {
                let str: &str = &*docs.get_doc(num).unwrap();
                println!("{}", num.to_string() + " : " + str + "\n");
                stream.write((num.to_string() + " : " + str + "\n").as_ref()).unwrap();
            }

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

    let mut docs = Document::new();
    docs.read("corpus/corpus.txt");

    let listener = TcpListener::bind("127.0.0.1:3997")?;
    for stream in listener.incoming() {
        handle_client(stream?, &indexer, &docs);
    }
    Ok(())
}