// [MIT License] Copyright (c) 2024 Michel Novus

use anyhow::{anyhow, Result};
use clap::Parser;

fn main() -> Result<()> {
    let args = setup::Args::parse();

    Ok(())
}

pub mod setup {
    use clap::{Parser, ValueEnum};

    #[derive(Debug, Parser)]
    pub struct Args {
        /// Sets shell settings.
        #[arg(long, value_name = "SHELL")]
        pub init: Option<ShellName>,
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
                    command prettyline\n\
                end\n\
                function fish_right_prompt\n    \
                command prettyline\n\
                end\n
                ";
            println!("{}", script);
            unimplemented!();
        }
    }
}
