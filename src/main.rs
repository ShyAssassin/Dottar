mod cli;
mod util;
mod config;
use std::env;
use std::path::{Path};
use std::fs;
use std::fs::{copy, create_dir_all};
use clap::{Parser, CommandFactory};

fn main() {
    let home_dir = env::var("HOME").unwrap();
    let dotfiles_dir = Path::new(&home_dir).join(".dotfiles");
    let current_dir = env::current_dir().unwrap();
    fs::create_dir(&dotfiles_dir).unwrap_or_default();
    println!("{}", home_dir); 
    println!("{}", dotfiles_dir.display());
    let args = cli::Cli::parse();



    match args.command {
        cli::CliSubcommand::Init { force } => {
            println!("Initializing dotfile repository... force: {force:?}");
        }

        cli::CliSubcommand::Add { file } => {
            // create full path to file / dir
            let full_path = current_dir.join(&file);
            if full_path.is_file(){
                // get file path relative to home dir
                let dest_path = full_path.strip_prefix(&home_dir).unwrap();
                // create parent dir if it doesn't exist
                let parent_dir = dotfiles_dir.join(dest_path.parent().unwrap());
                create_dir_all(parent_dir).unwrap();
                // copy file to dotfiles dir
                copy(&full_path, dotfiles_dir.join(dest_path)).unwrap();
            } 
            else if full_path.is_dir() {
                // get file path relative to home dir
                let dest_path = full_path.strip_prefix(&home_dir).unwrap();
                util::copy_recursively(&full_path,  dotfiles_dir.join(dest_path)).expect("Failed to copy directory");
            }
            else {
                println!("Cannot find file or directory!")
            }
        }

        cli::CliSubcommand::Remove { file } => {
            println!("Removing dotfile {file:?}...");
        }

        cli::CliSubcommand::Edit { path, editor } => {
            println!("Editing dotfile {path:?} with editor {editor:?}...");
        }

        cli::CliSubcommand::List => {
            println!("Listing dotfiles...");
        }

        cli::CliSubcommand::Generate { generator } => {
            let mut cmd = cli::Cli::command();
            println!("Generating completion file for {generator:?}...");
            cli::print_completions(generator, &mut cmd);
        }
    }
}
