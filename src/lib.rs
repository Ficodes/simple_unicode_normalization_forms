// Copyright (c) 2024 Future Internet Consulting and Development Solutions S.L.

use lazy_static::lazy_static;
use regex::Regex;
use pyo3::prelude::*;
use std::collections::HashSet;
use unicode_normalization::char::decompose_compatible;
use unicode_normalization::UnicodeNormalization;

lazy_static! {
    static ref WHITESPACE_RE: Regex = Regex::new(r"\s+").unwrap();
    static ref EMOJI_RE: Regex = Regex::new(r"[\p{Emoji_Presentation}\p{Emoji_Modifier}\p{Emoji_Modifier_Base}\{Cc}\uFE0E\uFE0F\u20E2\u20E3\u20E4]").unwrap();
}

/// Gives the normalized form of a string skipping some characters.
fn nfkc_normalization(str: String, allow_chars: HashSet<char>) -> String {
    let mut result = String::with_capacity(str.len());
    for c in str.chars() {
        if allow_chars.contains(&c) {
            result.push(c)
        } else {
            decompose_compatible(c, |r| {
                // Ignore characters outside the Basic Multilingual Plane and in the disallow_chars set
                if r <= '\u{FFFF}' {
                    result.push(r)
                }
            })
        }
    }

    result.nfc().collect::<String>()
}

#[pyfunction]
fn basic_string_clean(value: String) -> PyResult<String> {
    Ok(nfkc_normalization(value, HashSet::from(['º', 'ª'])).trim().to_string())
}

#[pyfunction]
fn remove_emojis(value: String) -> PyResult<String> {
    let cleaned_value = nfkc_normalization(value, HashSet::from(['º', 'ª']));
    let whitespace_cleaned_value = WHITESPACE_RE.replace_all(&cleaned_value, " ");
    let result = EMOJI_RE.replace_all(&whitespace_cleaned_value, "");

    Ok(result.trim().to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn simple_unicode_normalization_forms(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(basic_string_clean, m)?)?;
    m.add_function(wrap_pyfunction!(remove_emojis, m)?)?;
    Ok(())
}
