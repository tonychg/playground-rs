use crate::{
    document::Document,
    downloader::Downloader,
    repository::{document::DocumentRepository, save::SaveRepository},
    save::Save,
    storage::Storage,
};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub trait DocumentService {
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
            if storage.exists(&document)? {
                let save = storage.upload(&document)?;
                self.save_repository.save(&save)?;
                return Ok(save);
            }
        }
        Err("No storage available".into())
    }
}
