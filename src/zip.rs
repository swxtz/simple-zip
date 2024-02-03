use std::fs::File;
use std::path::{Path};
use std::{fs, io};
use zip::ZipArchive;
use std::io::{BufReader, copy};
use std::time::Instant;
use flate2::write::GzEncoder;
use flate2::Compression;

pub struct Decompress;
pub struct Compress;
impl Decompress {
    ///
    /// unzip file from a file path in the format of &str
    ///
    /// # Example
    ///
    /// ```
    ///use simple_zip::zip::Decompress;
    ///
    ///let path = "./a.zip";
    ///Decompress::local_str(&path);
    /// ```
    ///
    pub fn local_str(filepath: &str) {
        let filename = Path::new(filepath);
        let file = File::open(&filename).unwrap();
        let mut archive = ZipArchive::new(file).unwrap();

        for i in 0..archive.len() {
            let mut file = archive.by_index(i).unwrap();

            let outpath = match file.enclosed_name() {
                Some(path) => path.to_owned(),
                None => continue,
            };

            {
                let comment = file.comment();
                if !comment.is_empty() {
                    println!("File: {}, comment: {}", i, comment);
                }
            }

            if (*file.name()).ends_with("/") {
                fs::create_dir_all(&outpath).unwrap();
            } else {
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        fs::create_dir_all(&p).unwrap();
                    }
                }

                let mut outfile = File::create(&outpath).unwrap();
                io::copy(&mut file, &mut outfile).unwrap();
            }

            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;

                if let Some(mode) = file.unix_mode() {
                    fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
                }
            }
        }
    }

    ///
    /// unzip file from a PathBuffer
    ///
    /// # Example
    ///
    /// ```
    ///use std::path::Path;
    ///use simple_zip::zip::Decompress;
    ///
    ///let path = "./a.zip";
    ///let pathbuf = Path::new(&path);
    ///Decompress::local_buffer(&pathbuf);
    /// ```
    ///

    pub fn local_buffer(filepath: &Path) {
        let file = File::open(&filepath).unwrap();
        let mut archive = ZipArchive::new(file).unwrap();

        for i in 0..archive.len() {
            let mut file = archive.by_index(i).unwrap();

            let outpath = match file.enclosed_name() {
                Some(path) => path.to_owned(),
                None => continue,
            };

            {
                let comment = file.comment();
                if !comment.is_empty() {
                    println!("File: {}, comment: {}", i, comment);
                }
            }

            if (*file.name()).ends_with("/") {
                fs::create_dir_all(&outpath).unwrap();
            } else {
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        fs::create_dir_all(&p).unwrap();
                    }
                }

                let mut outfile = File::create(&outpath).unwrap();
                io::copy(&mut file, &mut outfile).unwrap();
            }

            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;

                if let Some(mode) = file.unix_mode() {
                    fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
                }
            }
        }
    }
}

impl Compress {



    pub fn zip(path: &Path, output: &Path) {
        let mut input = BufReader::new(File::open(path).unwrap());
        let output = File::create(output).unwrap();
        let mut encoder = GzEncoder::new(output, Compression::default());

        let start = Instant::now();
        copy(&mut input, &mut encoder).unwrap();
        let output = encoder.finish().unwrap();

        println!(
            "Source len: {:?}",
            input.get_ref().metadata().unwrap().len()
        );

        println!("Target len: {:?}", output.metadata().unwrap().len());
        println!("Elapsed: {:?}", start.elapsed());
    }
}
