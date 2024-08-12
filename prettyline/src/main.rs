// [MIT License] Copyright (c) 2024 Michel Novus

use anyhow::{anyhow, Result};
use clap::Parser;

fn main() -> Result<()> {
    let args = setup::Args::parse();
    println!("{:#?}", args);

    Ok(())
}

pub mod setup {
    use clap::{ArgGroup, Parser, ValueEnum};

    #[derive(Debug, Parser)]
    #[command(
        group = ArgGroup::new("init-conflict")
            .arg("show_lprompt")
            .arg("show_lprompt")
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
