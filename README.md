# regindex
Index &amp;str with regex in Rust (experiment)

Inspired by Ruby syntax for regexp matching using indexing.

Examples:

```rust
#[macro_use]
use regindex::ReIdx;

fn main() {
    let haystack = "abbcccdddd";
    let found = &haystack[ri!["cd+"]];  // == "cdddd"

    // or more verbose way without macros:
    let found = &haystack[ReIdx::new("bc+")];  // == "bccc"

    // if nothing found or error, it returns empty slice:
    let found = &haystack[ReIdx::new("(c+")];  // == "" (invalid regexp)
    let found = &haystack[ri!["(c+"]];  // but this will panic!
    let found = &haystack[ReIdx::new("ax+")];  // == "" (no match found)

    // you can also wrap regex:
    let re = Regex::new("ab+");
    let found = &haystack[ReIdx::from_regex(re)];  // == "abb"
}
```
