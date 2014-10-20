extern crate natural;
extern crate test;

use test::Bencher;
use natural::classifier::NaiveBayesClassifier;
use std::io::File;

#[bench]
fn bench_basic_usage(b: &mut Bencher) {
  b.iter(||{
        let mut nbc = NaiveBayesClassifier::new();
        nbc.train("Debug derive traits into impls that I use with my rust code".as_slice(), 
                "rust".as_slice());
        nbc.train("deriving show for your impl can definitely help you debug your rust code".as_slice(), 
                "rust".as_slice());
        nbc.train("Use more metaprogramming when writing ruby".as_slice(), "ruby".as_slice());
        nbc.train("Classes can have either instance variables or class variables".as_slice(), "ruby".as_slice());
  });
}

#[bench]
fn bench_medium_dataset(b: &mut Bencher) {
    let test_set = [("tests/test_data/medium1.txt", "positive"),
                    ("tests/test_data/medium2.txt", "positive"),
                    ("tests/test_data/medium3.txt", "positive"),
                    ("tests/test_data/medium4.txt", "negative"),
                    ("tests/test_data/medium5.txt", "negative"),
                    ("tests/test_data/medium6.txt", "negative"),
                    ("tests/test_data/medium7.txt", "negative"),];
    let text_set:Vec<(String, &str)> = test_set.iter().map(
        |&(path, class)|{(File::open(&Path::new(path)).read_to_string().unwrap(), class)}
        ).collect();
    let guess_text = File::open(&Path::new("tests/test_data/medium8.txt"))
                        .read_to_string()
                        .unwrap();
    b.iter(||{ 
        let mut nbc = NaiveBayesClassifier::new();
        for &(ref text, class) in text_set.iter(){
            nbc.train(text.as_slice(), class);
        }
        nbc.guess(guess_text.as_slice());
   });
}

