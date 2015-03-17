#![feature(plugin)]
#![plugin(regex_macros)]
extern crate regex;

use std::ops::Index;
use std::str::FromStr;
use regex::Regex;

pub struct ReIdx(Regex);

impl ReIdx {
    pub fn new(s: &str) -> ReIdx {
        ReIdx(Regex::new(s).unwrap())
    }

    pub fn from_regex(re: Regex) -> ReIdx {
        ReIdx(re)
    }
}

impl FromStr for ReIdx {
    type Err = regex::Error;
    fn from_str(s: &str) -> Result<ReIdx, regex::Error> {
        Regex::new(s).map(ReIdx)
    }
}

trait ToReIdx {
    fn to_regex_index(self) -> ReIdx;
}

impl ToReIdx for Regex {
    fn to_regex_index(self) -> ReIdx {
        ReIdx(self)
    }
}

impl Index<ReIdx> for str {
    type Output = str;
    fn index<'a>(&'a self, index: &ReIdx) -> &'a str {
        if let Some((start, end)) = index.0.find(self) {
            &self[start..end]
        } else {
            &self[..0]
        }
    }
}

/*
impl<T: ToReIdx> Index<T> for str {
    type Output = str;
    #[inline(always)]
    fn index<'a>(&'a self, index: &T) -> &'a str {
        self[index.to_regex_index()]
    }
}
*/

#[test]
fn test_index_by_regex() {
    let re = regex!("^ab+").to_regex_index();
    assert_eq!("abb", &"abbcccdddd"[re]);
    assert_eq!("", &"acccdddd"[re]);
    assert_eq!("abb", &"abbcccdddd"[ReIdx::new("^ab+")]);
}
