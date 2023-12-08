use crate::{
    document::Document,
    downloader::Downloader,
    repository::{document::DocumentRepository, save::SaveRepository},
    save::Save,
    storage::Storage,
};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub trait DocumentService {
    fn get_document_by_id(&self, document_id: &str) -> Result<Option<Document>>;
    fn get_document_by_url(&self, url: &str) -> Result<Option<Document>>;
    fn download(&mut self, url: &str) -> Result<Document>;
    fn store(&mut self, document_id: &str) -> Result<Save>;
}

pub struct DocumentServiceFactory {
    document_repository: Box<dyn DocumentRepository>,
    save_repository: Box<dyn SaveRepository>,
    downloaders: Vec<Box<dyn Downloader>>,
    storages: Vec<Box<dyn Storage>>,
}

impl DocumentServiceFactory {
    pub fn new(
        document_repository: Box<dyn DocumentRepository>,
        save_repository: Box<dyn SaveRepository>,
        downloaders: Vec<Box<dyn Downloader>>,
        storages: Vec<Box<dyn Storage>>,
    ) -> Self {
        Self {
            document_repository,
            save_repository,
            downloaders,
            storages,
        }
    }
}

impl DocumentService for DocumentServiceFactory {
    fn get_document_by_id(&self, document_id: &str) -> Result<Option<Document>> {
        Ok(self.document_repository.find(document_id)?)
    }

    fn get_document_by_url(&self, url: &str) -> Result<Option<Document>> {
        Ok(self.document_repository.find_by_url(url)?)
    }

    fn download(&mut self, url: &str) -> Result<Document> {
        let document = self.document_repository.find_by_url(url)?;
        if document.is_some() {
            return Err("Document already exists".into());
        }
        let mut document = Document::new(url);
        self.document_repository.save(&document)?;
        for downloader in self.downloaders.iter() {
            if downloader.is_available(&document)? {
                document = downloader.download(&document)?;
                self.document_repository.save(&document)?;
                return Ok(document);
            }
        }
        Err("No downloader available".into())
    }

    fn store(&mut self, document_id: &str) -> Result<Save> {
        let document = self.document_repository.find(document_id)?;
        if document.is_none() {
            return Err("Document not found".into());
        }
        let document = document.unwrap();
        for storage in self.storages.iter() {
            if !storage.exists(&document)? {
                let save = storage.upload(&document)?;
                self.save_repository.save(&save)?;
                return Ok(save);
            } else {
                return Err("Document already stored".into());
            }
        }
        Err("No storage available".into())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        downloader::FakeDownloader,
        repository::{document::DocumentMemoryRepository, save::SaveMemoryRepository},
        storage::FakeStorage,
    };

    use super::*;

    fn create_fake_document_service() -> DocumentServiceFactory {
        let document_repository = Box::new(DocumentMemoryRepository::new());
        let save_repository = Box::new(SaveMemoryRepository::new());
        let downloaders: Vec<Box<dyn Downloader>> = vec![Box::new(FakeDownloader::new("/tmp"))];
        let storages: Vec<Box<dyn Storage>> = vec![Box::new(FakeStorage::new("/tmp"))];
        DocumentServiceFactory::new(document_repository, save_repository, downloaders, storages)
    }

    #[test]
    fn test_document_service_download_with_valid_url() {
        let mut document_service = create_fake_document_service();
        let document_downloaded = document_service.download("https://example.com").unwrap();
        let document = document_service
            .get_document_by_id(&document_downloaded.id())
            .unwrap()
            .unwrap();
        assert_eq!(document.url(), "https://example.com");
        assert_eq!(
            document.filename(),
            Some(format!("{}.mp4", document.id().to_string()))
        );
    }

    #[test]
    fn test_document_service_download_with_invalid_url() {
        let mut document_service = create_fake_document_service();
        let document_downloaded = document_service.download("https://example.org");
        assert!(document_downloaded.is_err());
    }

    #[test]
    fn test_document_service_download_with_already_existing_document() {
        let mut document_service = create_fake_document_service();
        document_service.download("https://example.com").unwrap();
        let document_downloaded = document_service.download("https://example.com");
        assert_eq!(
            document_downloaded.unwrap_err().to_string(),
            "Document already exists"
        );
    }

    #[test]
    fn test_document_service_get_document_by_url_with_valid_document() {
        let mut document_service = create_fake_document_service();
        let document_downloaded = document_service.download("https://example.com").unwrap();
        let document = document_service
            .get_document_by_url("https://example.com")
            .unwrap()
            .unwrap();
        assert_eq!(document_downloaded.id(), document.id());
        assert_eq!(document_downloaded.url(), document.url());
    }

    #[test]
    fn test_document_service_store_with_valid_document() {
        let mut document_service = create_fake_document_service();
        let document_downloaded = document_service.download("https://example.com").unwrap();
        let save = document_service.store(&document_downloaded.id()).unwrap();
        assert_eq!(save.document_id(), document_downloaded.id());
    }

    #[test]
    fn test_document_service_store_with_invalid_document() {
        let mut document_service = create_fake_document_service();
        let save = document_service.store("invalid_document_id");
        assert_eq!(save.unwrap_err().to_string(), "Document not found");
    }

    #[test]
    fn test_document_service_store_with_already_stored_document() {
        let mut document_service = create_fake_document_service();
        let document_downloaded = document_service
            .download("https://example.com/existing")
            .unwrap();
        let save = document_service.store(&document_downloaded.id());
        assert_eq!(save.unwrap_err().to_string(), "Document already stored");
    }
}
