use std::fs::{create_dir_all, File};
use std::io;
use std::io::BufReader;
use std::path::Path;

use flate2::read::GzDecoder;
use tar::Archive;
use xz2::read::XzDecoder;
use zip::ZipArchive;

pub fn uncompress_archive(file: &Path, extract_dir: &Path, ext: &str) -> anyhow::Result<()> {
    match ext {
        "zip" => {
            unzip(file, extract_dir)?;
        }
        "tar" => {
            untar(file, extract_dir)?;
        }
        "tar.gz" | "tgz" => {
            untar_gz(file, extract_dir)?;
        }
        "tar.xz" => {
            untar_xz(file, extract_dir)?;
        }
        _ => {
            return Err(anyhow::anyhow!("Unsupported archive format: {}", ext));
        }
    }

    Ok(())
}

fn untar_xz(file: &Path, extract_dir: &Path) -> anyhow::Result<()> {
    let tar_xz = File::open(file)?;
    let tar = XzDecoder::new(BufReader::new(tar_xz));
    let mut archive = Archive::new(tar);

    Ok(archive.unpack(extract_dir)?)
}

fn untar_gz(file: &Path, extract_dir: &Path) -> anyhow::Result<()> {
    let tar_gz = File::open(file)?;
    let tar = GzDecoder::new(BufReader::new(tar_gz));
    let mut archive = Archive::new(tar);

    Ok(archive.unpack(extract_dir)?)
}

fn untar(file: &Path, extract_dir: &Path) -> anyhow::Result<()> {
    let tar_file = File::open(file)?;
    let tar = BufReader::new(tar_file);
    let mut archive = Archive::new(tar);

    Ok(archive.unpack(extract_dir)?)
}

fn unzip(file: &Path, extract_dir: &Path) -> anyhow::Result<()> {
    let file = File::open(file)?;
    let mut archive = ZipArchive::new(BufReader::new(file))?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = match file.enclosed_name() {
            Some(path) => extract_dir.join(path),
            None => continue,
        };

        if file.name().ends_with('/') {
            create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    create_dir_all(p)?;
                }
            }
            let mut outfile = File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
        }
    }

    Ok(())
}
