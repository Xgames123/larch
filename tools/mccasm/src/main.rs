use clap::{Parser, ValueEnum};
use std::{
    fs,
    io::{self, Read},
    path::Path,
    process,
};

use crate::emiting::*;
use libmcc::{bobbin_bits::U4, InstructionSet};
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
    #[arg(short = 'o')]
    output: String,

    /// Remove symbols from output
    #[arg(short = 's')]
    strip: bool,

    /// The format of the output
    #[arg(short = 'f', default_value = "auto")]
    format: Format,
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
    let output_file = Path::new(&cli.output);

    let input_data = get_input_data(&cli.input).unwrap_or_else(|err: io::Error| {
        die(&format!(
            "Failed to read input '{}'\n{}",
            cli.input,
            &err.to_string()
        ));
        String::new()
    });

    let out = match asm::assemble(input_data, InstructionSet::V2) {
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
        cli.strip,
    );

    fs::write(cli.output, content).unwrap_or_else(|err| {
        die(&format!("Failed to write output file\n\n {}", err));
    });
}
