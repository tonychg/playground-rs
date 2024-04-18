use generic_type::Id;
use std::{ops, sync::Arc};

pub type ItemBaseId = Id;
pub type Durability = u32;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Category {
    Equipment,
    Consumable,
    Rune,
}

#[derive(Default, Clone, Debug)]
pub struct Attributes {
    pub increase_token_amount: f64,
    pub increase_item_drop_rate: f64,
    pub reduce_mission_duration: f64,
    pub unlock_expert_mission: bool,
    pub equipment_slot: usize,
    pub recharge_durability: u32,
}

impl ops::Add<Attributes> for Attributes {
    type Output = Attributes;

    fn add(self, rhs: Attributes) -> Self::Output {
        let mut result = self.clone();
        result.increase_token_amount += rhs.increase_token_amount;
        result.increase_item_drop_rate += rhs.increase_item_drop_rate;
        result.reduce_mission_duration += rhs.reduce_mission_duration;
        result.unlock_expert_mission |= rhs.unlock_expert_mission;
        result.equipment_slot += rhs.equipment_slot;
        result.recharge_durability += rhs.recharge_durability;
        result
    }
}

#[derive(Clone, Debug)]
pub struct ItemBase {
    pub id: ItemBaseId,
    pub name: Arc<str>,
    pub description: Option<Arc<str>>,
    pub image_url: Option<Arc<str>>,
    pub category: Category,
    pub attributes: Arc<Attributes>,
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
    attributes: Attributes,
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
            attributes: Attributes::default(),
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

    pub fn attributes(mut self, attributes: Attributes) -> Self {
        self.attributes = attributes;
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
