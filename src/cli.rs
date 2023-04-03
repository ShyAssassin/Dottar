use std::path::PathBuf;
use clap::{Parser, Subcommand, Command, ValueHint};
use clap_complete::{generate, Generator, Shell};


pub fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut std::io::stdout());
}


#[derive(Debug, Subcommand, PartialEq)]
pub enum CliSubcommand {
    #[command(about = "Initialize a new dotfile repository")]
    Init {
        #[arg(long, help = "Force initialization even if a repository already exists")]
        force: bool,
    },

    #[command(about = "Add a new dotfile")]
    Add {
        #[arg(help = "Path to the dotfile")]
        file: PathBuf,
    },

    #[command(about = "Remove a dotfile")]
    Remove {
        #[arg(help = "Path to the dotfile")]
        file: PathBuf,
    },

    #[command(about = "Edit a dotfile with your default editor")]
    Edit {
        #[arg(help = "Path to the dotfile")]
        path: PathBuf,
        
        #[arg(value_hint = ValueHint::CommandName, help = "Override editor specified in config file and environment variables")]
        editor: Option<String>,
    },

    #[command(alias = "ls", about = "List all dotfiles")]
    List,

    #[command(about = "Generate shell completions")]
    Generate {
        #[arg(value_enum, help = "Shell to generate completions for")]
        generator: Shell,
    }
}


#[derive(Debug, Parser, PartialEq)]
pub struct CliOpts {
    #[arg(short, long, help = "Enable verbose output")]
    pub verbose: bool,
}


#[derive(Debug, Parser, PartialEq)]
#[command(name = "Dotarr")]
#[command(version = "0.1.0")]
#[command(bin_name = "dotarr")]
#[command(author = "ShyAssassin")]
#[command(about = "A simple CLI tool to manage your dotfiles")]
pub struct Cli {
    #[command(flatten)]
    pub opts: CliOpts,

    #[command(subcommand)]
    pub command: CliSubcommand,
}