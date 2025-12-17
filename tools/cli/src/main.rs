// SPDX-License-Identifier: MIT OR GPL-3.0-or-later OR Palimpsest-0.8
// SPDX-FileCopyrightText: 2025 Julia the Viper Contributors
//
// Julia the Viper - Command Line Interface

// Julia the Viper - Command Line Interface
use clap::{Parser, Subcommand};
use colored::*;
use jtv_lang::{parse_program, Interpreter, TypeChecker, PurityChecker};
use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;

mod repl;
mod rsr_check;
use repl::Repl;
use rsr_check::RsrChecker;

#[derive(Parser)]
#[command(name = "jtv")]
#[command(about = "Julia the Viper - Harvard Architecture Language", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a JtV program
    Run {
        /// Path to the .jtv file (use '-' for stdin)
        file: String,

        /// Enable execution tracing
        #[arg(short, long)]
        trace: bool,

        /// Print variable values after execution
        #[arg(short, long)]
        vars: bool,
    },

    /// Parse a JtV file and display the AST
    Parse {
        /// Path to the .jtv file
        file: String,

        /// Output format (json or pretty)
        #[arg(short, long, default_value = "pretty")]
        format: String,
    },

    /// Check a JtV file for errors without executing
    Check {
        /// Path to the .jtv file
        file: String,
    },

    /// Analyze legacy code for JtV extraction opportunities
    Analyze {
        /// Path to the legacy code file
        file: String,

        /// Language (python, javascript, ruby)
        #[arg(short, long, default_value = "javascript")]
        lang: String,
    },

    /// Display version and build information
    Version,

    /// Check RSR (Rhodium Standard Repository) compliance
    RsrCheck,

    /// Start the interactive REPL
    Repl,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { file, trace, vars } => {
            if let Err(e) = run_file(&file, trace, vars) {
                eprintln!("{} {}", "Error:".red().bold(), e);
                std::process::exit(1);
            }
        }
        Commands::Parse { file, format } => {
            if let Err(e) = parse_file(&file, &format) {
                eprintln!("{} {}", "Error:".red().bold(), e);
                std::process::exit(1);
            }
        }
        Commands::Check { file } => {
            if let Err(e) = check_file(&file) {
                eprintln!("{} {}", "Error:".red().bold(), e);
                std::process::exit(1);
            } else {
                println!("{} No errors found", "✓".green().bold());
            }
        }
        Commands::Analyze { file, lang } => {
            eprintln!("{} Analyzer not yet fully implemented", "Warning:".yellow().bold());
            eprintln!("Please use: deno run --allow-read packages/jtv-analyzer/src/main.ts {} {}", file, lang);
        }
        Commands::Version => {
            print_version();
        }
        Commands::RsrCheck => {
            let mut checker = RsrChecker::new();
            checker.check_all();
        }
        Commands::Repl => {
            let mut repl = Repl::new();
            if let Err(e) = repl.run() {
                eprintln!("{} {}", "Error:".red().bold(), e);
                std::process::exit(1);
            }
        }
    }
}

fn run_file(file_path: &str, trace: bool, show_vars: bool) -> Result<(), String> {
    let code = read_file(file_path)?;

    let program = parse_program(&code).map_err(|e| format!("Parse error: {}", e))?;

    let mut interpreter = Interpreter::new();

    if trace {
        interpreter.enable_trace();
    }

    interpreter.run(&program).map_err(|e| format!("Runtime error: {}", e))?;

    if trace {
        println!("\n{}", "=== Execution Trace ===".cyan().bold());
        for entry in interpreter.get_trace() {
            println!("{}: {}", entry.stmt_type.yellow(), entry.line);
        }
    }

    if show_vars {
        println!("\n{}", "=== Variables ===".cyan().bold());
        // In a real implementation, would iterate over interpreter.globals
        println!("(Variable display not yet implemented)");
    }

    Ok(())
}

fn parse_file(file_path: &str, format: &str) -> Result<(), String> {
    let code = read_file(file_path)?;

    let program = parse_program(&code).map_err(|e| format!("Parse error: {}", e))?;

    match format {
        "json" => {
            let json = serde_json::to_string_pretty(&program).map_err(|e| e.to_string())?;
            println!("{}", json);
        }
        "pretty" | _ => {
            println!("{}", "=== Abstract Syntax Tree ===".cyan().bold());
            println!("{:#?}", program);
        }
    }

    Ok(())
}

fn check_file(file_path: &str) -> Result<(), String> {
    let code = read_file(file_path)?;

    let program = parse_program(&code).map_err(|e| format!("Parse error: {}", e))?;

    // Type checking
    let mut type_checker = TypeChecker::new();
    type_checker
        .check_program(&program)
        .map_err(|e| format!("Type error: {}", e))?;

    // Purity checking
    let mut purity_checker = PurityChecker::new();
    purity_checker
        .check_program(&program)
        .map_err(|e| format!("Purity error: {}", e))?;

    Ok(())
}

fn read_file(file_path: &str) -> Result<String, String> {
    if file_path == "-" {
        let mut buffer = String::new();
        io::stdin()
            .read_to_string(&mut buffer)
            .map_err(|e| format!("Failed to read stdin: {}", e))?;
        Ok(buffer)
    } else {
        fs::read_to_string(file_path)
            .map_err(|e| format!("Failed to read file '{}': {}", file_path, e))
    }
}

fn print_version() {
    println!("{}", "Julia the Viper".cyan().bold());
    println!("Version: {}", env!("CARGO_PKG_VERSION"));
    println!("Build: {}", "Rust");
    println!();
    println!("{}", "Harvard Architecture Language".green());
    println!("Makes code injection grammatically impossible");
    println!();
    println!("Repository: https://github.com/Hyperpolymath/julia-the-viper");
    println!("Documentation: https://docs.julia-viper.dev");
}
