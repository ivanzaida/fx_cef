use fx_cef_internal::{get_cef_artifacts_dir, GenerateBindingsTask};

fn main() -> anyhow::Result<()> {
    let task = GenerateBindingsTask {
        artifacts_dir: get_cef_artifacts_dir()?,
    };
    task.run()?;
    Ok(())
}