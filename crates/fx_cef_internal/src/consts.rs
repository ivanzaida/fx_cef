use std::env::var;
use std::path::PathBuf;
use anyhow::Result;

const CEF_ARTIFACTS_DIR: &str = "CEF_ARTIFACTS_DIR";

pub fn get_cef_artifacts_dir() -> Result<PathBuf> {
    let artifacts_dir = var(CEF_ARTIFACTS_DIR)?;
    Ok(PathBuf::from(artifacts_dir))
}

pub fn get_cef_artifacts_name(version: &str) -> Result<String> {
    Ok(format!("cef_binary_{}_windows64_minimal", version))
}