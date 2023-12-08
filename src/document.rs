use ulid::Ulid;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn generate_id() -> String {
    Ulid::new().to_string()
}

#[derive(Debug, Clone)]
pub struct Document {
    id: String,
    url: String,
    filename: Option<String>,
    path: Option<String>,
}

impl Document {
    pub fn new(url: &str) -> Self {
        Self {
            id: generate_id(),
            url: url.to_string(),
            filename: None,
            path: None,
        }
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn url(&self) -> String {
        self.url.clone()
    }

    pub fn filename(&self) -> Option<String> {
        self.filename.clone()
    }

    pub fn set_filename(&mut self, filename: String) {
        self.filename = Some(filename);
    }

    pub fn path(&self) -> Option<String> {
        self.path.clone()
    }

    pub fn set_path(&mut self, path: String) {
        self.path = Some(path);
    }
}

#[derive(Debug, Clone)]
pub struct Save {
    id: String,
    document_id: String,
    url: Option<String>,
}

impl Save {
    pub fn new(document_id: &str) -> Self {
        Self {
            id: generate_id(),
            document_id: document_id.to_string(),
            url: None,
        }
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn url(&self) -> Option<String> {
        self.url.clone()
    }

    pub fn set_url(&mut self, url: String) {
        self.url = Some(url);
    }

    pub fn document_id(&self) -> String {
        self.document_id.clone()
    }
}

pub trait Downloader {
    fn is_available(&self, document: &Document) -> Result<bool>;
    fn get_filename(&self, document: &Document) -> Result<Document>;
    fn download(&self, document: &Document) -> Result<Document>;
}

pub trait Storage {
    fn exists(&self, document: &Document) -> Result<bool>;
    fn upload(&self, document: &Document) -> Result<Save>;
}

#[cfg(test)]
mod tests {
    const YOUTUBE_URL: &str = "https://www.youtube.com/watch?v=6v2L2UGZJAM";

    #[test]
    fn create_new_video() {
        let video = super::Document::new(YOUTUBE_URL);
        assert_eq!(video.url(), YOUTUBE_URL.to_string());
        assert_eq!(video.filename(), None);
    }
}
