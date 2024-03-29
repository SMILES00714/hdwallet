//! Module implementing the `hex` subcommand for encoding and decoding
//! hexadecimal strings.
//!
//! This subcommand exists because of the lack of an "easy" POSIX-compatible
//! way of decoding hex.

use crate::cmd;
use anyhow::Result;
use clap::Parser;
use std::{
    io::{self, Write},
    path::PathBuf,
    str,
};

#[derive(Debug, Parser)]
pub struct Options {
    #[clap(subcommand)]
    op: Op,
}

#[derive(Debug, Parser)]
enum Op {
    /// Encode binary data as a hexadecimal string.
    Encode {
        /// Path to the data to encode. Use `-` for standard in.
        #[clap(name = "DATA", default_value = "-")]
        data: PathBuf,
    },

    /// Decode a hexadecimal string as binary data.
    Decode {
        /// Path to the data to decode. Use `-` for standard in.
        #[clap(name = "DATA", default_value = "-")]
        data: PathBuf,
    },
}

pub fn run(options: Options) -> Result<()> {
    match options.op {
        Op::Encode { data } => {
            let data = cmd::read_input(&data)?;
            println!("0x{}", hex::encode(data));
        }
        Op::Decode { data } => {
            let data = cmd::read_input(&data)?;
            let bytes = cmd::permissive_hex(str::from_utf8(&data)?)?;
            io::stdout().write_all(&bytes)?;
        }
    }
    Ok(())
}
