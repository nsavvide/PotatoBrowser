#![doc = include_str!("../README.md")]

use clap::Parser;
use download_cef::{CefIndex, OsAndArch, DEFAULT_TARGET};
use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    #[arg(short, long)]
    force: bool,
    #[arg(short, long)]
    save_archive: bool,
    #[arg(short, long, default_value = DEFAULT_TARGET)]
    target: String,
    output: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let output = PathBuf::from(args.output);
    let parent = PathBuf::from(
        output
            .parent()
            .ok_or_else(|| anyhow::anyhow!("invalid target directory: {}", output.display()))?,
    );

    if fs::exists(&output)? {
        if !args.force {
            return Err(anyhow::anyhow!(
                "target directory already exists: {}",
                output.display()
            ));
        }

        let dir = output
            .file_name()
            .and_then(|dir| dir.to_str())
            .ok_or_else(|| anyhow::anyhow!("invalid target directory: {}", output.display()))?;
        let old_output = parent.join(format!("old_{dir}"));
        fs::rename(&output, &old_output)?;
        println!("Cleaning up: {}", old_output.display());
        fs::remove_dir_all(old_output)?
    }

    let target = args.target.as_str();
    let os_arch = OsAndArch::try_from(target)?;
    let cef_dir = os_arch.to_string();
    let cef_dir = parent.join(&cef_dir);

    if fs::exists(&cef_dir)? {
        let dir = cef_dir
            .file_name()
            .and_then(|dir| dir.to_str())
            .ok_or_else(|| anyhow::anyhow!("invalid target directory: {}", output.display()))?;
        let old_cef_dir = parent.join(format!("old_{dir}"));
        fs::rename(&cef_dir, &old_cef_dir)?;
        println!("Cleaning up: {}", old_cef_dir.display());
        fs::remove_dir_all(old_cef_dir)?
    }

    let cef_version = env!("CARGO_PKG_VERSION");
    let index = CefIndex::download()?;
    let platform = index.platform(target)?;
    let version = platform.version(cef_version)?;

    let archive = version.download_archive(&parent, true)?;
    let extracted_dir = download_cef::extract_target_archive(target, &archive, &parent, true)?;
    if extracted_dir != cef_dir {
        return Err(anyhow::anyhow!(
            "extracted dir {extracted_dir:?} does not match cef_dir {cef_dir:?}",
        ));
    }

    if !args.save_archive {
        println!("Cleaning up: {}", archive.display());
        fs::remove_file(archive)?;
    }

    {
        let archive_version = serde_json::to_string_pretty(version.minimal()?)?;
        let mut archive_json = File::create(extracted_dir.join("archive.json"))?;
        archive_json.write_all(archive_version.as_bytes())?;
    }

    if output != cef_dir {
        println!("Renaming: {}", output.display());
        fs::rename(cef_dir, output)?;
    }

    Ok(())
}
