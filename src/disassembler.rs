//! Module containing the disassembler implementation.

use crate::instruction::Instruction;
use anyhow::Result;
use ethnum::U256;
use std::{
    io::{self, BufReader, Read},
    str,
};

/// Disassembler implementation that reads hex-encoded EVM instructions from an
/// input stream.
pub struct Disassembler<R> {
    input: BufReader<R>,
    offset: usize,
    unknown: bool,
}

impl<R> Disassembler<R>
where
    R: Read,
{
    /// Creates a new disassembler for the specified input stream.
    pub fn new(input: R) -> Self {
        Self {
            input: BufReader::new(input),
            offset: 0,
            unknown: false,
        }
    }

    /// Reads hex-encoded string from the input stream.
    ///
    /// Note that the disassembler accepts very permissive hex encoding that
    /// ignores whitespace characters.
    fn read<'a>(&mut self, buf: &'a mut [u8]) -> Result<(), (usize, io::Error)> {
        assert!(buf.len() % 2 == 0, "reading off number of hex characters");
        self.offset += buf.len() / 2;
        for i in 0..buf.len() {
            loop {
                self.input
                    .read_exact(&mut buf[i..i + 1])
                    .map_err(|err| (i, err))?;
                if !buf[i].is_ascii_whitespace() {
                    break;
                }
            }
        }

        Ok(())
    }

    /// Reads the next byte from the input stream.
    fn next_byte(&mut self) -> Result<Option<u8>> {
        let mut buf = [0; 2];
        match self.read(&mut buf) {
            Ok(_) => (),
            Err((0, err)) if err.kind() == io::ErrorKind::UnexpectedEof => return Ok(None),
            Err((_, err)) => return Err(err.into()),
        };
        Ok(Some(u8::from_str_radix(str::from_utf8(&buf)?, 16)?))
    }

    /// Reads the next word from the input stream of the specified size.
    fn next_word(&mut self, size: u8) -> Result<U256> {
        let mut bytes = [0; 64];
        let buf = {
            let len = (size * 2) as usize;
            &mut bytes[..len]
        };
        self.read(buf).map_err(|(_, err)| err)?;
        Ok(U256::from_str_radix(str::from_utf8(buf)?, 16)?)
    }

    /// Reads the next instruction from the input stream.
    pub fn next_instruction(&mut self) -> Result<Option<Instruction>> {
        use Instruction::*;

        let op = match self.next_byte()? {
            Some(op) => op,
            None => return Ok(None),
        };

        // NOTE: Once we hit one unknown op code, then there is no way to know
        // if the remaining input contains valid instructions or is just binary
        // data, so only return `Unknown` once we hit a single unknown opcode.
        if self.unknown {
            return Ok(Some(Unknown(op)));
        }

        Ok(Some(match op {
            0x00 => Stop,
            0x01 => Add,
            0x02 => Mul,
            0x03 => Sub,
            0x04 => Div,
            0x05 => Sdiv,
            0x06 => Mod,
            0x07 => Smod,
            0x08 => AddMod,
            0x09 => MulMod,
            0x0a => Exp,
            0x0b => SignExtend,
            0x10 => Lt,
            0x11 => Gt,
            0x12 => Slt,
            0x13 => Sgt,
            0x14 => Eq,
            0x15 => IsZero,
            0x16 => And,
            0x17 => Or,
            0x18 => Xor,
            0x19 => Not,
            0x1a => Byte,
            0x1b => Shl,
            0x1c => Shr,
            0x1d => Sar,
            0x20 => Keccak256,
            0x30 => Address,
            0x31 => Balance,
            0x32 => Origin,
            0x33 => Caller,
            0x34 => CallValue,
            0x35 => CallDataLoad,
            0x36 => CallDataSize,
            0x37 => CallDataCopy,
            0x38 => CodeSize,
            0x39 => CodeCopy,
            0x3a => GasPrice,
            0x3b => ExtCodeSize,
            0x3c => ExtCodeCopy,
            0x3d => ReturnDataSize,
            0x3e => ReturnDataCopy,
            0x3f => ExtCodeHash,
            0x40 => BlockHash,
            0x41 => Coinbase,
            0x42 => Timestamp,
            0x43 => Number,
            0x44 => Difficulty,
            0x45 => GasLimit,
            0x46 => ChainId,
            0x50 => Pop,
            0x51 => MLoad,
            0x52 => MStore,
            0x53 => MStore8,
            0x54 => SLoad,
            0x55 => SStore,
            0x56 => Jump,
            0x57 => JumpI,
            0x58 => GetPc,
            0x59 => MSize,
            0x5a => Gas,
            0x5b => JumpDest(self.offset - 1),
            0x60..=0x7f => {
                let size = op - 0x5f;
                Push(size, self.next_word(size)?)
            }
            0x80..=0x8f => Dup(op - 0x7f),
            0x90..=0x9f => Swap(op - 0x8f),
            0xa0..=0xa4 => Log(op - 0xa0),
            0xf0 => Create,
            0xf1 => Call,
            0xf2 => CallCode,
            0xf3 => Return,
            0xf4 => DelegateCall,
            0xf5 => Create2,
            0xfa => StaticCall,
            0xfd => Revert,
            0xfe => Invalid,
            0xff => SelfDestruct,
            _ => {
                self.unknown = true;
                Unknown(op)
            }
        }))
    }
}
