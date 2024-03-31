use std::{
    fs,
    io::{self, Read},
    process,
};

use clap::Parser;
use emulator::Emulator;
use libmcc::{u4, v3::Instruction};

mod emulator;
mod ext;

#[derive(Parser)]
#[command(author, version)]
#[command(propagate_version = true)]
#[command(about = "Emulator for for my custom Minecraft computer", long_about = None)]
struct Cli {
    ///File to load in memory or - to read stdin
    #[arg(default_value = "-")]
    input: String,

    ///Step through the execution
    #[arg(short = 's', long)]
    step: bool,

    ///Step through the execution when reaching a nop instruction
    #[arg(short = 'b', long)]
    nop_break: bool,

    #[arg(short = 'x', long)]
    ext: Vec<ext::ExtType>,

    ///Print the top of the stack when the vm exits
    #[arg(short = 'p', long)]
    print: bool,
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
fn from_bin_packed(data: Vec<u8>) -> [u4; 256] {
    let mut out = [u4::ZERO; 256];
    let mut count = 0;
    for byte in data.into_iter() {
        out[count] = u4::from_low(byte);
        out[count + 1] = u4::from_high(byte);
        count += 2;
    }

    out
}

fn main() {
    let mut cli = Cli::parse();
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

    let extmgr = ext::ExtManager::new(cli.ext);
    let mut emulator = Emulator::new(memory, move |addr, value, write, emulator| {
        if write {
            extmgr.on_mem_write(addr, value, emulator);
            None
        } else {
            extmgr.on_mem_read(addr, emulator)
        }
    });

    emulator.start();

    let mut last_was_nop = false;
    loop {
        if !emulator.is_running {
            break;
        }
        let instruct = emulator.tick();
        if instruct == Some(Instruction::Nop) {
            if !last_was_nop && cli.nop_break {
                cli.step = true;
            }
            last_was_nop = true;
        } else {
            last_was_nop = false;
        }

        if cli.step {
            println!("VM BREAK");
            if let Some(instruct) = instruct {
                println!("instruct: {:#03x} {:?}", instruct.into_u4(), instruct);
            }
            println!("ip: {:#04x}", emulator.ip());
            println!("dp: {:#04x}", emulator.dp());
            println!("stack {:#03x}:", emulator.sp().into_low());
            for i in
                (emulator::STACK_START + 1)..emulator::STACK_START + emulator.sp().into_low() + 1
            {
                println!("   {:#03x}", emulator.read_mem(i));
            }
            println!("Press return to step forward or r to continue execution");
            let mut buf = String::new();
            std::io::stdin().read_line(&mut buf).unwrap();
            if buf.to_lowercase().trim() == "r" {
                cli.step = false;
            }
        }
    }
    if cli.print {
        println!("VM EXIT");
        println!("stack top was {:#03x}", emulator.stack_pop());
    }
}
