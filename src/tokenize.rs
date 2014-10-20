use std::str::CharSplits;

pub fn tokenize<'a>(text: &'a str) -> Vec<&'a str> {
    // let vec_with_empty: Vec<&str> = text.split(|c: char| char_is_token(c)).collect();
    let ret_vec = text.split(|c:char| char_is_token(c))
                      .filter(|s| s.len()>0)
                      .collect();
    ret_vec
}

fn char_is_token(a: char) -> bool {
  match a {
    ' ' => true,
    ',' => true,
    '.' => true,
    '!' => true,
    '?' => true,
    ';' => true,
    '\'' => true,
    '"' => true,
    ':' => true,
    '\t' => true,
    '\n' => true,
    '(' => true,
    ')' => true,
    '-' => true,
    _ => false
  }
}
