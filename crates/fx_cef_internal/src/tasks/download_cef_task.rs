use std::fs::{create_dir, create_dir_all, read_dir,  remove_dir_all, remove_file, File};
use std::{fs, io};
use std::io::Cursor;
use std::path::{Path, PathBuf};
use anyhow::Result;
use reqwest::blocking::get;
use crate::get_cef_artifacts_name;
use bzip2::read::BzDecoder;
use tar::{Archive, };

pub struct DownloadCefTask {
    pub version: String,
    pub artifacts_dir: PathBuf
}

impl DownloadCefTask {
    pub fn run(&self) -> Result<()> {
        if self.artifacts_dir.exists() {
            remove_dir_all(&self.artifacts_dir)?;
        }

        create_dir_all(&self.artifacts_dir)?;

        let url = self.get_cef_url()?;
        let out_dir = &self.artifacts_dir;

        let archive = Self::download(&url, &out_dir)?;
        Self::extract_bz2(&archive, &out_dir)?;
        remove_file(&archive)?;

        let contents = out_dir.join(get_cef_artifacts_name(&self.version)?);

        Self::copy_recursive(&contents, &out_dir)?;
        remove_dir_all(&contents)?;

        Ok(())
    }

    fn get_cef_url(&self) -> Result<String> {
        let file_name = get_cef_artifacts_name(&self.version)?;
        Ok(format!(
            "https://cef-builds.spotifycdn.com/{}.tar.bz2",
            file_name
        ))
    }

    fn download(url: &str, out_dir: &PathBuf) -> Result<PathBuf> {
        let out = out_dir.join("cef.tar.bz2");
        let response = get(url)?;
        let mut file = File::create(&out)?;
        let mut content = Cursor::new(response.bytes()?);
        io::copy(&mut content, &mut file)?;
        Ok(out)
    }

    fn extract_bz2(file: &Path, dir: &Path) -> Result<()> {
        let file = File::open(file)?;
        let decoder = BzDecoder::new(file);
        let mut archive = Archive::new(decoder);
        archive.unpack(dir)?;
        Ok(())
    }

    fn copy_recursive(src: &Path, dst: &Path) -> Result<()> {
        for entry in read_dir(src)? {
            let entry = entry?;
            let file_type = entry.file_type()?;

            if file_type.is_file() {
                let dst = dst.join(entry.file_name());

                if dst.exists() {
                    remove_file(&dst)?;
                }

                fs::copy(entry.path(), dst)?;
            } else if file_type.is_dir() {
                let dst = dst.join(entry.file_name());

                if dst.exists() {
                    remove_dir_all(&dst)?;
                }

                create_dir(&dst)?;
                Self::copy_recursive(&entry.path(), &dst)?;
            }
        }

        Ok(())
    }

}