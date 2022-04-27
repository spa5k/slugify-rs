# Slugify-rs
A utility macro for flexible slug genereation that handles unicode.

The slugify! macro implements a flexible slug generator, allowing for stop words, custom separator and maximum length options. The macro provides both a simple interface with sane default parameters but also allows the parameters to be overriden when needed.

## Features:

- Unicode strings support (phonetic conversion).
- Support for custom slug separator.
- Stop words filtering.
- Slug maximum length support.

## Usage
This crate is on crates.io and can be used by adding slugify to the dependencies in your project’s Cargo.toml
```toml
[dependencies]
slugify-rs = "0.0.1"
```
Examples
Basic slug generation
```rust
assert_eq!(slugify!("hello world"), "hello-world");

// Using a custom separator
assert_eq!(slugify!("hello world", separator = "."), "hello.world");
assert_eq!(slugify!("hello world", separator = " "), "hello world");
assert_eq!(slugify!("hello world", separator = ""), "helloworld");

// Stop words filtering
assert_eq!(slugify!("the quick brown fox jumps over the lazy dog", stop_words = "the,fox"), "quick-brown-jumps-over-lazy-dog");

// Maximum length
assert_eq!(slugify!("hello world", max_length = 5), "hello");
assert_eq!(slugify!("the hello world", stop_words = "the", max_length = 5), "hello");

// Random values added to string through nanoid
assert_eq!(slugify!("hello world", randomness=true).len(), "hello-world".len()+6);
assert_eq!(slugify!("hello world", randomness=true,randomness_length=8).len(), "hello-world".len()+9);

// Phonetic Conversion and accented text
assert_eq!(slugify!("影師嗎"), "ying-shi-ma");
assert_eq!(slugify!("Æúű--cool?"), "aeuu-cool");
assert_eq!(slugify!("Nín hǎo. Wǒ shì zhōng guó rén"), "nin-hao-wo-shi-zhong-guo-ren");

// Passing multiple optional parameters.
// NOTE: the order of optional parameters matters: stop_words, separator and then max_length. All of them are optional, however when specifying more than one optional parameter, this order must be adhered.

assert_eq!(slugify!("the hello world", stop_words = "the", separator = "-"), "hello-world");
assert_eq!(slugify!("the hello world", separator = ".", max_length = 10), "the.hello");
assert_eq!(slugify!("the hello world", stop_words = "the", max_length = 5), "hello");
assert_eq!(slugify!("the hello world", stop_words = "the", separator = "-", max_length = 20), "hello-world");
```

# Info

This slug was forked from the original slugify crate by [@mattgathu](https://github.com/mattgathu/slugify)