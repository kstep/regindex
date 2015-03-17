#![feature(plugin)]
#![plugin(regex_macros)]
extern crate regex;

use std::ops::Index;
use regex::Regex;

pub struct ReIdx<'a>(&'a Regex);

impl<'a> ReIdx<'a> {
    pub fn from_regex(re: &'a Regex) -> ReIdx<'a> {
        ReIdx(re)
    }
}

trait ToReIdx {
    fn to_regex_index<'a>(&'a self) -> ReIdx<'a>;
}

impl ToReIdx for Regex {
    fn to_regex_index<'a>(&'a self) -> ReIdx<'a> {
        ReIdx(self)
    }
}

impl<'i> Index<ReIdx<'i>> for str {
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
    let re = regex!("^ab+");
    let idx = re.to_regex_index();
    assert_eq!("abb", &"abbcccdddd"[idx]);
    assert_eq!("", &"acccdddd"[idx]);
}
