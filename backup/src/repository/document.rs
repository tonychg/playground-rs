use std::collections::HashMap;

use crate::document::Document;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub trait DocumentRepository {
    fn find(&self, id: &str) -> Result<Option<Document>>;
    fn find_by_url(&self, url: &str) -> Result<Option<Document>>;
    fn save(&mut self, item: &Document) -> Result<()>;
    fn delete(&mut self, id: &str) -> Result<()>;
}

pub struct DocumentMemoryRepository {
    items: HashMap<String, Document>,
}

impl DocumentMemoryRepository {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }
}

impl DocumentRepository for DocumentMemoryRepository {
    fn find(&self, id: &str) -> Result<Option<Document>> {
        match self.items.get(id) {
            Some(document) => Ok(Some(document.clone())),
            None => Ok(None),
        }
    }

    fn find_by_url(&self, url: &str) -> Result<Option<Document>> {
        for (_, document) in &self.items {
            if document.url() == url {
                return Ok(Some(document.clone()));
            }
        }
        Ok(None)
    }

    fn save(&mut self, item: &Document) -> Result<()> {
        self.items.insert(item.id(), item.clone());
        Ok(())
    }

    fn delete(&mut self, id: &str) -> Result<()> {
        self.items.remove(id);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::document::Document;

    use super::*;

    #[test]
    fn test_repository_find_with_not_existing_document() {
        let repository = DocumentMemoryRepository::new();
        let document = Document::new("http://example.com");
        assert_eq!(repository.find(&document.id()).unwrap().is_none(), true);
    }

    #[test]
    fn test_repository_find_with_existing_document() {
        let mut repository = DocumentMemoryRepository::new();
        let document = Document::new("http://example.com");
        repository.save(&document).unwrap();
        assert_eq!(repository.find(&document.id()).unwrap().is_some(), true);
    }

    #[test]
    fn test_repository_find_by_url_with_existing_document() {
        let mut repository = DocumentMemoryRepository::new();
        let document = Document::new("http://example.com");
        repository.save(&document).unwrap();
        assert_eq!(
            repository.find_by_url(&document.url()).unwrap().is_some(),
            true
        );
    }

    #[test]
    fn test_repository_save_with_new_document() {
        let mut repository = DocumentMemoryRepository::new();
        let document = Document::new("http://example.com");
        repository.save(&document).unwrap();
        assert_eq!(repository.find(&document.id()).unwrap().is_some(), true);
    }

    #[test]
    fn test_repository_delete_with_existing_document() {
        let mut repository = DocumentMemoryRepository::new();
        let document = Document::new("http://example.com");
        repository.save(&document).unwrap();
        repository.delete(&document.id()).unwrap();
        assert_eq!(repository.find(&document.id()).unwrap().is_none(), true);
    }
}
