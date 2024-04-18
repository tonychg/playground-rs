use super::{Attribute, Modifier};
use generic_type::Id;
use std::sync::Arc;

pub type ItemBaseId = Id;
pub type Durability = u32;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Category {
    Equipment,
    Consumable,
    Rune,
}

#[derive(Clone, Debug)]
pub struct ItemBase {
    pub id: ItemBaseId,
    pub name: Arc<str>,
    pub description: Option<Arc<str>>,
    pub image_url: Option<Arc<str>>,
    pub category: Category,
    pub attributes: Arc<[Attribute]>,
    pub base_durability: Durability,
}

impl ItemBase {
    pub fn builder(name: &str, category: Category) -> ItemBaseBuilder {
        ItemBaseBuilder::new(name, category)
    }
}

pub struct ItemBaseBuilder {
    id: ItemBaseId,
    name: Arc<str>,
    description: Option<Arc<str>>,
    image_url: Option<Arc<str>>,
    category: Category,
    attributes: Vec<Attribute>,
    base_durability: Durability,
}

impl ItemBaseBuilder {
    fn new(name: &str, category: Category) -> Self {
        Self {
            id: Id::new(uuid::Uuid::new_v4().to_string()),
            name: name.into(),
            description: None,
            image_url: None,
            category,
            attributes: Vec::new(),
            base_durability: 0,
        }
    }

    pub fn id(mut self, id: ItemBaseId) -> Self {
        self.id = id;
        self
    }

    pub fn description(mut self, description: &str) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn image_url(mut self, image_url: &str) -> Self {
        self.image_url = Some(image_url.into());
        self
    }

    pub fn attribute(mut self, modifier: &Modifier, value: u32, decimals: Option<u32>) -> Self {
        self.attributes.push(Attribute {
            modifier: modifier.clone(),
            value,
            decimals: decimals.unwrap_or(0),
        });
        self
    }

    pub fn base_durability(mut self, base_durability: Durability) -> Self {
        self.base_durability = base_durability;
        self
    }

    pub fn build(self) -> ItemBase {
        ItemBase {
            id: self.id,
            name: self.name,
            description: self.description,
            image_url: self.image_url,
            category: self.category,
            attributes: self.attributes.into(),
            base_durability: self.base_durability,
        }
    }
}
