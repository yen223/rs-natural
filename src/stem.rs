extern crate stem;
use std::str::{MaybeOwned, Owned};

pub fn stem<T:Str>(word: T) -> MaybeOwned<'t> {
  Owned(stem::get(word.as_slice()).unwrap())
}
