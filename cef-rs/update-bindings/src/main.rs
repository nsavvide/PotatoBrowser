#![doc = include_str!("../README.md")]

#[macro_use]
extern crate thiserror;

use clap::Parser;
use download_cef::DEFAULT_TARGET;
use std::{fs, io::Read, path::Path};

#[derive(Debug, Error)]
pub enum Error {
    #[error("Missing Parent")]
    MissingParent(std::path::PathBuf),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Bindgen(#[from] bindgen::BindgenError),
    #[error(transparent)]
    Regex(#[from] regex::Error),
    #[error(transparent)]
    Syn(#[from] syn::Error),
    #[error("Parsing bindgen output failed")]
    Parse(#[from] parse_tree::Unrecognized),
    #[error("Missing Path")]
    MissingPath(std::path::PathBuf),
}

pub type Result<T> = std::result::Result<T, Error>;

mod dirs;
mod parse_tree;
mod upgrade;

#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    #[arg(short, long)]
    download: bool,
    #[arg(short, long)]
    bindgen: bool,
    target: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let target = args.target.as_deref().unwrap_or(DEFAULT_TARGET);

    if args.bindgen {
        if args.download {
            let _ = upgrade::download(target);
        }

        upgrade::sys_bindgen(target)?;
    }

    let bindings_file = upgrade::get_target_bindings(target);
    let mut sys_bindings = dirs::get_sys_dir()?;
    sys_bindings.push("src");
    sys_bindings.push("bindings");
    sys_bindings.push(&bindings_file);
    let mut cef_bindings = dirs::get_cef_dir()?;
    cef_bindings.push("src");
    cef_bindings.push("bindings");
    cef_bindings.push(&bindings_file);

    let bindings = parse_tree::generate_bindings(&sys_bindings)?;
    let source = read_bindings(&bindings)?;
    let dest = read_bindings(&cef_bindings).unwrap_or_default();

    if source != dest {
        fs::copy(&bindings, &cef_bindings)?;
        println!("Updated: {}", cef_bindings.display());
    }

    Ok(())
}

fn read_bindings(source_path: &Path) -> crate::Result<String> {
    let mut source_file = fs::File::open(source_path)?;
    let mut updated = String::default();
    source_file.read_to_string(&mut updated)?;
    Ok(updated)
}
