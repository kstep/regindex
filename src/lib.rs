#![feature(plugin)]
#![plugin(regex_macros)]
extern crate regex;

use std::ops::Index;
use regex::Regex;
use std::borrow::IntoCow;

#[stable(since="0.1.0")]
pub struct ReIdx(Regex);

impl ReIdx {
    #[unstable]
    pub fn from_regex(re: Regex) -> ReIdx {
        ReIdx(re)
    }

    #[unstable]
    pub fn new<'a, S: IntoCow<'a, str>>(s: S) -> Result<ReIdx, regex::Error> {
        Regex::new(&s.into_cow()).map(|re| ReIdx(re))
    }
}

#[stable(since="0.1.0")]
impl Index<ReIdx> for str {
    type Output = str;
    fn index<'a>(&'a self, index: &ReIdx) -> &'a str {
        match index.0.find(self) {
            Some((start, end)) => &self[start..end],
            None => &self[..0]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ReIdx;

    #[test]
    fn test_index_by_regex() {
        let re = ReIdx::new("^ab+").unwrap();
        assert_eq!("abb", &"abbcccdddd"[re]);
        assert_eq!("", &"acccdddd"[re]);
    }
}
