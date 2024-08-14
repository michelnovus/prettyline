// [MIT License] Copyright (c) 2024 Michel Novus

use regex::{Captures, Regex};

/// Wraps ANSI escape sequences inside `string` with the
/// supplied `start` and `end` characters.
pub fn ansi_escape_wrapper(string: &str, start: &str, end: &str) -> String {
    let ansi_re = Regex::new(r"\u{1b}\[.*?m").unwrap();
    ansi_re
        .replace_all(string, |esc: &Captures| {
            format!("{start}{}{end}", &esc[0])
        })
        .to_string()
}
