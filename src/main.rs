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
            let mut send_msg: String = String::new();
            let mut i = 0;

            for num in result {
                let str: &str = &*docs.get_doc(num).unwrap();
                send_msg += &*(num.to_string() + " : " + str + "\n");

                i += 1;
                if i == 5 {
                    break;
                }
            }
            stream.write(send_msg.as_ref());

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