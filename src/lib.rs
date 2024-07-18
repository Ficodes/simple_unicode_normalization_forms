// Copyright (c) 2024 Future Internet Consulting and Development Solutions S.L.
mod emoji;

use emoji::IsEmoji;
use lazy_static::lazy_static;
use pyo3::prelude::*;
use regex::Regex;
use unicode_normalization::char::decompose_compatible;
use unicode_normalization::UnicodeNormalization;

lazy_static! {
    static ref EMOJI_RE: Regex = Regex::new(r"[\p{Emoji_Presentation}\p{Emoji_Modifier}\p{Emoji_Modifier_Base}\{Cc}\uFE0E\uFE0F\u20E2\u20E3\u20E4]").unwrap();
}

/// Gives the normalized form of a string skipping some characters.
fn custom_normalization(
    str: String,
    allow_chars: Vec<char>,
    collapse_whitespace: bool,
    remove_emojis: bool,
) -> String {
    let mut result = String::with_capacity(str.len());
    let mut previous_whitespace = false;
    for c in str.chars() {
        custom_character_normalization(
            &mut result,
            c,
            &allow_chars,
            collapse_whitespace,
            previous_whitespace,
            remove_emojis,
        );
        previous_whitespace = c.is_whitespace();
    }
    result.nfc().collect::<String>()
}

fn custom_character_normalization(
    str: &mut String,
    c: char,
    allow_chars: &Vec<char>,
    collapse_whitespace: bool,
    previous_whitespace: bool,
    remove_emojis: bool,
) {
    if allow_chars.contains(&c) {
        str.push(c)
    } else if c.is_whitespace() {
        if collapse_whitespace && previous_whitespace {
            return;
        } else {
            str.push(' ')
        }
    } else if remove_emojis && c.is_emoji() {
        return;
    } else {
        decompose_compatible(c, |r| {
            // Ignore characters outside the Basic Multilingual Plane and in the disallow_chars set
            if r <= '\u{FFFF}' {
                str.push(r)
            }
        })
    }
}

#[pyfunction]
fn basic_string_clean(value: String) -> PyResult<String> {
    Ok(custom_normalization(value, vec!['º', 'ª'], false, false)
        .trim()
        .to_string())
}

#[pyfunction]
fn remove_emojis(value: String) -> PyResult<String> {
    let result = custom_normalization(value, vec!['º', 'ª'], true, true);
    Ok(result.trim().to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn simple_unicode_normalization_forms(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(basic_string_clean, m)?)?;
    m.add_function(wrap_pyfunction!(remove_emojis, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::remove_emojis;
    use std::time::Instant;

    #[test]
    fn timeit() {
        let t1 = Instant::now();
        remove_emojis("  a\t   name with ❤️✳️0️⃣#️⃣  #©*1   ".to_string());
        let t2 = Instant::now();
        println!("{:?}", t2 - t1);
    }
}
