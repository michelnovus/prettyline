// [MIT License] Copyright (c) 2024 Michel Novus

use anyhow::{anyhow, Result};
use clap::Parser;
use constants::{colors, symbols};
use std::env;

fn main() -> Result<()> {
    let args = setup::Args::parse();

    if args.init.is_some() {
        return match args.init.unwrap() {
            setup::ShellName::Bash => {
                setup::install::bash();
                Ok(())
            }
            setup::ShellName::Zsh => {
                setup::install::zsh();
                Ok(())
            }
            setup::ShellName::Fish => {
                setup::install::fish();
                Ok(())
            }
        };
    }

    let mut left_prompt: Vec<prompt::Segment> = vec![];

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
        center: prompt::Chunk::new(&username)
            .fg(colors::USER_NORM_FG)
            .bg(colors::USER_NORM_BG)
            .weight(prompt::TextWeight::Bold)
            .pad(),
        right: Some(
            prompt::Chunk::new(symbols::R_ANGLED_FILL).fg(colors::USER_NORM_BG),
        ),
    };
    left_prompt.push(user_segment);

    let git_segment = prompt::Segment {
        left: None,
        center: prompt::Chunk::new(""),
        right: None,
    };
    left_prompt.push(git_segment);

    let exit_status = match args.exit_status {
        Some(value) => format!("E{}", value),
        None => "E?".into(),
    };
    let exitcode_segment = prompt::Segment {
        left: None,
        center: prompt::Chunk::new(&exit_status),
        right: None,
    };
    left_prompt.push(exitcode_segment);

    println!("{:#?}", left_prompt);

    Ok(())
}

pub mod constants {
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
    pub mod colors {
        use anstyle::{AnsiColor, Color};
        pub const USER_NORM_FG: Color = Color::Ansi(AnsiColor::Black);
        pub const USER_NORM_BG: Color = Color::Ansi(AnsiColor::White);
        pub const USER_SUDO_FG: Color = Color::Ansi(AnsiColor::Black);
        pub const USER_SUDO_BG: Color = Color::Ansi(AnsiColor::BrightYellow);
        pub const USER_ROOT_FG: Color = Color::Ansi(AnsiColor::Black);
        pub const USER_ROOT_BG: Color = Color::Ansi(AnsiColor::BrightMagenta);

        pub const EXITCODE_SUCCESS_FG: Color = Color::Ansi(AnsiColor::Black);
        pub const EXITCODE_SUCCESS_BG: Color =
            Color::Ansi(AnsiColor::BrightBlue);
        pub const EXITCODE_FAILED_FG: Color = Color::Ansi(AnsiColor::Black);
        pub const EXITCODE_FAILED_BG: Color = Color::Ansi(AnsiColor::BrightRed);

        pub const TIME_FG: Color = Color::Ansi(AnsiColor::Black);
        pub const TIME_BG: Color = Color::Ansi(AnsiColor::White);
    }
}

pub mod setup {
    use clap::{ArgGroup, Parser, ValueEnum};

    #[derive(Debug, Parser)]
    #[command(
        group = ArgGroup::new("init-conflict")
            .arg("show_lprompt")
            .arg("show_lprompt")
            .arg("exit_status")
            .conflicts_with("init")           
    )]
    pub struct Args {
        /// Sets shell settings.
        #[arg(long, value_name = "SHELL")]
        pub init: Option<ShellName>,
        #[arg(long, hide = true)]
        pub show_lprompt: bool,
        #[arg(long, hide = true)]
        pub show_rprompt: bool,
        #[arg(long, hide = true)]
        pub exit_status: Option<u8>,
    }

    #[derive(Debug, Clone, Copy, ValueEnum)]
    pub enum ShellName {
        Bash,
        Zsh,
        Fish,
    }

    pub mod install {
        pub fn bash() {
            unimplemented!()
        }
        pub fn zsh() {
            unimplemented!()
        }
        pub fn fish() {
            let script = "\
            function fish_prompt\n    \
                command prettyline --show-lprompt --exit_status $status\n\
            end\n\
            function fish_right_prompt\n    \
                command prettyline --show-lprompt\n\
            end\
            ";
            println!("{}", script);
        }
    }
}

pub mod prompt {
    use anstyle::{Color, Style};
    use std::fmt::Display;

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

    #[derive(Debug)]
    pub enum TextWeight {
        Bold,
        Dimm,
    }

    #[derive(Debug, Default)]
    pub struct Chunk<'a> {
        pub value: &'a str,
        pub weight: Option<TextWeight>,
        pub fg_color: Option<Color>,
        pub bg_color: Option<Color>,
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
        pub fn fg(mut self, color: Color) -> Self {
            self.fg_color = Some(color);
            self
        }
        pub fn bg(mut self, color: Color) -> Self {
            self.bg_color = Some(color);
            self
        }
        pub fn weight(mut self, weight: TextWeight) -> Self {
            self.weight = Some(weight);
            self
        }
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
