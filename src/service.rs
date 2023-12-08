use crate::{
    document::Document,
    downloader::Downloader,
    repository::{document::DocumentRepository, save::SaveRepository},
    save::Save,
    storage::Storage,
};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub trait DocumentService {
    fn download(&self, url: &str) -> Result<Document>;
    fn store(&self, document: &Document) -> Result<Save>;
}

pub struct LocalDocumentService {
    document_repository: Box<dyn DocumentRepository>,
    save_repository: Box<dyn SaveRepository>,
    downloaders: Vec<Box<dyn Downloader>>,
    storages: Vec<Box<dyn Storage>>,
}

impl LocalDocumentService {
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

impl DocumentService for LocalDocumentService {
    fn download(&self, url: &str) -> Result<Document> {
        todo!()
    }

    fn store(&self, document: &Document) -> Result<Save> {
        todo!()
    }
}
