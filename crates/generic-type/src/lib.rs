use std::fmt::{self, Display, Formatter};
use std::string::ToString;
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Id(Arc<str>);

impl Id {
    pub fn new<T: ToString>(id: T) -> Self {
        Self(id.to_string().into())
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq<Id> for &str {
    fn eq(&self, other: &Id) -> bool {
        self == &other.0.as_ref()
    }
}

impl PartialEq<Id> for String {
    fn eq(&self, other: &Id) -> bool {
        self == other.0.as_ref()
    }
}

impl AsRef<str> for Id {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

#[test]
fn arc_str_as_ref() {
    assert_eq!("123", Id::new("123").as_ref());
}

#[test]
fn arc_str_equals_str() {
    assert_eq!("123", Id::new("123"));
}

#[test]
fn arc_str_not_equals_str() {
    assert_ne!("321", Id::new("123"));
}

#[test]
fn arc_str_equals_string() {
    assert_eq!("123".to_string(), Id::new("123".to_string()));
}

#[test]
fn arc_str_not_equals_string() {
    assert_ne!("321".to_string(), Id::new("123".to_string()));
}

#[test]
fn arc_str_equals_uuid() {
    let uuid = uuid::Uuid::new_v4();
    assert_eq!(uuid.to_string(), Id::new(uuid));
}

#[test]
fn arc_str_not_equals_uuid() {
    assert_ne!(
        uuid::Uuid::new_v4().to_string(),
        Id::new(uuid::Uuid::new_v4())
    );
}
