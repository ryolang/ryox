use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod ast;
mod builtins;
mod codegen;
mod errors;
mod hir;
mod indent;
mod lexer;
mod linker;
mod lower;
mod parser;
mod pipeline;

#[derive(Parser)]
#[command(name = "ryo")]
#[command(about = "The Ryo programming language compiler")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Tokenize a Ryo source file and print the token stream
    Lex {
        /// Input file to tokenize
        file: PathBuf,
    },
    /// Parse a Ryo source file and print the AST
    Parse {
        /// Input file to parse
        file: PathBuf,
    },
    /// Generate and display Cranelift IR for a Ryo program
    Ir {
        /// Input file to generate IR for
        file: PathBuf,
    },
    /// Compile and run a Ryo program (JIT)
    Run {
        /// Input file to compile and run
        file: PathBuf,
    },
    /// Compile a Ryo program to a standalone binary (AOT)
    Build {
        /// Input file to compile
        file: PathBuf,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Lex { file } => pipeline::lex_command(&file)?,
        Commands::Parse { file } => pipeline::parse_command(&file)?,
        Commands::Ir { file } => pipeline::ir_command(&file)?,
        Commands::Run { file } => pipeline::run_file(&file)?,
        Commands::Build { file } => pipeline::build_file(&file)?,
    }

    Ok(())
}
