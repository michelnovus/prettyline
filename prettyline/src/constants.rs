// [MIT License] Copyright (c) 2024 Michel Novus
//! Symbols and colors are defined by constants here.

/// Symbols that can be used at extremes of segments.
pub mod symbols {
    pub const R_ANGLED_FILL: &'static str = "\u{E0B0}"; // 
    pub const L_ANGLED_FILL: &'static str = "\u{E0B2}"; // 
    pub const R_ANGLED_FLAT: &'static str = "\u{E0B1}"; // 
    pub const L_ANGLED_FLAT: &'static str = "\u{E0B3}"; // 
    pub const R_CURVED_FILL: &'static str = "\u{E0B4}"; // 
    pub const L_CURVED_FILL: &'static str = "\u{E0B6}"; // 
    pub const R_CURVED_FLAT: &'static str = "\u{E0B5}"; // 
    pub const L_CURVED_FLAT: &'static str = "\u{E0B7}"; // 
    pub const HONEYCOMB_FILL: &'static str = "\u{E0CC}"; // 
    pub const HONEYCOMB_FLAT: &'static str = "\u{E0CD}"; // 
    pub const BRANCH: &'static str = "\u{E0A0}"; // 
}

/// The colors of segments background and text.
///
/// Each color is defined as the `Color` enum of `anstyle` crate.
pub mod colors {
    use anstyle::{Ansi256Color, AnsiColor, Color};
    pub const USER_NORM_FG: Color = Color::Ansi(AnsiColor::Black);
    pub const USER_NORM_BG: Color = Color::Ansi(AnsiColor::BrightWhite);
    pub const USER_SUDO_FG: Color = Color::Ansi(AnsiColor::Black);
    pub const USER_SUDO_BG: Color = Color::Ansi256(Ansi256Color(55));
    pub const USER_ROOT_FG: Color = Color::Ansi(AnsiColor::Black);
    pub const USER_ROOT_BG: Color = Color::Ansi(AnsiColor::BrightRed);

    pub const EXITCODE_SUCCESS_FG: Color = Color::Ansi(AnsiColor::Black);
    pub const EXITCODE_SUCCESS_BG: Color = Color::Ansi(AnsiColor::BrightBlue);
    pub const EXITCODE_FAILED_FG: Color = Color::Ansi(AnsiColor::Black);
    pub const EXITCODE_FAILED_BG: Color = Color::Ansi(AnsiColor::BrightRed);

    pub const TIME_FG: Color = Color::Ansi(AnsiColor::White);
    pub const TIME_BG: Color = Color::Ansi256(Ansi256Color(237));

    pub const VENV_PYTHON_FG: Color = Color::Ansi256(Ansi256Color(220));
    pub const VENV_PYTHON_BG: Color = Color::Ansi256(Ansi256Color(25));
}
