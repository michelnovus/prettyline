// [MIT License] Copyright (c) 2024 Michel Novus

use anstyle::Color;
use anyhow::{anyhow, Result};
use chrono::Local;
use clap::Parser;
use prettyline::{
    constants,
    constants::{colors, symbols},
    misc, prompt, setup,
};
use std::{
    env,
    io::{self, prelude::*},
};

fn main() -> Result<()> {
    let args = setup::Args::parse();

    if args.init {
        return match args.shell {
            setup::ShellName::Bash => {
                println!("export VIRTUAL_ENV_DISABLE_PROMPT=1");
                setup::install::bash();
                Ok(())
            }
            setup::ShellName::Zsh => {
                println!("export VIRTUAL_ENV_DISABLE_PROMPT=1");
                setup::install::zsh();
                Ok(())
            }
            setup::ShellName::Fish => {
                println!("set --export VIRTUAL_ENV_DISABLE_PROMPT 1");
                setup::install::fish();
                Ok(())
            }
        };
    }

    let terminal_background_color = misc::get_term_bg_color()
        .unwrap_or_else(|| Color::Ansi(anstyle::AnsiColor::Black));

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
                    .bg(terminal_background_color),
                _ => prompt::Chunk::new(symbols::R_ANGLED_FILL)
                    .fg(colors::USER_NORM_BG)
                    .bg(terminal_background_color),
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
                    .fg(terminal_background_color)
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
        let left_chunk = |color: Color| {
            prompt::Chunk::new(constants::symbols::L_CURVED_FILL).fg(color)
        };
        let right_chunk = |color: Color| {
            prompt::Chunk::new(constants::symbols::R_CURVED_FILL).fg(color)
        };

        if env::var_os("VIRTUAL_ENV").is_some() {
            let virtualenv_python = prompt::Segment {
                left: Some(left_chunk(constants::colors::VENV_PYTHON_BG)),
                center: prompt::Chunk::new("\u{E235}")
                    .bg(constants::colors::VENV_PYTHON_BG)
                    .fg(constants::colors::VENV_PYTHON_FG)
                    .weight(prompt::TextWeight::Bold),
                right: Some(right_chunk(constants::colors::VENV_PYTHON_BG)),
            };
            segments.push(virtualenv_python);
        }

        let current_time: String = Local::now().format("%H:%M").to_string();
        let time = prompt::Segment {
            left: Some(left_chunk(constants::colors::TIME_BG)),
            center: prompt::Chunk::new(&current_time)
                .bg(constants::colors::TIME_BG)
                .fg(constants::colors::TIME_FG)
                .weight(prompt::TextWeight::Dimm),
            right: Some(right_chunk(constants::colors::TIME_BG)),
        };
        segments.push(time);

        segments
            .iter()
            .map(|segment| format!(" {}", segment.to_string()))
            .collect::<String>()
    };

    let (left_prompt, right_prompt) = match args.shell {
        setup::ShellName::Bash => (
            misc::ansi_escape_wrapper(&left_prompt, "\\[", "\\]"),
            misc::ansi_escape_wrapper(&right_prompt, "\\[", "\\]"),
        ),
        setup::ShellName::Zsh => (
            misc::ansi_escape_wrapper(&left_prompt, "%{", "%}"),
            misc::ansi_escape_wrapper(&right_prompt, "%{", "%}"),
        ),
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
