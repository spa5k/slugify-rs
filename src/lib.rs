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
//! slugify-rs = "0.0.2"
//! ```
//!
//!  and this to your crate root:
//!
//!```rust,ignore
//!use slugify-rs::slugify;
//!```
//!
//!# Examples
//!
//!## Basic slug generation
//!
//!```rust
//! # use slugify_rs::slugify;
//! # fn main() {
//!assert_eq!(slugify!("hello world"), "hello-world");
//! # }
//!```
//!
//!## Using a custom separator
//!
//! ```rust
//! # use slugify_rs::slugify;
//! # fn main() {
//!assert_eq!(slugify!("hello world", separator = "."), "hello.world");
//!assert_eq!(slugify!("hello world", separator = " "), "hello world");
//!assert_eq!(slugify!("hello world", separator = ""), "helloworld");
//! # }
//! ```
//!
//!## Stop words filtering
//!
//!```rust
//! # use slugify_rs::slugify;
//! # fn main() {
//!assert_eq!(slugify!("the quick brown fox jumps over the lazy dog", stop_words = "the,fox"), "quick-brown-jumps-over-lazy-dog");
//! # }
//!```
//!
//!## Maximum length
//!
//!```rust
//! # use slugify_rs::slugify;
//! # fn main() {
//!assert_eq!(slugify!("hello world", max_length = 5), "hello");
//!assert_eq!(slugify!("the hello world", stop_words = "the", max_length = 5), "hello");
//! # }
//!```
//!
//!## Random values added to string through nanoid
//! ```rust
//! # use slugify_rs::slugify;
//! # // Default randomness is of 5 characters
//! # fn main() {
//! assert_eq!(slugify!("hello world", randomness=true).len(), "hello-world".len()+5);
//! # }
//! ```
//! ```rust
//! # use slugify_rs::slugify;
//! # // You can also add custom length to the randomness
//! # fn main() {
//! assert_eq!(slugify!("hello world", randomness=true,randomness_length=8).len(), "hello-world".len()+8);
//! # }
//! ```
//!## Phonetic Conversion and accented text
//!
//!```rust
//! # use slugify_rs::{slugify, Case};
//! # fn main() {
//!assert_eq!(slugify!("影師嗎"), "ying-shi-ma");
//!assert_eq!(slugify!("Æúű--cool?", transform = Case::Lower), "aeuu-cool");
//!assert_eq!(slugify!("Nín hǎo. Wǒ shì zhōng guó rén", transform = Case::Same), "Nin-hao-Wo-shi-zhong-guo-ren");
//! # }
//!```
//!## Easily convert text between different letter cases
//!
//!```rust
//! # use slugify_rs::{slugify, Case};
//! # fn main() {
//!assert_eq!(slugify!("Hello World"), "hello-world");
//!assert_eq!(slugify!("Hello World", transform = Case::Same), "Hello-World");
//!assert_eq!(slugify!("hello world", transform = Case::Upper), "HELLO-WORLD");
//!assert_eq!(slugify!("HeLlO wOrLd", transform = Case::Lower), "hello-world");
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
//! # use slugify_rs::slugify;
//! # fn main() {
//!assert_eq!(slugify!("the hello world", stop_words = "the", separator = "-"), "hello-world");
//!assert_eq!(slugify!("the hello world", separator = ".", max_length = 10), "the.hello");
//!assert_eq!(slugify!("the hello world", stop_words = "the", max_length = 5), "hello");
//!assert_eq!(slugify!("the hello world", stop_words = "the", separator = "-", max_length = 20), "hello-world");
//! # }
//!```
//!
use deunicode::deunicode;

pub enum Case {
    Lower,
    Upper,
    Same,
}

#[macro_export]
macro_rules! slugify {
    ($text:expr) => {{
        slugify($text, "", "-", None, false, 5, None)
    }};

    // with a boolean
    ($text:expr, randomness=$bool:expr) => {{
        slugify($text, "", "-", None, $bool, 5, None)
    }};

    ($text:expr, randomness=$bool:expr,randomness_length=$usize:expr) => {{
        slugify($text, "", "-", None, $bool, $usize, None)
    }};

    ($text:expr, stop_words=$stopwords:expr) => {{
        slugify($text, $stopwords, "-", None, false, 5, None)
    }};

    ($text:expr, stop_words=$stopwords:expr,randomness=$bool:expr) => {{
        slugify($text, $stopwords, "-", None, $bool, 5, None)
    }};

    ($text:expr, stop_words=$stopwords:expr,randomness=$bool:expr,randomness_length=$usize:expr) => {{
        slugify($text, $stopwords, "-", None, $bool, $usize, None)
    }};

    ($text:expr, separator=$sep:expr) => {{
        slugify($text, "", $sep, None, false, 5, None)
    }};

    ($text:expr, separator=$sep:expr,randomness=$bool:expr) => {{
        slugify($text, "", $sep, None, $bool, 5, None)
    }};

    ($text:expr, separator=$sep:expr,randomness=$bool:expr,randomness_length=$usize:expr) => {{
        slugify($text, "", $sep, None, $bool, $usize, None)
    }};

    ($text:expr, max_length=$len:expr) => {{
        slugify($text, "", "-", Some($len), false, 5, None)
    }};

    ($text:expr, max_length=$len:expr,randomness=$bool:expr) => {{
        slugify($text, "", "-", Some($len), $bool, 5, None)
    }};

    ($text:expr, max_length=$len:expr,randomness=$bool:expr,randomness_length=$usize:expr) => {{
        slugify($text, "", "-", Some($len), $bool, $usize, None)
    }};

    ($text:expr, stop_words=$stopwords:expr, separator=$sep:expr) => {{
        slugify($text, $stopwords, $sep, None, false, 5, None)
    }};

    ($text:expr, stop_words=$stopwords:expr, separator=$sep:expr,randomness=$bool:expr) => {{
        slugify($text, $stopwords, $sep, None, $bool, 5, None)
    }};

    ($text:expr, stop_words=$stopwords:expr, separator=$sep:expr,randomness=$bool:expr,randomness_length=$usize:expr $(, transform=$case:expr )?) => {{
        let case = None;
        $ ( case = Some($case); )?
        slugify($text, $stopwords, $sep, None, $bool, $usize, case)
    }};

    ($text:expr, stop_words=$stopwords:expr, max_length=$len:expr) => {{
        slugify($text, $stopwords, "-", Some($len), false, 5, None)
    }};

    ($text:expr, stop_words=$stopwords:expr, max_length=$len:expr,randomness=$bool:expr) => {{
        slugify($text, $stopwords, "-", Some($len), $bool, 5, None)
    }};

    ($text:expr, stop_words=$stopwords:expr, max_length=$len:expr,randomness=$bool:expr,randomness_length=$usize:expr $(, transform=$case:expr )?) => {{
        let case = None;
        $ ( case = Some($case); )?
        slugify($text, $stopwords, "-", Some($len), $bool, $usize, case)
    }};

    ($text:expr, separator=$sep:expr, max_length=$len:expr) => {{
        slugify($text, "", $sep, Some($len), false, 5, None)
    }};

    ($text:expr, separator=$sep:expr, max_length=$len:expr,randomness=$bool:expr) => {{
        slugify($text, "", $sep, Some($len), $bool, 5, None)
    }};

    ($text:expr, separator=$sep:expr, max_length=$len:expr,randomness=$bool:expr,randomness_length=$usize:expr $(, transform=$case:expr )?) => {{
        let mut case = None;
        $ ( case = Some($case); )?
        slugify($text, "", $sep, Some($len), $bool, $usize, case)
    }};

    ($text:expr, stop_words=$stopwords:expr, separator=$sep:expr, max_length=$len:expr $(, transform=$case:expr )?) => {{
        let case = None;
        $ ( case = Some($case); )?
        slugify($text, $stopwords, $sep, Some($len), false, 5, case)
    }};

    ($text:expr, stop_words=$stopwords:expr, separator=$sep:expr, max_length=$len:expr,randomness=$bool:expr $(, transform=$case:expr )?) => {{
        let mut case = None;
        $ ( case = Some($case); )?
        slugify($text, $stopwords, $sep, Some($len), $bool, 5, case)
    }};

    ($text:expr, stop_words=$stopwords:expr, separator=$sep:expr, max_length=$len:expr,randomness=$bool:expr,randomness_length=$usize:expr $(, transform=$case:expr )?) => {{
        let mut case = None;
        $ ( case = Some($case); )?
        slugify($text, $stopwords, $sep, Some($len), $bool, $usize, case)
    }};

    ($text:expr, transform=$case:expr) => {{
        slugify($text, "", "-", None, false, 5, Some($case))
    }};

    ($text:expr, randomness=$bool:expr, transform=$case:expr) => {{
        slugify($text, "", "-", None, $bool, 5, Some($case))
    }};

    ($text:expr, randomness=$bool:expr, randomness_length=$usize:expr, transform=$case:expr) => {{
        slugify($text, "", "-", None, $bool, $usize, Some($case))
    }};

    ($text:expr, stop_words=$stopwords:expr, transform=$case:expr) => {{
        slugify($text, $stopwords, "-", None, false, 5, Some($case))
    }};

    ($text:expr, stop_words=$stopwords:expr, separator=$sep:expr, transform=$case:expr) => {{
        slugify($text, $stopwords, $sep, None, false, 5, Some($case))
    }};

    ($text:expr, stop_words=$stopwords:expr, separator=$sep:expr, randomness=$bool:expr, transform=$case:expr) => {{
        slugify($text, $stopwords, $sep, None, $bool, 5, Some($case))
    }};

    ($text:expr, stop_words=$stopwords:expr, max_length=$len:expr, transform=$case:expr) => {{
        slugify($text, $stopwords, "-", Some($len), false, 5, Some($case))
    }};

    ($text:expr, stop_words=$stopwords:expr, max_length=$len:expr,randomness=$bool:expr, transform=$case:expr) => {{
        slugify($text, $stopwords, "-", Some($len), $bool, 5, Some($case))
    }};

    ($text:expr, separator=$sep:expr, max_length=$len:expr, transform=$case:expr) => {{
        slugify($text, "", $sep, Some($len), false, 5, Some($case))
    }};

    ($text:expr, separator=$sep:expr, max_length=$len:expr,randomness=$bool:expr, transform=$case:expr) => {{
        slugify($text, "", $sep, Some($len), $bool, 5, Some($case))
    }};

}

pub fn slugify(
    string: &str,
    stop_words: &str,
    sep: &str,
    max_length: Option<usize>,
    randomness: bool,
    randomness_length: usize,
    transform: Option<Case>,
) -> String {
    let char_vec: Vec<char> = sep.chars().collect();
    let mut string: String = deunicode(string)
        .trim()
        .trim_matches(match char_vec.first() {
            Some(a) => a.to_owned(),
            None => ' ',
        })
        .replace(' ', sep);

    // remove stop words
    for word in stop_words.split(',') {
        if !word.is_empty() {
            string = string.to_lowercase().replace(word, sep);
        }
    }

    let mut slug = Vec::with_capacity(string.len());

    let mut is_sep = true;

    for x in string.chars() {
        match x {
            'A'..='Z' | 'a'..='z' | '0'..='9' => {
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

    if !char_vec.is_empty() && slug.last() == Some(&(char_vec[0] as u8)) {
        slug.pop();
    }

    let mut s = String::from_utf8(slug).unwrap();

    if let Some(x) = max_length {
        s.truncate(x);
        s = s.trim_end_matches(char_vec[0]).to_string();
    }

    // if randomness is true, generate a nanoid with of size 5 and append it to s
    if randomness {
        // Decrease one from randomness_length
        let randomness_length = randomness_length - 1;
        let nanoid = nanoid::nanoid!(randomness_length);
        // change letters to lowercase
        let nanoid = nanoid.to_lowercase();
        // append separator to infront of nanoid
        s.push_str(sep);
        s.push_str(&nanoid);
    }

    match transform {
        Some(Case::Lower) => s.to_lowercase(),
        Some(Case::Upper) => s.to_uppercase(),
        Some(Case::Same) => s,
        _ => s.to_lowercase(),
    }
}

#[cfg(test)]
mod tests {
    use crate::{slugify, Case};

    #[test]
    fn basic() {
        assert_eq!(
            slugify("hello world", "", "-", None, false, 5, None),
            "hello-world"
        );
        assert_eq!(
            slugify("hello world-", "", "-", None, false, 5, None),
            "hello-world"
        );
        assert_eq!(
            slugify("hello world ", "", "-", None, false, 5, None),
            "hello-world"
        );
        assert_eq!(
            slugify("hello world ", "", "-", None, true, 5, None).len(),
            "hello-world".len() + 5
        );
        assert_eq!(
            slugify("hello world ", "", "-", None, false, 5, Some(Case::Upper)),
            "HELLO-WORLD"
        );
        assert_eq!(
            slugify("Hello World ", "", "-", None, false, 5, Some(Case::Same)),
            "Hello-World"
        );
        assert_eq!(
            slugify("hello world ", "", "", None, false, 5, None),
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
            "hello-world".len() + 5
        );
    }

    #[test]
    fn test_starts_with_number() {
        assert_eq!(slugify!("10 amazing secrets"), "10-amazing-secrets");
    }

    #[test]
    fn test_contains_numbers() {
        assert_eq!(slugify!("the 101 Dalmatians"), "the-101-dalmatians");
        assert_eq!(
            slugify!("the 101 dalmatians", randomness = true).len(),
            "the-101-dalmatians".len() + 5
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
            slugify("hello world", "world", "-", None, false, 5, None),
            "hello",
        );
        assert_eq!(slugify!("hello world", stop_words = "world"), "hello");
        assert_eq!(
            slugify!("hello world", stop_words = "world", randomness = true).len(),
            "hello".len() + 5
        );
    }

    #[test]
    fn test_differently_cased_stopword_match() {
        assert_eq!(
            slugify("Foo A FOO B foo C", "foo", "-", None, false, 5, None),
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
                5,
                None
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
                5,
                None
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
                5,
                None
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
                8,
                None
            )
            .len(),
            "quick brown fox jumps over lazy dog".len() + 8
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
            "quick brown fox jumps over lazy dog".len() + 5
        );

        assert_eq!(
            slugify!(
                "the quick brown fox jumps over the lazy dog",
                stop_words = "the",
                separator = " ",
                randomness = true,
                randomness_length = 10
            )
            .len(),
            "quick brown fox jumps over lazy dog".len() + 10
        );
    }

    #[test]
    fn test_separator() {
        assert_eq!(
            slugify("hello world", "", ".", None, false, 5, None),
            "hello.world"
        );

        assert_eq!(
            slugify("hello world", "", "_", None, false, 5, None),
            "hello_world"
        );
        assert_eq!(slugify!("hello world", separator = "_"), "hello_world");
        assert_eq!(
            slugify!("hello world-", separator = "_", randomness = true).len(),
            "hello_world".len() + 5
        );
    }

    #[test]
    fn test_phonetic_conversion() {
        assert_eq!(
            slugify("影師嗎", "", "-", None, false, 5, Some(Case::Same)),
            "Ying-Shi-Ma"
        );
        assert_eq!(
            slugify("影師嗎", "", "-", None, false, 5, Some(Case::Lower)),
            "ying-shi-ma"
        );
        assert_eq!(
            slugify("影師嗎", "", "-", None, false, 5, None),
            "ying-shi-ma"
        );
    }

    #[test]
    fn test_accented_text() {
        assert_eq!(
            slugify("hello world", "", ".", None, false, 5, None),
            "hello.world"
        );
        assert_eq!(
            slugify(
                "Nín hǎo. Wǒ shì zhōng guó rén",
                "",
                "-",
                None,
                false,
                5,
                Some(Case::Lower)
            ),
            "nin-hao-wo-shi-zhong-guo-ren"
        );
        assert_eq!(
            slugify(
                "Nín hǎo. Wǒ shì zhōng guó rén",
                "",
                "-",
                None,
                false,
                5,
                None
            ),
            "nin-hao-wo-shi-zhong-guo-ren"
        );
        assert_eq!(
            slugify(
                "Nín hǎo. Wǒ shì zhōng guó rén",
                "",
                "-",
                None,
                false,
                5,
                Some(Case::Same)
            ),
            "Nin-hao-Wo-shi-zhong-guo-ren"
        );
    }

    #[test]
    fn test_convert_case() {
        assert_eq!(
            slugify("Hello World", "", "-", None, false, 5, None),
            "hello-world",
        );
        assert_eq!(
            slugify!("Hello World", transform = Case::Lower),
            "hello-world"
        );
        assert_eq!(
            slugify!("hello world", transform = Case::Upper),
            "HELLO-WORLD"
        );
        assert_eq!(
            slugify!("Hello World", transform = Case::Same),
            "Hello-World"
        );
    }

    #[test]
    fn test_accented_text_non_word_chars() {
        assert_eq!(slugify!("jaja---lol-méméméoo--a"), "jaja-lol-mememeoo-a")
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
            slugify!("Hello world", separator = " ", max_length = 8),
            "hello wo"
        );
        assert_eq!(
            slugify!("hello world", separator = "x", max_length = 8),
            "helloxwo"
        );
        assert_eq!(
            slugify!(
                "the hello world",
                stop_words = "the",
                separator = "-",
                transform = Case::Upper
            ),
            "HELLO-WORLD"
        );
        assert_eq!(
            slugify!(
                "The Great Khan",
                stop_words = "the",
                separator = "-",
                transform = Case::Lower
            ),
            "great-khan"
        );
        assert_eq!(
            slugify!("the hello world", stop_words = "the", max_length = 5),
            "hello"
        );
        assert_eq!(
            slugify!(
                "The Hello World",
                stop_words = "the",
                max_length = 5,
                transform = Case::Lower
            ),
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
