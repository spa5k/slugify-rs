use crate::Case;
use deunicode::deunicode;

pub struct SlugBuilder {
    text: String,
    stop_words: String,
    sep: String,
    max_length: Option<usize>,
    randomness: bool,
    randomness_length: usize,
    transform: Option<Case>,
}

impl SlugBuilder {
    pub fn default() -> Self {
        SlugBuilder {
            text: String::default(),
            stop_words: String::default(),
            sep: "-".to_owned(),
            max_length: None,
            randomness: false,
            randomness_length: 5,
            transform: Some(Case::Lower),
        }
    }

    pub fn with_stop_words(self, stop_words: String) -> SlugBuilder {
        SlugBuilder { stop_words, ..self }
    }

    pub fn with_sep(self, sep: String) -> SlugBuilder {
        SlugBuilder { sep, ..self }
    }

    pub fn with_max_length(self, max_length: usize) -> SlugBuilder {
        SlugBuilder {
            max_length: Some(max_length),
            ..self
        }
    }

    pub fn with_randomness(self, randomness: bool) -> SlugBuilder {
        SlugBuilder { randomness, ..self }
    }

    pub fn with_randomness_length(self, randomness_length: usize) -> SlugBuilder {
        SlugBuilder {
            randomness_length,
            ..self
        }
    }

    pub fn with_transform(self, transform: Case) -> SlugBuilder {
        SlugBuilder {
            transform: Some(transform),
            ..self
        }
    }
}

impl SlugBuilder {
    pub fn execute(&self) -> String {
        let char_vec: Vec<char> = self.sep.chars().collect();
        let mut string: String = deunicode(&self.text)
            .trim()
            .trim_matches(match char_vec.first() {
                Some(a) => a.to_owned(),
                None => ' ',
            })
            .replace(' ', &self.sep);

        // remove stop words
        for word in self.stop_words.split(',') {
            if !word.is_empty() {
                string = string.to_lowercase().replace(word, &self.sep);
            }
        }

        let mut slug = String::with_capacity(string.len());

        let mut is_sep = true;

        for x in string.chars() {
            match x {
                'A'..='Z' | 'a'..='z' | '0'..='9' => {
                    is_sep = false;
                    slug.push(x);
                }
                _ => {
                    if !is_sep {
                        is_sep = true;
                        slug.push(char_vec[0]);
                    } else {
                    }
                }
            }
        }

        if !char_vec.is_empty() && slug.ends_with(char_vec[0]) {
            slug.pop();
        }

        let mut s = slug;

        if let Some(x) = self.max_length {
            s.truncate(x);
            s = s.trim_end_matches(char_vec[0]).to_string();
        }

        // if randomness is true, generate a nanoid with of size 5 and append it to s
        if self.randomness {
            // Decrease one from randomness_length
            let randomness_length = self.randomness_length - 1;
            let nanoid = nanoid::nanoid!(randomness_length);
            // change letters to lowercase
            let nanoid = nanoid.to_lowercase();
            // append separator to infront of nanoid
            s.push_str(&self.sep);
            s.push_str(&nanoid);
        }

        match self.transform {
            Some(Case::Lower) => s.to_ascii_lowercase(),
            Some(Case::Upper) => s.to_ascii_uppercase(),
            _ => s,
        }
    }
}

pub fn slugify(text: String) -> SlugBuilder {
    SlugBuilder {
        text,
        ..SlugBuilder::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn build_basic() {
        assert_eq!(slugify("Hello World".to_owned()).execute(), "hello-world");
        assert_eq!(
            slugify("Hello World".to_owned())
                .with_stop_words("world".to_owned())
                .execute(),
            "hello"
        );
        assert_eq!(
            slugify("hello world".to_owned())
                .with_sep("+".to_owned())
                .execute(),
            "hello+world"
        );
        assert_eq!(
            slugify("hello world".to_owned())
                .with_randomness(true)
                .execute()
                .len(),
            "hello-world".len() + 5
        );
        assert_eq!(
            slugify("hello world".to_owned())
                .with_transform(Case::Upper)
                .execute(),
            "HELLO-WORLD"
        );
        assert_eq!(
            slugify("Lorem ipsum is placeholder text commonly used in the graphic, print, and publishing industries for previewing layouts and visual mockups".to_owned())
                .with_sep("+".to_owned())
                .with_max_length(6)
                .with_transform(Case::Same)
                .execute(),
            "Lorem"
        );
    }
}
