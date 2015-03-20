#![feature(plugin)]
#![plugin(regex_macros)]
extern crate regex;

use std::ops::Index;
use regex::Regex;

pub struct ReIdx(Regex);

impl ReIdx {
    pub fn new(re: Regex) -> ReIdx {
        ReIdx(re)
    }
}

impl Index<ReIdx> for str {
    type Output = str;
    fn index<'a>(&'a self, index: &ReIdx) -> &'a str {
        match index.0.find(self) {
            Some((start, end)) => &self[start..end],
            None => &self[..0]
        }
    }
}

#[test]
fn test_index_by_regex() {
    let re = ReIdx::new(regex!("^ab+"));
    assert_eq!("abb", &"abbcccdddd"[re]);
    assert_eq!("", &"acccdddd"[re]);
}
