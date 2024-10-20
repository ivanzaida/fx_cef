use std::env::var;
use std::fs;
use std::path::{Path, PathBuf};

pub struct GenerateBindingsTask {
    pub artifacts_dir: PathBuf
}

impl GenerateBindingsTask {
    pub fn run(&self) -> anyhow::Result<()> {
        let include_dir = self.artifacts_dir.join("include");
        let capi_include_dir = include_dir.join("capi");
        let bindings_dir = PathBuf::from(var("CARGO_MANIFEST_DIR")?).join("..").join("fx_cef").join("src").join("ffi");

        let includes = ["cef_sandbox_win.h"].iter().map(|header| {
            include_dir.join(header)
        }).collect::<Vec<PathBuf>>();


        let bindings = bindgen::Builder::default()
            .raw_line("#![allow(non_snake_case)]")
            .raw_line("#![allow(non_camel_case_types)]")
            .raw_line("#![allow(dead_code)]")
            .raw_line("#![allow(unused_imports)]")
            .raw_line("#![allow(unused_variables)]")
            .allowlist_type("_?cef_.*")
            .allowlist_function("_?cef_.*")
            .allowlist_var("_?cef_.*")
            .size_t_is_usize(true)
            .clang_arg(format!("-I{}", self.artifacts_dir.to_str().unwrap())) // Include `include/capi` dir
            .headers(Self::collect_header_files(&capi_include_dir).iter().map(|path| path.to_str().unwrap()))
            .headers(includes.iter().map(|path| path.to_str().unwrap()))
            .generate()
            .expect("Unable to generate bindings");

        bindings.write_to_file(bindings_dir.join("bindings.rs"))?;

        Ok(())
    }

    fn collect_header_files(dir: &Path) -> Vec<PathBuf> {
        let mut header_files = Vec::new();
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                header_files.extend(Self::collect_header_files(&path));
            } else if path.extension().and_then(|ext| ext.to_str()) == Some("h") {
                // Skip headers that are test-related (you can customize this filter as needed)
                if path.to_string_lossy().contains("test") || path.to_string_lossy().contains("unittest") {
                    continue;
                }
                header_files.push(path);
            }
        }
        header_files
    }
}