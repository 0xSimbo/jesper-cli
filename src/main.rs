use serde::{Deserialize, Serialize};
use std::collections::HashMap;
mod find_selector;
mod parse_sol_file;
use parse_sol_file::{parse_sol_file, SolidityMessageAndArgs};
mod constants;
use clap::{arg, command, value_parser, ArgAction, Command, Parser, Subcommand};
use std::option::Option;
mod config;
pub mod handle_parse_all_files;
mod typescript_boilerplate;
use crate::{config::generate_mode_files, handle_parse_all_files::handle_parse_all_files};
use config::{create_output_folder_if_not_exists, generate_basic_config, read_config};

#[derive(Parser, Debug)]
#[command(name = "Jesper", version = "1.0", author = "0xSimon", about = "A powerful error generator for Solidity", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Generate TypeScript errors from Solidity code
    Gen {
        // Define additional arguments/options for the Gen command if needed
    },

    /// Initialize the project with a default configuration
    Init {
        // Additional options for Init can be added here
    },
}

fn main() {
    let config = read_config();
    let all_abi_files = config.all_abi_files.clone();
    let all_solidity_files: Vec<String> = config.all_solidity_files.clone();
    create_output_folder_if_not_exists(&config);
    let cli = Cli::parse(); // Parse the command line arguments
    match cli.command {
        Some(Commands::Gen { .. }) => {
            handle_parse_all_files(all_abi_files, all_solidity_files, &config);
            generate_mode_files(&config);
            println!("Generated output");
        }
        Some(Commands::Init { .. }) => {
            generate_basic_config();
        }
        None => {
            println!("No command found. Use --help for more information.");
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
