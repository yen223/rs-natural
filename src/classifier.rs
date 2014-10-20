extern crate stem;

use tokenize::tokenize;
use stem::get;
use std::collections::{HashMap, PriorityQueue};
use std::collections::hashmap::{Occupied, Vacant};
use std::str::{MaybeOwned};


#[deriving(PartialEq, Show)]
struct WordProb<'tmp>(&'tmp str, f32);
impl<'tmp> PartialOrd for WordProb<'tmp> {
    fn partial_cmp(&self, other: &WordProb) -> Option<Ordering>{
        Some(self.cmp(other))
    }
}

impl<'tmp> Ord for WordProb<'tmp> {
    fn cmp(&self, other:&WordProb) -> Ordering {
        let WordProb(_, prob_a) = *self;
        let WordProb(_, prob_b) = *other;
        prob_a.partial_cmp(&prob_b).unwrap_or(Equal)
    }
}

impl<'tmp> Eq for WordProb<'tmp> {}

pub struct NaiveBayesClassifier <'a>{
  documents: HashMap<String, HashMap<String, uint>>,
  total_document_count: uint
}

impl<'a> NaiveBayesClassifier<'a> {
  pub fn new() -> NaiveBayesClassifier<'a> {
    NaiveBayesClassifier{ documents: HashMap::new(), total_document_count: 0 }
  }
  
  pub fn train(&mut self, text: &str, classification: &str) {
    let classification_map = match self.documents.entry(classification.to_string()) {
      Vacant(entry) => entry.set(HashMap::new()),
      Occupied(entry) => entry.into_mut()
    };
    let stemmed_and_tokenized = get_tokenized_and_stemmed(text);
    for stemmed_word in stemmed_and_tokenized.into_iter() {
      match classification_map.entry(stemmed_word.to_string()) {
        Vacant(entry) => { entry.set(1); }, // Arm must return ()
        Occupied(mut entry) => *entry.get_mut() += 1
      }
    }
    self.total_document_count += 1;
  }
  
  pub fn guess(&'a self, text: &str) -> &'a str {
    let stemmed_and_tokenized = get_tokenized_and_stemmed(text);
    
    let mut label_probabilities = PriorityQueue::new();
    for (k,v) in self.documents.iter() {
      //Get the probability that the passed-in text is each class
      let mut probability: f32 = 0.0;
      for stemmed_word in stemmed_and_tokenized.iter() {
        if (*v).contains_key(&stemmed_word.to_string()) {
          probability += (1.0 / v.len() as f32).ln();
        }
      }
    let prob = v.len() as f32 * probability.abs() / self.total_document_count as f32;
    label_probabilities.push(WordProb(k.as_slice(), prob));
      // }
    }
    let answer = match label_probabilities.pop(){
        Some(WordProb(k, _)) => k,
        None => "",
    };
    answer
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

#[test]
fn test_word_prob(){
    let mut pq = PriorityQueue::new();
    let a = WordProb("test1", 0.0);
    let b = WordProb("test2", 5.7);
    let c = WordProb("test3", 3.2);
    assert!(a < b);
    pq.push(a);
    pq.push(b);
    pq.push(c);
    assert_eq!(pq.pop(), Some(b));
    assert_eq!(pq.pop(), Some(c));
    assert_eq!(pq.pop(), Some(a));
}
