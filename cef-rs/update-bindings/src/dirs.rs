use std::{convert::From, env, path::PathBuf};

pub fn get_manifest_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

pub fn get_out_dir() -> PathBuf {
    PathBuf::from(env!("OUT_DIR"))
}

pub fn get_cef_root(os: &str, arch: &str) -> PathBuf {
    env::var(format!("CEF_PATH_{os}_{arch}"))
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let mut out_dir = get_out_dir();
            out_dir.push(format!("cef_{os}_{arch}"));
            out_dir
        })
}

pub fn get_sys_dir() -> crate::Result<PathBuf> {
    let manifest_dir = get_manifest_dir();
    let mut bindings_dir = get_manifest_dir().parent().map_or_else(
        || Err(crate::Error::MissingParent(manifest_dir)),
        |parent| Ok(PathBuf::from(parent)),
    )?;
    bindings_dir.push("sys");
    Ok(bindings_dir)
}

pub fn get_cef_dir() -> crate::Result<PathBuf> {
    let manifest_dir = get_manifest_dir();
    let mut webview2_com_dir = get_manifest_dir().parent().map_or_else(
        || Err(crate::Error::MissingParent(manifest_dir)),
        |parent| Ok(PathBuf::from(parent)),
    )?;
    webview2_com_dir.push("cef");
    Ok(webview2_com_dir)
}
