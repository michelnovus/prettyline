// [MIT License] Copyright (c) 2024 Michel Novus
//! Contains things related to program configuration.

use clap::{ArgGroup, Parser, ValueEnum};

/// Expected arguments entered when starting the program.
///
/// The user only need call the `--init --shell {SHELLNAME}` argument inside
/// `eval "$(prettyline --init --shell {SHELLNAME})"`.
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

/// The function prints to stdout the necessary configuration
/// to run the program correctly.
pub fn install(shell_name: ShellName) {
    match shell_name {
        ShellName::Bash => {
            println!("{}", shell::bash());
            println!("export VIRTUAL_ENV_DISABLE_PROMPT=1")
        }
        ShellName::Fish => {
            println!("{}", shell::fish());
            println!("export VIRTUAL_ENV_DISABLE_PROMPT=1")
        }
        ShellName::Zsh => {
            println!("{}", shell::zsh());
            println!("set --export VIRTUAL_ENV_DISABLE_PROMPT 1")
        }
    }
}

/// Defines configuration of each shell in its shell language.
mod shell {
    pub fn bash() -> String {
        // TODO: falta asignar el texto derecho!
        "\
        prompt_function ()\n\
        {\n    \
            PS1=\"$(prettyline --shell bash --show-lprompt --exit-status $?)\"\n\
        }\n
        PROMPT_COMMAND=prompt_function\n\
        ".into()
    }
    pub fn zsh() -> String {
        "\
        function precmd() {\n    \
            PROMPT=\"$(prettyline --shell zsh --show-lprompt --exit-status $?)\"\n    \
            RPROMPT=\"$(prettyline --shell zsh --show-rprompt)\"\n\
        }\
        ".into()
    }
    pub fn fish() -> String {
        "\
        function fish_prompt\n    \
            command prettyline --shell fish --show-lprompt --exit-status $status\n\
        end\n\
        function fish_right_prompt\n    \
            command prettyline --shell fish --show-rprompt\n\
        end\
        ".into()
    }
}
