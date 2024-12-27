extern crate flate2;

use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;

fn main() {
    let mut env_args = env::args();
    env_args.next();

    let source = match env_args.next() {
        Some(s) => s,
        None => {
            eprintln!("provide path to file");
            return;
        }
    };

    let target = match env_args.next() {
        Some(s) => s,
        None => {
            eprintln!("provide filename for compressed file");
            return;
        }
    };

    compress_file(&source, &target);
    // decompress_file(&source, &target);
}

fn compress_file(source: &String, target: &String) {
    let mut e = GzEncoder::new(File::create(target).unwrap(), Compression::default());

    let buf = fs::read(source).unwrap();

    e.write_all(&buf).unwrap();
    e.finish().unwrap();

    println!("file compressed");
}

fn decompress_file(source: &String, target: &String) {
    let buf = fs::read(source).unwrap();

    let mut d = GzDecoder::new(&buf[..]);

    let mut df = File::create(target).unwrap();

    let mut s = Vec::new();
    d.read_to_end(&mut s).unwrap();

    df.write(&s).unwrap();

    println!("file de-compressed");
}
