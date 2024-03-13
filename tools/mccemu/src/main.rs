use std::{
    fs,
    io::{self, Read},
    process,
};

use clap::Parser;
use emulator::Emulator;
use libmcc::Memory;

mod emulator;

#[derive(Parser)]
#[command(author, version)]
#[command(propagate_version = true)]
#[command(about = "Emulator for for my custom Minecraft computer", long_about = None)]
struct Cli {
    ///file to load in memory or - to read stdin
    #[arg(default_value = "-")]
    input: String,
}

fn get_input_data(path: &str) -> io::Result<Vec<u8>> {
    if path == "-" {
        let mut buf = Vec::new();
        let mut stdin = io::stdin();
        stdin.read_to_end(&mut buf)?;
        Ok(buf)
    } else {
        let can_path = std::fs::canonicalize(&path)?;
        let vec = fs::read(can_path)?;
        Ok(vec)
    }
}
fn die(message: &str) {
    eprintln!("FATAL: {}", message);
    process::exit(-1);
}
fn from_bin_packed(data: Vec<u8>) -> Memory {
    let mut out = Vec::new();
    let mut current = Vec::new();
    for byte in data.into_iter() {
        let lower = byte & 0xF0;
        let upper = byte & 0x0F >> 4;
        current.push(lower);
        current.push(upper);
        if current.len() == 16 {
            out.push(
                TryInto::<[u8; 16]>::try_into(current.clone())
                    .unwrap()
                    .into(),
            );
            current.clear()
        }
    }

    out.try_into().unwrap()
}

fn main() {
    let cli = Cli::parse();
    let input_data = get_input_data(&cli.input).unwrap_or_else(|err: io::Error| {
        die(&format!(
            "Failed to read input '{}'\n{}",
            cli.input,
            &err.to_string()
        ));
        Vec::new()
    });
    let memory = from_bin_packed(input_data);
    let mut emulator = Emulator::new(memory);

    emulator.start();

    loop {
        emulator.tick();
    }
}
