use crate::{document::Document, save::Save};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub trait Storage {
    fn exists(&self, document: &Document) -> Result<bool>;
    fn upload(&self, document: &Document) -> Result<Save>;
}

pub struct FakeStorage {
    directory: String,
}

impl FakeStorage {
    pub fn new(directory: &str) -> Self {
        Self {
            directory: directory.to_string(),
        }
    }
}

impl Storage for FakeStorage {
    fn exists(&self, document: &Document) -> Result<bool> {
        match document.filename() {
            Some(filename) => match filename.as_str() {
                "existing" => Ok(true),
                _ => Ok(false),
            },
            None => Ok(false),
        }
    }

    fn upload(&self, document: &Document) -> Result<Save> {
        let mut save = Save::new(&document.id());
        save.set_url(format!("{}/{}", self.directory, save.id()));
        Ok(save)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_exists_with_not_existing_document() {
        let storage = FakeStorage::new("/tmp");
        let document = Document::new("http://example.com");
        assert_eq!(storage.exists(&document).unwrap(), false);
    }

    #[test]
    fn test_storage_exists_with_existing_document() {
        let storage = FakeStorage::new("/tmp");
        let mut document = Document::new("http://example.com");
        document.set_filename("existing".to_string());
        storage.upload(&document).unwrap();
        assert_eq!(storage.exists(&document).unwrap(), true);
    }

    #[test]
    fn test_storage_upload_with_valid_document() {
        let storage = FakeStorage::new("/tmp");
        let document = Document::new("http://example.com");
        let save = storage.upload(&document).unwrap();
        assert_eq!(save.document_id(), document.id());
        assert_eq!(save.url().unwrap(), format!("/tmp/{}", save.id()));
    }
}
