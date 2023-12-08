#[cfg(test)]
mod tests {
    use crate::document::*;

    pub struct MockDownloader {
        directory: String,
    }

    impl MockDownloader {
        fn new(directory: &str) -> Self {
            Self {
                directory: directory.to_string(),
            }
        }
    }

    impl Downloader for MockDownloader {
        fn is_available(&self, _document: &Document) -> Result<bool> {
            Ok(true)
        }

        fn get_filename(&self, document: &Document) -> Result<Document> {
            let mut document = document.clone();
            document.set_filename(format!("{}.mp4", document.id()));
            Ok(document)
        }

        fn download(&self, document: &Document) -> Result<Document> {
            let mut document = self.get_filename(document)?;
            document.set_path(format!(
                "{}/{}",
                self.directory,
                document.filename().unwrap()
            ));
            Ok(document)
        }
    }

    pub struct MockStorage {
        directory: String,
    }

    impl MockStorage {
        fn new(directory: &str) -> Self {
            Self {
                directory: directory.to_string(),
            }
        }
    }

    impl Storage for MockStorage {
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

    #[test]
    fn test_downloader_download_with_valid_document() {
        let downloader = MockDownloader::new("/tmp");
        let document = Document::new("http://example.com");
        let document = downloader.download(&document).unwrap();
        assert_eq!(
            document.path().unwrap(),
            format!("/tmp/{}.mp4", document.id())
        );
    }

    #[test]
    fn test_storage_exists_with_not_existing_document() {
        let storage = MockStorage::new("/tmp");
        let document = Document::new("http://example.com");
        assert_eq!(storage.exists(&document).unwrap(), false);
    }

    #[test]
    fn test_storage_exists_with_existing_document() {
        let storage = MockStorage::new("/tmp");
        let mut document = Document::new("http://example.com");
        document.set_filename("existing".to_string());
        storage.upload(&document).unwrap();
        assert_eq!(storage.exists(&document).unwrap(), true);
    }

    #[test]
    fn test_storage_upload_with_valid_document() {
        let storage = MockStorage::new("/tmp");
        let document = Document::new("http://example.com");
        let save = storage.upload(&document).unwrap();
        assert_eq!(save.document_id(), document.id());
        assert_eq!(save.url().unwrap(), format!("/tmp/{}", save.id()));
    }
}
