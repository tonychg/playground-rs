use crate::document::Document;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub trait Downloader {
    fn is_available(&self, document: &Document) -> Result<bool>;
    fn get_filename(&self, document: &Document) -> Result<Document>;
    fn download(&self, document: &Document) -> Result<Document>;
}

pub struct FakeDownloader {
    directory: String,
}

impl FakeDownloader {
    fn new(directory: &str) -> Self {
        Self {
            directory: directory.to_string(),
        }
    }
}

impl Downloader for FakeDownloader {
    fn is_available(&self, document: &Document) -> Result<bool> {
        Ok(document.url().starts_with("https://example.com"))
    }

    fn get_filename(&self, document: &Document) -> Result<Document> {
        let mut document = document.clone();
        document.set_filename(format!("{}.mp4", document.id()));
        Ok(document)
    }

    fn download(&self, document: &Document) -> Result<Document> {
        let mut document = self.get_filename(document)?;
        document.set_local_path(format!(
            "{}/{}",
            self.directory,
            document.filename().unwrap()
        ));
        Ok(document)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_downloader_is_available_with_valid_document() {
        let downloader = FakeDownloader::new("/tmp");
        let document = Document::new("https://example.com");
        assert_eq!(downloader.is_available(&document).unwrap(), true);
    }

    #[test]
    fn test_downloader_download_with_valid_document() {
        let downloader = FakeDownloader::new("/tmp");
        let document = Document::new("http://example.com");
        let document = downloader.download(&document).unwrap();
        assert_eq!(
            document.local_path().unwrap(),
            format!("/tmp/{}.mp4", document.id())
        );
    }
}
