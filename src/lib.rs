use flate2::read::{GzDecoder, ZlibDecoder};
use flate2::write::{GzEncoder, ZlibEncoder};
use flate2::Compression;
use std::io::prelude::*;
use std::{
    env,
    fs::{self, File},
    io::Write,
};
pub struct Compressor {
    pub flag: String,
    source: String,
    target: String,
}

impl Compressor {
    pub fn process(mut args: env::Args) -> Result<Compressor, &'static str> {
        let flag = match args.next() {
            Some(s) => s,
            None => {
                return Err("provide a flag, `-c` or `-dc`");
            }
        };

        let source = match args.next() {
            Some(s) => s,
            None => {
                return Err("provide path to file");
            }
        };

        let target = match args.next() {
            Some(s) => s,
            None => {
                return Err("provide filename for compressed file");
            }
        };

        Ok(Compressor {
            flag,
            source,
            target,
        })
    }

    pub fn compress_file(&self) {
        let mut e = GzEncoder::new(File::create(&self.target).unwrap(), Compression::default());

        let buf = fs::read(&self.source).unwrap();

        e.write_all(&buf).unwrap();
        e.finish().unwrap();

        println!("file compressed");
    }

    pub fn z_compress_file(&self) {
        let mut e = ZlibEncoder::new(File::create(&self.target).unwrap(), Compression::default());

        let buf = fs::read(&self.source).unwrap();

        e.write_all(&buf).unwrap();
        e.finish().unwrap();

        println!("file compressed");
    }

    pub fn decompress_file(&self) {
        let buf = fs::read(&self.source).unwrap();

        let mut d = GzDecoder::new(&buf[..]);

        let mut df = File::create(&self.target).unwrap();

        let mut s = Vec::new();
        d.read_to_end(&mut s).unwrap();

        df.write(&s).unwrap();

        println!("file de-compressed");
    }

    pub fn z_decompress_file(&self) {
        let buf = fs::read(&self.source).unwrap();

        let mut d = ZlibDecoder::new(&buf[..]);

        let mut df = File::create(&self.target).unwrap();

        let mut s = Vec::new();
        d.read_to_end(&mut s).unwrap();

        df.write(&s).unwrap();

        println!("file de-compressed");
    }
}
