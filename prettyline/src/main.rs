// [MIT License] Copyright (c) 2024 Michel Novus

use anstyle::RgbColor;
use anyhow::{anyhow, Result};
use chrono::Local;
use clap::Parser;
use constants::{colors, symbols};
use std::{
    env,
    io::{self, prelude::*},
};

fn main() -> Result<()> {
    let args = setup::Args::parse();

    if args.init {
        return match args.shell {
            setup::ShellName::Bash => Ok(setup::install::bash()),
            setup::ShellName::Zsh => Ok(setup::install::zsh()),
            setup::ShellName::Fish => Ok(setup::install::fish()),
        };
    }

    let left_prompt: String = {
        let mut segments: Vec<prompt::Segment> = vec![];

        let username = match env::var_os("USER")
            .unwrap_or_else(|| "???".into())
            .into_string()
        {
            Ok(val) => val,
            Err(_) => {
                return Err(anyhow!(
                    "It seems that the username has strange characters"
                ))
            }
        };
        let user_segment = prompt::Segment {
            left: None,
            center: match username.as_str() {
                "root" => prompt::Chunk::new(&username)
                    .fg(colors::USER_ROOT_FG)
                    .bg(colors::USER_ROOT_BG)
                    .weight(prompt::TextWeight::Bold)
                    .pad(),
                _ => prompt::Chunk::new(&username)
                    .fg(colors::USER_NORM_FG)
                    .bg(colors::USER_NORM_BG)
                    .weight(prompt::TextWeight::Bold)
                    .pad(),
            },
            right: Some(match username.as_str() {
                "root" => prompt::Chunk::new(symbols::R_ANGLED_FILL)
                    .fg(colors::USER_ROOT_BG)
                    .bg(RgbColor(18, 18, 18).into()),
                _ => prompt::Chunk::new(symbols::R_ANGLED_FILL)
                    .fg(colors::USER_NORM_BG)
                    .bg(RgbColor(18, 18, 18).into()),
            }),
        };
        segments.push(user_segment);

        let exit_status = match args.exit_status {
            Some(value) => format!("E{}", value),
            None => "E?".into(),
        };
        let (exit_color_fg, exit_color_bg) = match args.exit_status {
            Some(val) if val == 0u8 => {
                (colors::EXITCODE_SUCCESS_FG, colors::EXITCODE_SUCCESS_BG)
            }
            Some(val) if val != 0u8 => {
                (colors::EXITCODE_FAILED_FG, colors::EXITCODE_FAILED_BG)
            }
            Some(_) => unreachable!(),
            None => (colors::EXITCODE_FAILED_FG, colors::EXITCODE_FAILED_BG),
        };
        let exitcode_segment = prompt::Segment {
            left: Some(
                prompt::Chunk::new(constants::symbols::R_ANGLED_FILL)
                    .fg(RgbColor(18, 18, 18).into())
                    .bg(exit_color_bg),
            ),
            center: prompt::Chunk::new(&exit_status)
                .fg(exit_color_fg)
                .bg(exit_color_bg)
                .pad(),
            right: Some(
                prompt::Chunk::new(constants::symbols::R_ANGLED_FILL)
                    .fg(exit_color_bg),
            ),
        };
        segments.push(exitcode_segment);

        segments
            .iter()
            .map(|segment| segment.to_string())
            .collect::<String>()
    };

    let right_prompt: String = {
        let mut segments: Vec<prompt::Segment> = vec![];

        let current_time: String = Local::now().format("%H:%M").to_string();
        let time = prompt::Segment {
            left: Some(
                prompt::Chunk::new(constants::symbols::L_CURVED_FILL)
                    .fg(constants::colors::TIME_BG),
            ),
            center: prompt::Chunk::new(&current_time)
                .bg(constants::colors::TIME_BG)
                .fg(constants::colors::TIME_FG)
                .weight(prompt::TextWeight::Dimm),
            right: Some(
                prompt::Chunk::new(constants::symbols::R_CURVED_FILL)
                    .fg(constants::colors::TIME_BG),
            ),
        };
        segments.push(time);

        segments
            .iter()
            .map(|segment| segment.to_string())
            .collect::<String>()
    };

    let (left_prompt, right_prompt) = match args.shell {
        setup::ShellName::Bash => unimplemented!(),
        setup::ShellName::Zsh => {
            (
                misc::ansi_escape_wrapper(&left_prompt, "%{", "%}"), 
                misc::ansi_escape_wrapper(&right_prompt, "%{", "%}")
            )
        },
        setup::ShellName::Fish => (left_prompt, right_prompt),
    };

    if args.show_lprompt {
        let mut stdout = io::stdout();
        stdout.write_all(left_prompt.as_bytes())?;
        stdout.write(" ".as_bytes())?;
        stdout.flush()?;
    }
    if args.show_rprompt {
        let mut stdout = io::stdout();
        stdout.write_all(right_prompt.as_bytes())?;
        stdout.write(" ".as_bytes())?;
        stdout.flush()?;
    }

    Ok(())
}

/// Symbols and colors are defined by constants here.
pub mod constants {
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
        use anstyle::{AnsiColor, Color};
        pub const USER_NORM_FG: Color = Color::Ansi(AnsiColor::Black);
        pub const USER_NORM_BG: Color = Color::Ansi(AnsiColor::BrightWhite);
        pub const USER_SUDO_FG: Color = Color::Ansi(AnsiColor::Black);
        pub const USER_SUDO_BG: Color = Color::Ansi(AnsiColor::BrightYellow);
        pub const USER_ROOT_FG: Color = Color::Ansi(AnsiColor::Black);
        pub const USER_ROOT_BG: Color = Color::Ansi(AnsiColor::BrightRed);

        pub const EXITCODE_SUCCESS_FG: Color = Color::Ansi(AnsiColor::Black);
        pub const EXITCODE_SUCCESS_BG: Color =
            Color::Ansi(AnsiColor::BrightBlue);
        pub const EXITCODE_FAILED_FG: Color = Color::Ansi(AnsiColor::Black);
        pub const EXITCODE_FAILED_BG: Color = Color::Ansi(AnsiColor::BrightRed);

        pub const TIME_FG: Color = Color::Ansi(AnsiColor::Black);
        pub const TIME_BG: Color = Color::Ansi(AnsiColor::White);
    }
}

/// Contains things related to program configuration.
pub mod setup {
    use clap::{ArgGroup, Parser, ValueEnum};

    /// Expected arguments entered when starting the program.
    ///
    /// The user only need call the `--init {SHELL}` argument inside
    /// `eval "$(prettyline --init SHELLNAME)"`.
    #[derive(Debug, Parser)]
    #[command(version, about, long_about=None, 
        group = ArgGroup::new("required")
            .args(&["init", "shell", "show_lprompt", "show_rprompt", "exit_status"])
            .required(true)
            .multiple(true)
    )]
    pub struct Args {
        /// Sets shell settings.
        #[arg(long)]
        pub init: bool,
        #[arg(long, value_name = "SHELL")]
        pub shell: ShellName,
        #[arg(long, hide = true)]
        pub show_lprompt: bool,
        #[arg(long, hide = true)]
        pub show_rprompt: bool,
        #[arg(long, hide = true)]
        pub exit_status: Option<u8>,
    }

    /// Supported Shells.
    #[derive(Debug, Clone, Copy, ValueEnum)]
    pub enum ShellName {
        Bash,
        Zsh,
        Fish,
    }

    /// Each function prints to stdout the necessary configuration
    /// to run the program correctly.
    pub mod install {
        pub fn bash() {
            unimplemented!()
        }
        pub fn zsh() {
            let script = "\
            function precmd() {\n    \
                PROMPT=\"$(prettyline --shell zsh --show-lprompt --exit-status $?)\"\n    \
                RPROMPT=\"$(prettyline --shell zsh --show-rprompt)\"\n\
            }\
            ";
            println!("{}", script);
        }
        pub fn fish() {
            let script = "\
            function fish_prompt\n    \
                command prettyline --shell fish --show-lprompt --exit-status $status\n\
            end\n\
            function fish_right_prompt\n    \
                command prettyline --shell fish --show-rprompt\n\
            end\
            ";
            println!("{}", script);
        }
    }
}

pub mod misc {
    use regex::{Regex, Captures};

    /// Wraps ANSI escape sequences inside `string` with the
    /// supplied `start` and `end` characters.
    pub fn ansi_escape_wrapper(string: &str, start: &str, end: &str) -> String {
        let ansi_re = Regex::new(r"\u{1b}\[.*?m").unwrap();
        ansi_re.replace_all(
            string, 
            |esc: &Captures| format!("{start}{}{end}", &esc[0])
        ).to_string()
    }
}

/// Defines printable structures.
pub mod prompt {
    use anstyle::{Color, Style};
    use std::fmt::Display;

    /// Main element that allows to encapsulate each piece of the prompt.
    ///
    /// It contains three parts, the left character, the text in the middle
    /// and the right character; the left and right characters are extremes
    /// of the segment and should be one of those defined in the
    /// `constants::symbols` module.
    #[derive(Debug)]
    pub struct Segment<'a> {
        pub left: Option<Chunk<'a>>,
        pub center: Chunk<'a>,
        pub right: Option<Chunk<'a>>,
    }
    impl<'a> Display for Segment<'a> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let left: String = match self.left.as_ref() {
                Some(chunk) => chunk.to_string(),
                None => String::new(),
            };
            let center: String = self.center.to_string();
            let right: String = match self.right.as_ref() {
                Some(chunk) => chunk.to_string(),
                None => String::new(),
            };
            write!(f, "{}{}{}", left, center, right)
        }
    }

    /// Defines whether the text should be thick (bold) or thin (dimm).
    #[derive(Debug)]
    pub enum TextWeight {
        Bold,
        Dimm,
    }

    /// Defines how the text should appear.
    #[derive(Debug, Default)]
    pub struct Chunk<'a> {
        /// The text.
        pub value: &'a str,
        /// The text has a bold, dimm or normal (when it is `None`).
        pub weight: Option<TextWeight>,
        /// The text color itself.
        pub fg_color: Option<Color>,
        /// The background color of text.
        pub bg_color: Option<Color>,
        /// Adds a spaces arround text.
        pad: bool,
    }
    impl<'a> Chunk<'a> {
        pub fn new(text: &'a str) -> Self {
            Self {
                value: text,
                weight: None,
                fg_color: None,
                bg_color: None,
                pad: false,
            }
        }
        /// Sets the text color.
        pub fn fg(mut self, color: Color) -> Self {
            self.fg_color = Some(color);
            self
        }
        /// Sets the background text color.
        pub fn bg(mut self, color: Color) -> Self {
            self.bg_color = Some(color);
            self
        }
        /// Sets text weight (bold, dimm or normal).
        pub fn weight(mut self, weight: TextWeight) -> Self {
            self.weight = Some(weight);
            self
        }
        /// Turn a spaces around text on or off.
        pub fn pad(mut self) -> Self {
            self.pad = !self.pad;
            self
        }
    }
    impl<'a> Display for Chunk<'a> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut style =
                Style::new().fg_color(self.fg_color).bg_color(self.bg_color);
            if let Some(weight) = &self.weight {
                style = match weight {
                    TextWeight::Bold => style.bold(),
                    TextWeight::Dimm => style.dimmed(),
                }
            }
            if self.pad {
                write!(f, "{style} {} {style:#}", self.value)
            } else {
                write!(f, "{style}{}{style:#}", self.value)
            }
        }
    }
}
