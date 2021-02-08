mod disassembler;
mod instruction;

use crate::disassembler::Disassembler;
use anyhow::Result;
use std::{
    fs::File,
    io::{self, Read},
    path::PathBuf,
};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Options {
    /// The file containing hex-encoded EVM bytecode to disassemble.
    #[structopt(name = "FILE")]
    file: Option<PathBuf>,
}

fn main() -> Result<()> {
    let options = Options::from_args();

    let file = options.file.unwrap_or_else(|| PathBuf::from("-"));
    match file.to_str() {
        Some("-") => disassemble(io::stdin().lock()),
        _ => disassemble(File::open(file)?),
    }
}

fn disassemble(input: impl Read) -> Result<()> {
    let mut disassembler = Disassembler::new(input);
    while let Some(instruction) = disassembler.next_instruction()? {
        println!("{}", instruction);
    }

    Ok(())
}
