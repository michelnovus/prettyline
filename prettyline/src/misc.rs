// [MIT License] Copyright (c) 2024 Michel Novus

use anstyle::Color;
use regex::{Captures, Regex};
use std::env;

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

/// Returns the value of the terminal background color if it can be found.
///
/// For now look for an environment variable called `TERM_BG_COLOR`,
/// containing a hexadecimal HTML color code like `#rrggbb` or `rrggbb`.
pub fn get_term_bg_color() -> Option<Color> {
    if let Some(color) = env::var_os("TERM_BG_COLOR") {
        let color = match color.into_string() {
            Ok(c) => c,
            Err(_) => return None,
        };
        let hex_color_re = Regex::new(
            r"^[#]?([0-9A-Fa-f]{2})([0-9A-Fa-f]{2})([0-9A-Fa-f]{2})$",
        )
        .unwrap();
        if hex_color_re.is_match(&color) {
            let cap = hex_color_re.captures(&color).unwrap();
            Some(Color::Rgb(anstyle::RgbColor(
                u8::from_str_radix(cap.get(1).unwrap().as_str(), 16).unwrap(),
                u8::from_str_radix(cap.get(2).unwrap().as_str(), 16).unwrap(),
                u8::from_str_radix(cap.get(3).unwrap().as_str(), 16).unwrap(),
            )))
        } else {
            None
        }
    } else {
        None
    }
}
