// [MIT License] Copyright (c) 2024 Michel Novus

use anyhow::{anyhow, Result};
use clap::Parser;

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

    Ok(())
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
