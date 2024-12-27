use flate2::read::{GzDecoder, ZlibDecoder};
use flate2::write::{GzEncoder, ZlibEncoder};
use flate2::Compression;
use std::io::{copy, prelude::*};
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

    pub fn gz_compress_file(&self) {
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

    pub fn gz_decompress_file(&self) {
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

    pub fn decompress_file(&self) {
        let file = fs::File::open(&self.source).unwrap();

        //using the archive reader function
        let mut archive = zip::ZipArchive::new(file).unwrap();

        for i in 0..archive.len() {
            let mut file = archive.by_index(i).unwrap();

            // gets the buffer path
            let target_path = match file.enclosed_name() {
                Some(path) => path,
                None => continue,
            };

            if file.name().ends_with('/') {
                // create folder if file is a folder and has contents
                fs::create_dir_all(&target_path).unwrap();
            } else if !file.name().contains("/") {
                let mut outfile = fs::File::create(&target_path).unwrap();
                write_to_file(&mut file, &mut outfile);
            } else {
                if let Some(p) = target_path.parent() {
                    if !p.exists() {
                        println!("again");
                        fs::create_dir_all(&p).unwrap();
                    }
                }

                let mut outfile = fs::File::create(&target_path).unwrap();

                write_to_file(&mut file, &mut outfile);
            }

            // Get and Set permissions for the extracted files
            // #[cfg(unix)]
            // {
            //     use std::os::unix::fs::PermissionsExt;

            //     if let Some(mode) = file.unix_mode() {
            //         fs::set_permissions(&target_path, fs::Permissions::from_mode(mode)).unwrap();
            //     }
            // }
        }

        println!("file decompression done");
    }
}

fn write_to_file<T: Sized + std::io::Read>(reader: &mut T, writer: &mut File) {
    copy(reader, writer).unwrap();
}
