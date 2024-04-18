use super::{Durability, ItemBase};
use generic_type::Id;
use std::sync::Arc;

pub type ItemId = Id;

#[derive(Clone, Debug)]
pub struct Item {
    pub id: ItemId,
    pub base: Arc<ItemBase>,
    pub durability: Durability,
    pub is_locked: bool,
}

impl Item {
    pub fn builder(base: &ItemBase) -> ItemBuilder {
        ItemBuilder::new(base)
    }

    pub fn is_broken(&self) -> bool {
        self.durability < 1
    }
}

pub struct ItemBuilder {
    id: ItemId,
    base: ItemBase,
    durability: Durability,
    is_locked: bool,
}

impl ItemBuilder {
    fn new(base: &ItemBase) -> Self {
        Self {
            id: Id::new(uuid::Uuid::new_v4()),
            base: base.clone(),
            durability: base.base_durability,
            is_locked: false,
        }
    }

    pub fn id(mut self, id: ItemId) -> Self {
        self.id = id;
        self
    }

    pub fn durability(mut self, durability: Durability) -> Self {
        self.durability = durability;
        self
    }

    pub fn is_locked(mut self, is_locked: bool) -> Self {
        self.is_locked = is_locked;
        self
    }

    pub fn build(self) -> Item {
        Item {
            id: self.id,
            base: Arc::new(self.base),
            durability: self.durability,
            is_locked: self.is_locked,
        }
    }
}
