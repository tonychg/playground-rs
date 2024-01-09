use std::sync::Arc;
use ulid::Ulid;

fn new_ulid_as_str() -> String {
    Ulid::new().to_string()
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Id(Arc<str>);

impl Id {
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }

    pub fn new() -> Id {
        Id(new_ulid_as_str().into())
    }
}

impl From<&str> for Id {
    fn from(value: &str) -> Self {
        Id(Arc::from(value))
    }
}

pub type ItemId = Id;

#[derive(Clone, Debug)]
pub struct Item {
    id: ItemId,
    name: String,
}

impl Item {
    pub fn new(id: &str, name: &str) -> Item {
        Item {
            id: id.into(),
            name: name.to_string(),
        }
    }

    pub fn from_name(name: &str) -> Item {
        Self::new(Id::new().as_str(), name)
    }

    pub fn default() -> Item {
        Item {
            id: "0000000-00000-0000-0000".into(),
            name: "default".into(),
        }
    }

    pub fn id(&self) -> &str {
        self.id.as_str()
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_item_default() {
        let item = Item::default();
        assert_eq!(item.id(), "0000000-00000-0000-0000");
        assert_eq!(item.name(), "default");
    }

    #[test]
    fn test_create_item() {
        let item = Item::new("1234567-12345-1234-1234", "test");
        assert_eq!(item.id(), "1234567-12345-1234-1234");
        assert_eq!(item.name(), "test");
    }

    #[test]
    fn test_create_item_from_name() {
        let item = Item::from_name("test");
        assert_eq!(item.name(), "test");
    }
}
