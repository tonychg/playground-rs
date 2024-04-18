use super::{Attribute, Category, Item, ItemId, Modifier};
use generic_type::Id;

use anyhow::{bail, Result};
use std::{cmp::min, collections::HashMap, sync::Arc};

pub type InventoryId = Id;

#[derive(thiserror::Error, Clone, Debug)]
pub enum InventoryError {
    #[error("Item with id {0} not found in inventory {1}")]
    ItemNotFound(ItemId, InventoryId),
    #[error("Item with id {0} is currently locked")]
    ItemIsLocked(ItemId),
    #[error("Try to apply a non-rune item")]
    InvalidRune,
    #[error("Rune not applicable on this item")]
    RuneNotApplicable,
    #[error("Target inventory {0} is full")]
    InventoryFull(InventoryId),
}

#[derive(Clone, Debug)]
pub struct InventoryLimits {
    pub consumable: Option<usize>,
    pub equipment: Option<usize>,
    pub rune: Option<usize>,
}

#[derive(Clone, Debug)]
pub struct Inventory {
    pub id: InventoryId,
    pub limits: Arc<InventoryLimits>,
    pub items: HashMap<ItemId, Item>,
}

impl Inventory {
    pub fn transfer(&mut self, item_id: &ItemId, target: &mut Inventory) -> Result<()> {
        let item = self.request_item(item_id)?;
        if self.is_full(&item.base.category) {
            bail!(InventoryError::InventoryFull(target.id.clone()));
        }
        self.remove(item_id);
        target.update(item);
        Ok(())
    }

    pub fn apply_rune(&mut self, rune_id: &ItemId, item_id: &ItemId) -> Result<()> {
        let rune = self.request_item(rune_id)?;
        let mut item = self.request_item(item_id)?;
        if rune.base.category != Category::Rune {
            bail!(InventoryError::InvalidRune);
        } else if item.base.category != Category::Equipment {
            bail!(InventoryError::RuneNotApplicable);
        } else {
            for attribute in rune.base.attributes.iter() {
                if attribute.modifier == Modifier::RechargeDurability {
                    item.durability =
                        min(item.base.base_durability, item.durability + attribute.value);
                }
            }
            self.remove(rune_id);
            self.update(item);
            Ok(())
        }
    }

    pub fn remove_broken_consumables(&mut self) {
        self.items.retain(|_, item| match item.base.category {
            Category::Consumable => !item.is_broken(),
            _ => true,
        });
    }

    pub fn consume(&mut self) -> Result<()> {
        self.remove_broken_consumables();
        self.items
            .values_mut()
            .for_each(|item| match item.base.category {
                Category::Consumable => item.durability -= 1,
                Category::Equipment => {
                    if !item.is_broken() {
                        item.durability -= 1;
                    }
                }
                Category::Rune => (),
            });
        Ok(())
    }

    pub fn remove(&mut self, item_id: &ItemId) {
        self.items.remove(item_id);
    }

    pub fn update(&mut self, item: Item) {
        self.items.insert(item.id.clone(), item);
    }

    pub fn is_full(&self, category: &Category) -> bool {
        let count = self.count(category);
        let limit = match category {
            Category::Consumable => self.limits.consumable.unwrap_or(usize::MAX),
            Category::Equipment => self.limits.equipment.unwrap_or(usize::MAX),
            Category::Rune => self.limits.rune.unwrap_or(usize::MAX),
        };
        count < limit
    }

    pub fn count(&self, category: &Category) -> usize {
        self.items
            .values()
            .filter(|item| &item.base.category == category)
            .count()
    }

    pub fn modifier_total(&self, modifier: &Modifier) -> Option<Attribute> {
        let attributes: Vec<&Attribute> = self
            .items
            .values()
            .flat_map(|item| item.base.attributes.iter())
            .filter(|attribute| &attribute.modifier == modifier)
            .collect();
        if attributes.is_empty() {
            None
        } else {
            Some(
                attributes
                    .into_iter()
                    .fold(Attribute::default(), |acc, x| acc + x.clone()),
            )
        }
    }

    pub fn has_item(&self, item_id: &ItemId) -> bool {
        self.items.contains_key(item_id)
    }

    pub fn request_item(&self, item_id: &ItemId) -> Result<Item> {
        let item = self
            .items
            .get(item_id)
            .cloned()
            .ok_or_else(|| InventoryError::ItemNotFound(item_id.clone(), self.id.clone()))?;
        if item.is_locked {
            bail!(InventoryError::ItemIsLocked(item_id.clone()))
        }
        Ok(item)
    }
}
