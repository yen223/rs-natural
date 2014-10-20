extern crate natural;

use natural::classifier::NaiveBayesClassifier;
use std::io::File;

#[test]
fn test_basic_usage() {
  let mut nbc = NaiveBayesClassifier::new();
  nbc.train("Debug derive traits into impls that I use with my rust code", 
            "rust");
  nbc.train("deriving show for your impl can definitely help you debug your rust code", 
            "rust");
  nbc.train("Use more metaprogramming when writing ruby", "ruby");
  nbc.train("Classes can have either instance variables or class variables", "ruby");
  assert_eq!(nbc.guess("debug this rust code"), "rust");
  assert_eq!(nbc.guess("This class is about ruby"), "ruby");
}

#[test]
fn test_medium_dataset() {
    let test_set = [("tests/test_data/medium1.txt", "positive"),
                    ("tests/test_data/medium2.txt", "positive"),
                    ("tests/test_data/medium3.txt", "positive"),
                    ("tests/test_data/medium4.txt", "negative"),
                    ("tests/test_data/medium5.txt", "negative"),
                    ("tests/test_data/medium6.txt", "negative"),
                    ("tests/test_data/medium7.txt", "negative"),];
    
  let mut nbc = NaiveBayesClassifier::new();
  for &(path, class) in test_set.iter(){
    let text = File::open(&Path::new(path)).read_to_string().unwrap();
    nbc.train(text, class);
  }
  let guess_text = File::open(&Path::new("tests/test_data/medium8.txt"))
                        .read_to_string()
                        .unwrap();
  let guess = nbc.guess(guess_text);

  assert_eq!(guess, "positive".as_slice());

}

#[test]
fn no_fail_on_empty_strings() {
  let mut nbc = NaiveBayesClassifier::new();
  nbc.train(String::from_str(""), String::from_str(""));
  assert!(true);
}
