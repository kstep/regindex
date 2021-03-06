#![feature(plugin)]
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

impl Index<Result<ReIdx, regex::Error>> for str {
    type Output = str;
    fn index<'a>(&'a self, index: &Result<ReIdx, regex::Error>) -> &'a str {
        match *index {
            Ok(ref ri) => &self.index(ri),
            Err(_) => &self[..0]
        }
    }
}

#[macro_export]
macro_rules! ri {
    ($re:expr) => (ReIdx::new($re).unwrap())
}

#[cfg(test)]
mod tests {
    use super::ReIdx;

    #[test]
    fn test_index_by_regex() {
        let re = ReIdx::new("^ab+");
        assert_eq!("abb", &"abbcccdddd"[re]);
        assert_eq!("", &"acccdddd"[re]);
    }

    #[test]
    fn test_invalid_regex() {
        let re = ReIdx::new("^a(b+");
        assert_eq!("", &"abbcccdddd"[re]);
    }

    #[test] fn test_index_with_macro() {
        assert_eq!("cdddd", &"abbcccdddd"[ri!["cd+"]]);
    }
}
