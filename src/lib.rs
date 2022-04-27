//! **A utility macro for flexible slug genereation that handles unicode.**
//!
//! The `slugify!` macro implements a flexible slug generator, allowing for stop words, custom separator, randomness
//! and maximum length options. The macro provides both a simple interface with sane default parameters
//! but also allows the parameters to be overriden when needed.
//!
//! Features:
//!
//!* Unicode strings support (phonetic conversion).
//!* Support for custom slug separator.
//!* Stop words filtering.
//!* Slug maximum length support.
//!
//!
//!# Usage
//!
//! This crate is on crates.io and can be used by adding `slugify-rs` to the dependencies in your project's
//! `Cargo.toml`
//!
//! ```toml
//! [dependencies]
//! slugify-rs = "0.0.1"
//! ```
//!
//!  and this to your crate root:
//!
//!```rust,ignore
//! #[macro_use] extern crate slugify-rs;
//!use slugify-rs::slugify;
//!```
//!
//!# Examples
//!
//!## Basic slug generation
//!
//!```rust
//! # #[macro_use] extern crate slugify-rs;
//! # use slugify-rs::slugify;
//! # fn main() {
//!assert_eq!(slugify!("hello world"), "hello-world");
//! # }
//!```
//!
//!## Using a custom separator
//!
//! ```rust
//! # #[macro_use] extern crate slugify;
//! # use slugify::slugify;
//! # fn main() {
//!assert_eq!(slugify!("hello world", separator = "."), "hello.world");
//!assert_eq!(slugify!("hello world", separator = " "), "hello world");
//! assert_eq!(slugify!("hello world", separator = ""), "helloworld");
//! # }
//! ```
//!
//!## Stop words filtering
//!
//!```rust
//! # #[macro_use] extern crate slugify;
//! # use slugify::slugify;
//! # fn main() {
//!assert_eq!(slugify!("the quick brown fox jumps over the lazy dog", stop_words = "the,fox"), "quick-brown-jumps-over-lazy-dog");
//! # }
//!```
//!
//!## Maximum length
//!
//!```rust
//! # #[macro_use] extern crate slugify;
//! # use slugify::slugify;
//! # fn main() {
//!assert_eq!(slugify!("hello world", max_length = 5), "hello");
//!assert_eq!(slugify!("the hello world", stop_words = "the", max_length = 5), "hello");
//! # }
//!```
//!
//!## Random values added to string through nanoid
//! ```rust
//! # #[macro_use] extern crate slugify;
//! # use slugify::slugify;
//! # fn main() {
//! assert_eq!(slugify!("hello world", randomness=true).len(), "hello-world".len()+6);
//! # }
//! ```
//! ```rust
//! # #[macro_use] extern crate slugify;
//! # use slugify::slugify;
//! # fn main() {
//! assert_eq!(slugify!("hello world", randomness=true,randomness_length=8).len(), "hello-world".len()+9);
//! # }
//! ```
//!## Phonetic Conversion and accented text
//!
//!```rust
//! # #[macro_use] extern crate slugify;
//! # use slugify::slugify;
//! # fn main() {
//!assert_eq!(slugify!("影師嗎"), "ying-shi-ma");
//!assert_eq!(slugify!("Æúű--cool?"), "aeuu-cool");
//!assert_eq!(slugify!("Nín hǎo. Wǒ shì zhōng guó rén"), "nin-hao-wo-shi-zhong-guo-ren");
//! # }
//!```
//!
//!## Passing multiple optional parameters.
//!
//! **NOTE:** the order of optional parameters matters: **stop_words**, **separator**
//! and then **max_length**. All of them are optional, however when specifying more than one optional parameter, this
//! order must be adhered.
//!
//!```rust
//! # #[macro_use] extern crate slugify;
//! # use slugify::slugify;
//! # fn main() {
//!assert_eq!(slugify!("the hello world", stop_words = "the", separator = "-"), "hello-world");
//!assert_eq!(slugify!("the hello world", separator = ".", max_length = 10), "the.hello");
//!assert_eq!(slugify!("the hello world", stop_words = "the", max_length = 5), "hello");
//!assert_eq!(slugify!("the hello world", stop_words = "the", separator = "-", max_length = 20), "hello-world");
//! # }
//!```
//!
use deunicode::deunicode;

#[macro_export]
macro_rules! slugify {
    ($text:expr) => {{
        slugify($text, "", "-", None, false, 5)
    }};
    // with a boolean
    ($text:expr, randomness=$bool:expr) => {{
        slugify($text, "", "-", None, $bool, 5)
    }};

    ($text:expr, randomness=$bool:expr,randomness_length=$usize:expr) => {{
        slugify($text, "", "-", None, $bool, $usize)
    }};

    ($text:expr, stop_words=$stopwords:expr) => {{
        slugify($text, $stopwords, "-", None, false, 5)
    }};

    ($text:expr, stop_words=$stopwords:expr,randomness=$bool:expr) => {{
        slugify($text, $stopwords, "-", None, $bool, 5)
    }};

    ($text:expr, stop_words=$stopwords:expr,randomness=$bool:expr,randomness_length=$usize:expr) => {{
        slugify($text, $stopwords, "-", None, $bool, $usize)
    }};

    ($text:expr, separator=$sep:expr) => {{
        slugify($text, "", $sep, None, false, 5)
    }};

    ($text:expr, separator=$sep:expr,randomness=$bool:expr) => {{
        slugify($text, "", $sep, None, $bool, 5)
    }};

    ($text:expr, separator=$sep:expr,randomness=$bool:expr,randomness_length=$usize:expr) => {{
        slugify($text, "", $sep, None, $bool, $usize)
    }};
    ($text:expr, max_length=$len:expr) => {{
        slugify($text, "", "-", Some($len), false, 5)
    }};

    ($text:expr, max_length=$len:expr,randomness=$bool:expr) => {{
        slugify($text, "", "-", Some($len), $bool, 5)
    }};

    ($text:expr, max_length=$len:expr,randomness=$bool:expr,randomness_length=$usize:expr) => {{
        slugify($text, "", "-", Some($len), $bool, $usize)
    }};

    ($text:expr, stop_words=$stopwords:expr, separator=$sep:expr) => {{
        slugify($text, $stopwords, $sep, None, false, 5)
    }};

    ($text:expr, stop_words=$stopwords:expr, separator=$sep:expr,randomness=$bool:expr) => {{
        slugify($text, $stopwords, $sep, None, $bool, 5)
    }};

    ($text:expr, stop_words=$stopwords:expr, separator=$sep:expr,randomness=$bool:expr,randomness_length=$usize:expr) => {{
        slugify($text, $stopwords, $sep, None, $bool, $usize)
    }};

    ($text:expr, stop_words=$stopwords:expr, max_length=$len:expr) => {{
        slugify($text, $stopwords, "-", Some($len), false, 5)
    }};

    ($text:expr, stop_words=$stopwords:expr, max_length=$len:expr,randomness=$bool:expr) => {{
        slugify($text, $stopwords, "-", Some($len), $bool, 5)
    }};

    ($text:expr, stop_words=$stopwords:expr, max_length=$len:expr,randomness=$bool:expr,randomness_length=$usize:expr) => {{
        slugify($text, $stopwords, "-", Some($len), $bool, $usize)
    }};

    ($text:expr, separator=$sep:expr, max_length=$len:expr) => {{
        slugify($text, "", $sep, Some($len), false, 5)
    }};

    ($text:expr, separator=$sep:expr, max_length=$len:expr,randomness=$bool:expr) => {{
        slugify($text, "", $sep, Some($len), $bool, 5)
    }};

    ($text:expr, separator=$sep:expr, max_length=$len:expr,randomness=$bool:expr,randomness_length=$usize:expr) => {{
        slugify($text, "", $sep, Some($len), $bool, $usize)
    }};

    ($text:expr, stop_words=$stopwords:expr, separator=$sep:expr, max_length=$len:expr) => {{
        slugify($text, $stopwords, $sep, Some($len), false, 5)
    }};

    ($text:expr, stop_words=$stopwords:expr, separator=$sep:expr, max_length=$len:expr,randomness=$bool:expr) => {{
        slugify($text, $stopwords, $sep, Some($len), $bool, 5)
    }};

    ($text:expr, stop_words=$stopwords:expr, separator=$sep:expr, max_length=$len:expr,randomness=$bool:expr,randomness_length=$usize:expr) => {{
        slugify($text, $stopwords, $sep, Some($len), $bool, $usize)
    }};
}

pub fn slugify(
    string: &str,
    stop_words: &str,
    sep: &str,
    max_length: Option<usize>,
    randomness: bool,
    randomness_length: usize,
) -> String {
    let char_vec: Vec<char> = sep.chars().collect();
    let mut string: String = deunicode(string.into())
        .to_lowercase()
        .trim()
        .trim_matches(match char_vec.get(0) {
            Some(a) => a.to_owned(),
            None => ' ',
        })
        .replace(' ', &sep.to_string());

    // remove stop words
    for word in stop_words.split(",") {
        if !word.is_empty() {
            string = string.replace(word, &sep.to_string());
        }
    }

    let mut slug = Vec::with_capacity(string.len());

    let mut is_sep = true;

    for x in string.chars() {
        match x {
            'a'..='z' | '0'..='9' => {
                is_sep = false;
                slug.push(x as u8);
            }
            _ => {
                if !is_sep {
                    is_sep = true;
                    slug.push(char_vec[0] as u8);
                } else {
                }
            }
        }
    }

    if char_vec.len() > 0 && slug.last() == Some(&(char_vec[0] as u8)) {
        slug.pop();
    }

    let mut s = String::from_utf8(slug).unwrap();

    match max_length {
        Some(x) => {
            s.truncate(x);
            s = s.trim_end_matches(char_vec[0]).to_string();
        }
        None => {}
    }

    // if randomness is true, generate a nanoid with of size 5 and append it to s
    if randomness {
        let nanoid = nanoid::nanoid!(randomness_length);
        // change letters to lowercase
        let nanoid = nanoid.to_lowercase();
        // append separator to infront of nanoid
        s.push_str(&sep);
        s.push_str(&nanoid);
    }
    println!("{}", s);
    s
}

#[cfg(test)]
mod tests {
    use crate::slugify;
    #[test]
    fn basic() {
        assert_eq!(
            slugify("hello world", "", "-", None, false, 5),
            "hello-world"
        );
        assert_eq!(
            slugify("hello world-", "", "-", None, false, 5),
            "hello-world"
        );
        assert_eq!(
            slugify("hello world ", "", "-", None, false, 5),
            "hello-world"
        );
        assert_eq!(
            slugify("hello world ", "", "-", None, true, 5).len(),
            "hello-world".len() + 6
        );
        assert_eq!(
            slugify("hello world ", "", "", None, false, 5),
            "helloworld"
        );
    }

    #[test]
    fn test_email() {
        assert_eq!(slugify!("alice@bob.com"), "alice-bob-com");
        assert_eq!(slugify!("alice@bob.com"), "alice-bob-com");
    }

    // Test length of randomness
    #[test]
    fn test_randomness() {
        assert_eq!(
            slugify!("hello world", randomness = true).len(),
            "hello-world".len() + 6
        );
    }

    #[test]
    fn test_starts_with_number() {
        assert_eq!(slugify!("10 amazing secrets"), "10-amazing-secrets");
    }

    #[test]
    fn test_contains_numbers() {
        assert_eq!(slugify!("the 101 dalmatians"), "the-101-dalmatians");
        assert_eq!(
            slugify!("the 101 dalmatians", randomness = true).len(),
            "the-101-dalmatians".len() + 6
        );
    }

    #[test]
    fn test_ends_with_number() {
        assert_eq!(slugify!("lucky number 7"), "lucky-number-7");
    }

    #[test]
    fn test_numbers_only() {
        assert_eq!(slugify!("101"), "101");
    }

    #[test]
    fn test_numbers_and_symbols() {
        assert_eq!(
            slugify!("1000 reasons you are #1"),
            "1000-reasons-you-are-1"
        );
    }

    #[test]
    fn test_stop_words() {
        assert_eq!(
            slugify("hello world", "world", "-", None, false, 5),
            "hello",
        );
        assert_eq!(slugify!("hello world", stop_words = "world"), "hello");
        assert_eq!(
            slugify!("hello world", stop_words = "world", randomness = true).len(),
            "hello".len() + 6
        );
    }

    #[test]
    fn test_differently_cased_stopword_match() {
        assert_eq!(
            slugify("Foo A FOO B foo C", "foo", "-", None, false, 5),
            "a-b-c",
        );
    }

    #[test]
    fn test_multiple_stop_words() {
        assert_eq!(
            slugify(
                "the quick brown fox jumps over the lazy dog",
                "the",
                "-",
                None,
                false,
                5
            ),
            "quick-brown-fox-jumps-over-lazy-dog",
        );
        assert_eq!(
            slugify(
                "the quick brown fox jumps over the lazy dog",
                "the,fox",
                "-",
                None,
                false,
                5
            ),
            "quick-brown-jumps-over-lazy-dog",
        );
        assert_eq!(
            slugify!(
                "the quick brown fox jumps over the lazy dog",
                stop_words = "the,fox"
            ),
            "quick-brown-jumps-over-lazy-dog"
        );
    }

    #[test]
    fn test_stopwords_with_different_separator() {
        assert_eq!(
            slugify(
                "the quick brown fox jumps over the lazy dog",
                "the",
                " ",
                None,
                false,
                5
            ),
            "quick brown fox jumps over lazy dog"
        );

        assert_eq!(
            slugify(
                "the quick brown fox jumps over the lazy dog",
                "the",
                " ",
                None,
                true,
                8
            )
            .len(),
            "quick brown fox jumps over lazy dog".len() + 9
        );
        assert_eq!(
            slugify!(
                "the quick brown fox jumps over the lazy dog",
                stop_words = "the",
                separator = " "
            ),
            "quick brown fox jumps over lazy dog"
        );

        assert_eq!(
            slugify!(
                "the quick brown fox jumps over the lazy dog",
                stop_words = "the",
                separator = " ",
                randomness = true
            )
            .len(),
            "quick brown fox jumps over lazy dog".len() + 6
        );

        assert_eq!(
            slugify!(
                "the quick brown fox jumps over the lazy dog",
                stop_words = "the",
                separator = " ",
                randomness = true,
                randomness_length = 8
            )
            .len(),
            "quick brown fox jumps over lazy dog".len() + 9
        );
    }

    #[test]
    fn test_separator() {
        assert_eq!(
            slugify("hello world", "", ".", None, false, 5),
            "hello.world"
        );
        assert_eq!(
            slugify("hello world", "", "_", None, false, 5),
            "hello_world"
        );
        assert_eq!(slugify!("hello world", separator = "_"), "hello_world");
        assert_eq!(
            slugify!("hello world-", separator = "_", randomness = true).len(),
            "hello_world".len() + 6
        );
    }

    #[test]
    fn test_phonetic_conversion() {
        assert_eq!(slugify("影師嗎", "", "-", None, false, 5), "ying-shi-ma");
    }

    #[test]
    fn test_accented_text() {
        assert_eq!(slugify("Æúű--cool?", "", "-", None, false, 5), "aeuu-cool");
        assert_eq!(
            slugify("Nín hǎo. Wǒ shì zhōng guó rén", "", "-", None, false, 5),
            "nin-hao-wo-shi-zhong-guo-ren"
        );
    }

    #[test]
    fn test_accented_text_non_word_chars() {
        assert_eq!(slugify!("jaja---lol-méméméoo--a"), "jaja-lol-mememeoo-a");
    }

    #[test]
    fn test_cyrillic_text() {
        assert_eq!(slugify!("Компьютер"), "komp-iuter");
    }

    #[test]
    fn test_macro() {
        assert_eq!(slugify!("Компьютер"), "komp-iuter");
        assert_eq!(slugify!("hello world", separator = "-"), "hello-world");
        assert_eq!(slugify!("hello world", separator = " "), "hello world");
        assert_eq!(slugify!("hello world", max_length = 5), "hello");
        assert_eq!(slugify!("hello world", max_length = 6), "hello");
        assert_eq!(
            slugify!("hello world", separator = " ", max_length = 8),
            "hello wo"
        );
        assert_eq!(
            slugify!("hello world", separator = "x", max_length = 8),
            "helloxwo"
        );
        assert_eq!(
            slugify!("the hello world", stop_words = "the", separator = "-"),
            "hello-world"
        );
        assert_eq!(
            slugify!("the hello world", stop_words = "the", max_length = 5),
            "hello"
        );
        assert_eq!(
            slugify!(
                "the hello world",
                stop_words = "the",
                separator = "-",
                max_length = 10
            ),
            "hello-worl"
        );
        assert_eq!(
            slugify!(
                "the hello world",
                stop_words = "the",
                separator = "-",
                max_length = 20
            ),
            "hello-world"
        );
    }
}
