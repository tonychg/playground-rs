use crate::{document::Document, save::Save};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub trait Storage {
    fn exists(&self, document: &Document) -> Result<bool>;
    fn upload(&mut self, document: &Document) -> Result<Save>;
}

pub struct FakeStorage {
    directory: String,
    cache: Vec<Save>,
}

impl FakeStorage {
    pub fn new(directory: &str) -> Self {
        Self {
            directory: directory.to_string(),
            cache: Vec::new(),
        }
    }
}

impl Storage for FakeStorage {
    fn exists(&self, document: &Document) -> Result<bool> {
        Ok(self
            .cache
            .iter()
            .any(|save| save.document_id() == document.id()))
    }

    fn upload(&mut self, document: &Document) -> Result<Save> {
        let mut save = Save::new(&document.id());
        save.set_url(format!("{}/{}", self.directory, save.id()));
        self.cache.push(save.clone());
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
        let mut storage = FakeStorage::new("/tmp");
        let document = Document::new("http://example.com");
        storage.upload(&document).unwrap();
        assert_eq!(storage.exists(&document).unwrap(), true);
    }

    #[test]
    fn test_storage_upload_with_valid_document() {
        let mut storage = FakeStorage::new("/tmp");
        let document = Document::new("http://example.com");
        let save = storage.upload(&document).unwrap();
        assert_eq!(save.document_id(), document.id());
        assert_eq!(save.url().unwrap(), format!("/tmp/{}", save.id()));
    }
}
