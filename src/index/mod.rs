use std::collections::HashMap;

pub struct Indexer {
    pair_list: Vec<(String, u32)>,
    terms: Vec<(String, Vec<u32>)>
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
        self.pair_list.push((String::from("abc"), 100));
        self.pair_list.push((String::from("def"), 21));
        self.pair_list.push((String::from("ghi"), 0));
        self.pair_list.push((String::from("abc"), 4));
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
            match self.terms.iter_mut().find(|x| x.0 == *term) {
                Some(postings) => {
                    postings.1.push(*doc);
                }
                None => {
                    self.terms.push((String::from(term), vec![*doc]));
                }
            }
        }
        self
    }

    // sort dictionary, posting lists
    pub fn sort(&mut self) -> &mut Self {
        self.terms.sort();
        for postings in self.terms.iter_mut() {
            postings.1.sort()
        }

        self
    }

    // print dictionary
    pub fn print(&mut self) -> &mut Self {
        for postings in self.terms.iter() {
            println!("{} : ", postings.0);
            for posting in postings.1.iter() {
                print!("{} ", posting);
            }
            println!();
        }

        self
    }

    // save inverted index as binary file
    pub fn save(&mut self) -> &mut Self {
        self
    }
}