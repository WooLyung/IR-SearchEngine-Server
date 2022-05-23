use std::collections::HashMap;
use crate::index::comp::ListNode;

mod comp;

pub struct Indexer {
    dictionary: Dictionary,
    pair_list: Vec<(String, u32)>
}

pub struct Dictionary {
    head: Option<Box<ListNode>>
}

impl Indexer {
    pub fn new() -> Self {
        Indexer {
            dictionary: Dictionary { head: None },
            pair_list: Vec::new()
        }
    }

    // read corpus text files
    pub fn read(&mut self, fname: &str) -> &mut Self {
        self.pair_list.push((String::from("abc"), 0));
        self.pair_list.push((String::from("def"), 1));
        self.pair_list.push((String::from("ghi"), 2));
        self.pair_list.push((String::from("abc"), 1));
        self.pair_list.push((String::from("abc"), 2));
        self.pair_list.push((String::from("ghi"), 3));
        self
    }

    // lemmatization
    pub fn lemmatize(&mut self) -> &mut Self {
        self.pair_list = self.pair_list.iter().map(|x| {
            (String::from(&x.0), &x.1 + 5)
        }).collect();
        self
    }

    // index term-docs lists
    pub fn index(&mut self) -> &mut Self {
        for pair in self.pair_list.iter() {
            let term = &pair.0;
            let doc = &pair.1;
            self.dictionary.insert(String::from(term), *doc);
        }
        self
    }

    // save inverted index as binary file
    pub fn save(&mut self) -> &mut Self {
        self
    }
}

impl Dictionary {
    pub fn insert(&mut self, term: String, doc: u32) {
        match self.head {
            Some(_) => {
            }
            None => {
                self.head = Some(Box::new(ListNode {
                    link: None,
                    term: term,
                    frequency: 0,
                    ptr: ()
                }));
            }
        }
    }
}