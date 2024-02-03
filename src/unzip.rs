use std::fs::File;
use std::path::{Path};
use std::{fs, io};
use zip::ZipArchive;

pub struct Unzip;

impl Unzip {
    ///
    /// unzip file from a file path in the format of &str
    ///
    /// # Example
    ///
    /// ```
    ///use simple_zip::unzip::Unzip;
    ///
    ///let path = "./a.zip";
    ///Unzip::local_str(&path);
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
    /// use simple_zip::unzip::Unzip;
    ///
    ///let path = "./a.zip";
    ///let pathbuf = Path::new(&path);
    ///Unzip::local_buffer(&pathbuf);
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
