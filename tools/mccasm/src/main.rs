use clap::Parser;
use std::{
    fs,
    io::{self, Read},
    process,
};

mod asm;

#[derive(Parser)]
#[command(author, version)]
#[command(propagate_version = true)]
#[command(about = "Assembler for for the mcc assembly language", long_about = None)]
struct Cli {
    /// Input file or - to read from stdin
    #[arg(default_value = "-")]
    input: String,

    /// Output file
    #[arg(short = 'o')]
    output: String,

    /// Remove # comments form the output file
    #[arg(short = 's')]
    strip: bool,
}

fn get_input_data(path: &str) -> io::Result<String> {
    if path == "-" {
        let mut str = String::new();
        let mut stdin = io::stdin();
        stdin.read_to_string(&mut str)?;
        Ok(str)
    } else {
        let str = fs::read_to_string(&path)?;
        Ok(str)
    }
}

fn die(message: &str) {
    eprintln!("FATAL: {}", message);
    process::exit(-1);
}
fn main() {
    let cli = Cli::parse();
    let input_data = get_input_data(&cli.input).unwrap_or_else(|err: io::Error| {
        die(&format!(
            "Failed to read input '{}'\n{}",
            cli.input,
            &err.to_string()
        ));
        String::new()
    });

    asm::assemble(&input_data);
}
