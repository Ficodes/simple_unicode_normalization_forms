use pyo3::prelude::*;
use unicode_normalization::char::decompose_compatible;

/// Gives the normalized form of a string skipping some characters.
#[pyfunction]
fn nfkc_normalization(str: String, allow_chars: Vec<char>) -> PyResult<String> {
    let mut result = String::with_capacity(str.len() * 2);
    for c in str.chars() {
        if allow_chars.contains(&c) {
            result.push(c)
        } else {
            decompose_compatible(c, |r| result.push(r))
        }
    }
    Ok(result)
}

/// A Python module implemented in Rust.
#[pymodule]
fn ficodes_string_normalization(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(nfkc_normalization, m)?)?;
    Ok(())
}
