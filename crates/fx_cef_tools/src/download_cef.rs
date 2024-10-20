use anyhow::Result;
use fx_cef_internal::{get_cef_artifacts_dir, DownloadCefTask};


fn main() -> Result<()> {
    let task = DownloadCefTask {
        version: String::from("130.1.2+g48f3ef6+chromium-130.0.6723.44"),
        artifacts_dir: get_cef_artifacts_dir()?,
    };

    task.run()?;
    Ok(())
}

