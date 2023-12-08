pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub trait Repository<T> {
    fn find(&self, id: &str) -> Result<T>;
    fn save(&mut self, item: &T) -> Result<()>;
    fn delete(&mut self, id: &str) -> Result<()>;
}

#[cfg(test)]
mod tests {
    use crate::document::Document;
    use std::collections::HashMap;

    use super::*;

    pub struct MockRepository<T> {
        items: HashMap<String, T>,
    }

    impl MockRepository<Document> {
        fn new() -> Self {
            Self {
                items: HashMap::new(),
            }
        }
    }

    impl Repository<Document> for MockRepository<Document> {
        fn find(&self, id: &str) -> Result<Document> {
            match self.items.get(id) {
                Some(document) => Ok(document.clone()),
                None => Err("Not found".into()),
            }
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

    #[test]
    fn test_repository_find_with_not_existing_document() {
        let repository = MockRepository::new();
        let document = Document::new("http://example.com");
        assert_eq!(repository.find(&document.id()).is_err(), true);
    }

    #[test]
    fn test_repository_find_with_existing_document() {
        let mut repository = MockRepository::new();
        let document = Document::new("http://example.com");
        repository.save(&document).unwrap();
        assert_eq!(repository.find(&document.id()).is_ok(), true);
    }

    #[test]
    fn test_repository_save_with_new_document() {
        let mut repository = MockRepository::new();
        let document = Document::new("http://example.com");
        repository.save(&document).unwrap();
        assert_eq!(repository.find(&document.id()).is_ok(), true);
    }

    #[test]
    fn test_repository_delete_with_existing_document() {
        let mut repository = MockRepository::new();
        let document = Document::new("http://example.com");
        repository.save(&document).unwrap();
        repository.delete(&document.id()).unwrap();
        assert_eq!(repository.find(&document.id()).is_err(), true);
    }
}
