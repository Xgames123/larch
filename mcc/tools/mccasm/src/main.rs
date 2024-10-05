use clap::{Parser, ValueEnum};
use std::{
    fs,
    io::{self, Read},
    path::Path,
    process,
};
use stderrlog::LogLevelNum;

use crate::{emiting::*, util::count_nonzero_pages};
mod asm;
mod emiting;
mod util;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    /// Input file or - to read from stdin
    #[arg(default_value = "-")]
    input: String,

    /// Output file
    #[arg(short = 'o', long)]
    output: String,

    /// The format of the output
    #[arg(short = 'f', long, default_value = "auto")]
    format: Format,

    /// Prints the amount of space the program uses
    #[arg(short = 'm', long)]
    memory_usage: bool,

    /// The minimum log level
    #[arg(long, default_value = "0")]
    log_level: usize,
}

#[derive(Debug, Clone, ValueEnum, PartialEq, Eq)]
#[value()]
enum Format {
    ///Choose format based on file extension
    Auto,
    Hex,
    Bin,
    ///Unpacked binary format (every nibble is byte aligned)
    Ubin,
}
fn get_input_data(path: &str) -> io::Result<String> {
    if path == "-" {
        let mut str = String::new();
        let mut stdin = io::stdin();
        stdin.read_to_string(&mut str)?;
        Ok(str)
    } else {
        let can_path = std::fs::canonicalize(&path)?;
        let str = fs::read_to_string(can_path)?;
        Ok(str)
    }
}

fn die(message: &str) {
    eprintln!("FATAL: {}", message);
    process::exit(-1);
}
fn main() {
    let cli = Cli::parse();
    stderrlog::new()
        .verbosity(LogLevelNum::from(cli.log_level))
        .module(module_path!())
        .show_module_names(true)
        .init()
        .unwrap();

    let output_file = Path::new(&cli.output);

    let input_data = get_input_data(&cli.input).unwrap_or_else(|err: io::Error| {
        die(&format!(
            "Failed to read input '{}'\n{}",
            cli.input,
            &err.to_string()
        ));
        String::new()
    });

    let out = match asm::assemble(input_data) {
        Ok(out) => out,
        Err(err) => {
            die(&err.to_string());
            return;
        }
    };

    let content = emit(
        cli.format,
        output_file.extension().map(|ext| ext.to_str()).flatten(),
        out,
    );

    fs::write(cli.output, content).unwrap_or_else(|err| {
        die(&format!("Failed to write output file\n\n {}", err));
    });

    if cli.memory_usage {
        println!("Using {}/16 pages", count_nonzero_pages(&out));
    }
}
