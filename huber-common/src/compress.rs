use std::fs::File;
use std::io;
use std::io::BufReader;
use std::path::Path;

use flate2::read::GzDecoder;
use tar::Archive;
use xz2::read::XzDecoder;
use zip::ZipArchive;

use crate::fs::set_executable_permission;

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
        "gz" => {
            ungz(file, extract_dir)?;
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

    let mut reader = ZipArchive::new(BufReader::new(file))?;
    reader.extract(extract_dir)?;

    Ok(())
}

fn ungz(file: &Path, extract_dir: &Path) -> anyhow::Result<()> {
    let gz = File::open(file)?;
    let mut gz = GzDecoder::new(BufReader::new(gz));

    let outfile_path = extract_dir.join(
        file.file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .replace(".gz", ""),
    );
    let mut out_file = File::create(&outfile_path)?;
    io::copy(&mut gz, &mut out_file)?;
    set_executable_permission(&outfile_path)?;

    Ok(())
}
