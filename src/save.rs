use ulid::Ulid;

#[derive(Debug, Clone)]
pub struct Save {
    id: String,
    document_id: String,
    url: Option<String>,
}

impl Save {
    pub fn new(document_id: &str) -> Self {
        Self {
            id: Ulid::new().to_string(),
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
