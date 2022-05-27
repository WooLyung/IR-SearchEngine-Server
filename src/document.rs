use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use regex::Regex;
use crate::Indexer;

pub struct Document {
    postings: HashMap<u32, String>
}

impl Document{
    pub fn new() -> Self {
        Document {
            postings: HashMap::new()
        }
    }

    pub fn get_doc(&self, doc_id: u32) -> Option<String> {
        if self.postings.contains_key(&doc_id) {
            let str = self.postings.get(&doc_id).unwrap();
            return Some(str.to_string());
        }
        else {
            return None;
        }
    }

    // read corpus text files
    pub fn read(&mut self, fname: &str) {
        let mut file = File::open(fname).expect("file not found.");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Something went wrong reading the file");

        let mut doc_id: u32 = 0;
        let regex = Regex::new(r"<title>.*</title>").unwrap();

        let lines: Vec<&str> = contents.split("\n").collect();
        for line in lines {
            if regex.is_match(line.trim()) {
                let str = line.replace("<title>", "")
                    .replace("</title>", "");
                let parts: Vec<&str> = str.split(". ").collect();

                doc_id = parts[0].parse().unwrap();
                if !self.postings.contains_key(&doc_id) {
                    self.postings.insert(doc_id, String::from(""));
                }
            }
            else {
                (*self.postings.get_mut(&doc_id).unwrap()) += line.trim();
            }
        }
    }
}