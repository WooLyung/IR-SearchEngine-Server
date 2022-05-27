use crate::Indexer;

pub struct Retriever<'a> {
    indexer: &'a Indexer
}

impl<'a> Retriever<'a> {
    pub fn new(indexer: &'a Indexer) -> Self {
        Retriever {
            indexer
        }
    }

    pub fn retrieve(&self, query: String) -> Vec<u32> {
        vec![]
    }
}