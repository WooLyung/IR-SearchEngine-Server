use std::collections::{HashMap, HashSet};
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

    // retrieve by ltc.ntc
    pub fn retrieve(&self, query: String) -> Vec<u32> {
        let mut query_terms: HashSet<&str> = query.split(" ")
            .map(|x| x.trim())
            .collect();
        let mut scores: HashMap<u32, f64> = HashMap::new();

        for pair in self.indexer.terms.iter() {
            let term = &pair.0[..];
            let posting_list = &pair.1;

            if query_terms.contains(term) {
                for posting in posting_list.iter() {
                    let doc_id = posting.0;
                    let score = posting.1;

                    if !scores.contains_key(&doc_id) {
                        scores.insert(doc_id, 0.0);
                    }

                    *scores.get_mut(&doc_id).unwrap() += score;
                }
            }
        }

        let mut ranking: Vec<(u32, f64)> = Vec::new();
        for pair in scores.iter() {
            ranking.push((*pair.0, *pair.1));
        }

        ranking.sort_by(|&x, &y| y.1.partial_cmp(&x.1).unwrap());
        ranking.iter().map(|x| x.0).collect::<Vec<_>>()
    }
}