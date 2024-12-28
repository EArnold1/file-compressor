extern crate flate2;

use file_compressor::Compressor;
use std::{env, process};

fn main() {
    let mut env_args = env::args();
    env_args.next();

    let compressor = Compressor::process(env_args).unwrap_or_else(|err| {
        eprintln!("Problem parsing args: {}", err);
        process::exit(1)
    });

    if compressor.flag == "-c" {
        compressor.compress_file();
    } else if compressor.flag == "-dc" {
        compressor.decompress_file();
    } else {
        eprintln!("invalid syntax")
    }
}
