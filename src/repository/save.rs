use std::collections::HashMap;

use crate::save::Save;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub trait SaveRepository {
    fn find(&self, id: &str) -> Result<Option<Save>>;
    fn find_document_id(&self, url: &str) -> Result<Vec<Save>>;
    fn save(&mut self, item: &Save) -> Result<()>;
    fn delete(&mut self, id: &str) -> Result<()>;
}

pub struct SaveMemoryRepository {
    items: HashMap<String, Save>,
}

impl SaveMemoryRepository {
    fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }
}

impl SaveRepository for SaveMemoryRepository {
    fn find(&self, id: &str) -> Result<Option<Save>> {
        match self.items.get(id) {
            Some(save) => Ok(Some(save.clone())),
            None => Ok(None),
        }
    }

    fn find_document_id(&self, document_id: &str) -> Result<Vec<Save>> {
        let mut result = vec![];
        for (_, save) in &self.items {
            if save.document_id() == document_id {
                result.push(save.clone());
            }
        }
        Ok(result)
    }

    fn save(&mut self, item: &Save) -> Result<()> {
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
    use crate::save::Save;

    use super::*;

    #[test]
    fn test_repository_find_with_not_existing_save() {
        let repository = SaveMemoryRepository::new();
        let save = Save::new("test");
        assert_eq!(repository.find(&save.id()).unwrap().is_none(), true);
    }

    #[test]
    fn test_repository_find_with_existing_save() {
        let mut repository = SaveMemoryRepository::new();
        let save = Save::new("test");
        repository.save(&save).unwrap();
        assert_eq!(repository.find(&save.id()).unwrap().is_some(), true);
    }

    #[test]
    fn test_repository_save_with_new_save() {
        let mut repository = SaveMemoryRepository::new();
        let save = Save::new("test");
        repository.save(&save).unwrap();
        assert_eq!(repository.find(&save.id()).unwrap().is_some(), true);
    }

    #[test]
    fn test_repository_delete_with_existing_save() {
        let mut repository = SaveMemoryRepository::new();
        let save = Save::new("test");
        repository.save(&save).unwrap();
        repository.delete(&save.id()).unwrap();
        assert_eq!(repository.find(&save.id()).unwrap().is_none(), true);
    }
}
