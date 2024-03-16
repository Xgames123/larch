use std::{
    fs,
    io::{self, Read},
    process,
};

use clap::Parser;
use emulator::Emulator;
use libmcc::U4;

mod emulator;

#[derive(Parser)]
#[command(author, version)]
#[command(propagate_version = true)]
#[command(about = "Emulator for for my custom Minecraft computer", long_about = None)]
struct Cli {
    ///file to load in memory or - to read stdin
    #[arg(default_value = "-")]
    input: String,
    #[arg(short = 's', long = "step")]
    step: bool,
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
fn from_bin_packed(data: Vec<u8>) -> [U4; 256] {
    let mut out = [U4::B0000; 256];
    let mut count = 0;
    for byte in data.into_iter() {
        let lower = byte >> 4 & 0x0F;
        let upper = byte & 0x0F;
        out[count] = lower.into();
        out[count + 1] = upper.into();
        count += 2;
    }

    out
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
    if input_data.len() != 128 {
        die("Input data was not the size of the memory (128 bytes). Are you sure your data isn't in ubin format?")
    }

    let memory = from_bin_packed(input_data);
    let mut emulator = Emulator::new(memory);

    emulator.start();

    loop {
        if !emulator.is_running {
            break;
        }
        let instruct = emulator.tick();
        if cli.step {
            if let Some(instruct) = instruct {
                println!("instruct: {:#04x} {:?}", instruct.into_u4(), instruct);
            }
            println!("ip: {:#04x}", emulator.ip());
            println!("dp: {:#04x}", emulator.dp());
            println!("stack: {:#04x}", emulator.stack_peek());
            println!("");
            println!("Press return key to step forward");
            let mut buf = String::new();
            std::io::stdin().read_line(&mut buf).unwrap();
        }
    }
}
