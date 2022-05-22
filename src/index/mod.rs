mod comp;

pub struct Indexer {
    dictionary: Dictionary,
}

pub struct Dictionary {
    root: Option<Box<Indexer>>
}

impl Indexer {
    pub fn new() -> Self {
        Indexer { dictionary: Dictionary { root: None } }
    }

    pub fn read(&self, fname: &str) -> &Self {
        println!("{}", &fname);
        self
    }

    pub fn index(&self) -> &Self {
        self
    }

    pub fn save(&self) -> &Self {
        self
    }
}

impl Dictionary {

}