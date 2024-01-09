use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    ZipError(#[from] zip::result::ZipError),
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

pub struct Archive {
    path: PathBuf,
}

impl Archive {
    pub fn new<P: Into<PathBuf>>(path: P) -> Self {
        Archive { path: path.into() }
    }

    pub fn as_path(&self) -> &Path {
        self.path.as_path()
    }

    pub fn create<P: AsRef<Path>>(&self, paths: &[P]) -> Result<(), Error> {
        let file = File::create(self.as_path())?;
        let mut zip = zip::ZipWriter::new(file);
        for path in paths {
            zip.start_file(
                path.as_ref().display().to_string(),
                zip::write::FileOptions::default(),
            )?;
            zip.write_all(&std::fs::read(&path)?)?;
        }
        Ok(())
    }

    pub fn extract<P: AsRef<Path>>(&self, destination: P) -> Result<(), Error> {
        let file = File::open(self.as_path())?;
        let mut archive = zip::ZipArchive::new(file)?;
        Ok(archive.extract(destination)?)
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use tempfile::tempdir;

    use super::*;

    #[test]
    fn test_archive_new_from_path() {
        let path = Path::new("test.zip");
        let archive = Archive::new(path);
        assert_eq!(archive.as_path(), path);
    }

    #[test]
    fn test_archive_new_from_str() {
        let archive = Archive::new("test.zip");
        assert_eq!(archive.as_path(), Path::new("test.zip"));
    }

    #[test]
    fn test_archive_create() {
        let target = tempdir().unwrap();
        let dest = tempdir().unwrap();
        let file_path = target.path().join("test01.txt");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "test01").unwrap();
        let archive = Archive::new(dest.path().join("test.zip"));
        let paths = vec![file_path];
        archive.create(&paths).unwrap();
    }

    #[test]
    fn test_archive_extract() {
        todo!()
    }
}
