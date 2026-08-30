use anyhow::Result;
use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Archive file to list
    file: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let file = fs::File::open(&args.file)?;

    if args.file.ends_with(".zip") || args.file.ends_with(".jar") {
        list_zip(file)?;
    } else {
        list_tar(file)?;
    }
    Ok(())
}

fn list_tar(file: std::fs::File) -> Result<()> {
    let mut archive = tar::Archive::new(file);
    for entry in archive.entries()? {
        let entry = entry?;
        let path = entry.path()?.display().to_string();
        let size = entry.size();
        println!("{:>10}  {}", size, path);
    }
    Ok(())
}

fn list_zip(file: std::fs::File) -> Result<()> {
    let mut archive = zip::ZipArchive::new(file)?;
    for i in 0..archive.len() {
        let entry = archive.by_index(i)?;
        println!("{:>10}  {}", entry.size(), entry.name());
    }
    Ok(())
}
