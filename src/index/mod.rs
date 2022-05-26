use std::fs::File;
use std::io::Read;
use regex::Regex;
use libm::log;

pub struct Indexer {
    pair_list: Vec<(String, u32)>,
    terms: Vec<(String, Vec<(u32, f64)>)>
}

impl Indexer {
    pub fn new() -> Self {
        Indexer {
            pair_list: Vec::new(),
            terms: Vec::new()
        }
    }

    // read corpus text files
    pub fn read(&mut self, fname: &str) -> &mut Self {
        let mut file = File::open(fname).expect("file not found.");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Something went wrong reading the file");

        let mut doc_id = 0;
        let lines: Vec<&str> = contents.split("\n").collect();
        for line in lines {
            let mut tokens: Vec<&str> = line.trim().split(" ").collect();
            doc_id = tokens[0].parse().unwrap();
            tokens.remove(0);

            for token in tokens {
                self.pair_list.push((String::from(token), doc_id));
            }
        }

        self
    }

    // index term-docs lists
    pub fn index(&mut self) -> &mut Self {
        for pair in self.pair_list.iter() {
            let term = &pair.0;
            let doc = &pair.1;
            match self.terms.iter_mut().find(|x| x.0 == *term) {
                Some(postings) => {
                    match postings.1.iter_mut().find(|x| (**x).0 == *doc) {
                        Some(posting) => {
                            posting.1 += 1.0;
                        }
                        None => {
                            postings.1.push((*doc, 1.0));
                        }
                    }
                }
                None => {
                    self.terms.push((String::from(term), vec![(*doc, 1.0)]));
                }
            }
        }
        self
    }

    // sort dictionary, posting lists
    pub fn sort(&mut self) -> &mut Self {
        self.terms.sort_by(|x, y| x.0.cmp(&y.0));
        for postings in self.terms.iter_mut() {
            postings.1.sort_by(|x, y| x.0.cmp(&y.0));
        }

        self
    }

    // print dictionary
    pub fn print(&mut self) -> &mut Self {
        for postings in self.terms.iter() {
            println!("{} : ", postings.0);
            for posting in postings.1.iter() {
                print!("{}:{} ", posting.0, posting.1);
            }
            println!();
        }

        self
    }

    // calculate tf-idf
    pub fn tfidf(&mut self) -> &mut Self {
        let N = self.terms.iter()
            .max_by(|x, y| x.1.len().cmp(&y.1.len()))
            .unwrap()
            .1.len();

        for postings in self.terms.iter_mut() {
            let df = postings.1.len();
            let idf = log((N as f64) / (df as f64));

            for posting in postings.1.iter_mut() {
                let tf = posting.1;
                let wtf = 1.0 + log(tf as f64);
                let tfidf = wtf * idf;

                posting.1 = tfidf;
            }
        }

        self
    }

    // save inverted index as binary file
    pub fn save(&mut self) -> &mut Self {
        self
    }
}