extern crate stem;

use tokenize::tokenize;
use stem::get;
use std::collections::{HashMap, PriorityQueue};
use std::collections::hashmap::{Occupied, Vacant};
use std::str::{MaybeOwned};


pub struct NaiveBayesClassifier <'a>{
  documents: HashMap<String, HashMap<String, uint>>,
  total_document_count: uint
}

impl<'a> NaiveBayesClassifier<'a> {
  pub fn new() -> NaiveBayesClassifier<'a> {
    NaiveBayesClassifier{ documents: HashMap::new(), total_document_count: 0 }
  }
  
  pub fn train(&mut self, text: &str, classification: &'a str) {
    let classification_map = match self.documents.entry(classification.to_string()) {
        Vacant(entry) => entry.set(HashMap::new()),
        Occupied(entry) => entry.into_mut()
    };
    let stemmed_and_tokenized = get_tokenized_and_stemmed(text);
    for stemmed_word in stemmed_and_tokenized.into_iter() {
        match classification_map.entry(stemmed_word.into_string()) {
            Vacant(entry) => { entry.set(1); }, // Arm must return ()
            Occupied(mut entry) => *entry.get_mut() += 1
        }
    }
    self.total_document_count += 1;
  }
  
  pub fn guess(&'a self, text: &str) -> &'a str {
    let stemmed_and_tokenized = get_tokenized_and_stemmed(text);
    let mut best_guess = "";
    let mut best_prob = 0.0;
    for (k,v) in self.documents.iter() {
        //Get the probability that the passed-in text is each class
        let mut probability: f32 = 0.0;
        for stemmed_word in stemmed_and_tokenized.iter() {
            if (*v).contains_key(&stemmed_word.to_string()) {
                probability += (1.0 / v.len() as f32).ln();
            }
        }
        let prob = v.len() as f32 * probability.abs() / self.total_document_count as f32;
        if prob > best_prob {
            best_guess = k.as_slice();
            best_prob = prob
        }
    }
    best_guess
  }
}

fn get_tokenized_and_stemmed<'t>(text: &str) -> Vec<MaybeOwned<'t>> 
{
  let tokenized_text = tokenize(text.as_slice())
                                    .iter()
                                    .map(|&w|{stem::get(w).unwrap_or("".to_string())
                                                          .into_maybe_owned()})
                                    .collect();
  tokenized_text
}

