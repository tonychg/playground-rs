use ulid::Ulid;

#[derive(Debug, Clone)]
pub struct Document {
    id: String,
    url: String,
    filename: Option<String>,
    local_path: Option<String>,
}

impl Document {
    pub fn new(url: &str) -> Self {
        Self {
            id: Ulid::new().to_string(),
            url: url.to_string(),
            filename: None,
            local_path: None,
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

    pub fn local_path(&self) -> Option<String> {
        self.local_path.clone()
    }

    pub fn set_local_path(&mut self, local_path: String) {
        self.local_path = Some(local_path);
    }
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
