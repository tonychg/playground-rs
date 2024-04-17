use generic_type::Id;

use anyhow::{bail, Result};
use std::{collections::HashMap, sync::Arc};

pub type Durability = u32;

#[derive(thiserror::Error, Clone, Debug)]
pub enum Error {
    #[error("Equipment is full")]
    EquipmentIsFull,
    #[error("Cannot equip broken equipment")]
    BrokenEquipment,
    #[error("Equipment is locked")]
    EquipmentIsLocked,
    #[error("Equipment not in inventory")]
    EquipmentNotInInventory,
    #[error("Not enough runes in inventory")]
    NotEnoughRune,
}

#[derive(Clone, Debug)]
pub enum Effect {
    UnlockExpertMission,
}

#[derive(Clone, Debug)]
pub enum Modifier {
    IncreaseTokenAmount,
    IncreaseItemDropRate,
    EquipmentSlot,
    ReduceMissionDuration,
}

#[derive(Clone, Debug)]
pub enum Spell {
    RechargeDurability,
}

#[derive(Clone, Debug)]
pub struct ItemBase {
    pub name: Arc<str>,
    pub image_url: Arc<str>,
    pub description: Arc<str>,
}

impl Default for ItemBase {
    fn default() -> Self {
        Self {
            name: "".into(),
            image_url: "".into(),
            description: "".into(),
        }
    }
}

impl ItemBase {
    pub fn create(name: &str, image_url: &str, description: &str) -> Self {
        Self {
            name: name.into(),
            image_url: image_url.into(),
            description: description.into(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Equipment {
    pub item_id: Id,
    pub base: Arc<ItemBase>,
    pub attributes: Arc<[(Modifier, u32)]>,
    pub effects: Arc<[Effect]>,
    pub base_durability: Durability,
    pub is_consumable: bool,
}

impl Equipment {
    pub fn builder() -> EquipmentBuilder {
        EquipmentBuilder::default()
    }
}

#[derive(Clone, Debug)]
pub struct EquipmentBuilder {
    pub item_id: Id,
    pub base: Arc<ItemBase>,
    pub attributes: Arc<[(Modifier, u32)]>,
    pub effects: Arc<[Effect]>,
    pub base_durability: Durability,
    pub is_consumable: bool,
}

impl Default for EquipmentBuilder {
    fn default() -> Self {
        Self {
            item_id: Id::new(uuid::Uuid::new_v4()),
            base: Arc::new(ItemBase::default()),
            attributes: Arc::new([]),
            effects: Arc::new([]),
            base_durability: 0,
            is_consumable: false,
        }
    }
}

impl EquipmentBuilder {
    pub fn attributes(mut self, attributes: &[(Modifier, u32)]) -> Self {
        self.attributes = Arc::from(attributes);
        self
    }

    pub fn effects(mut self, effects: &[Effect]) -> Self {
        self.effects = Arc::from(effects);
        self
    }

    pub fn base_durability(mut self, number: Durability) -> Self {
        self.base_durability = number;
        self
    }

    pub fn build(self) -> Equipment {
        Equipment {
            item_id: self.item_id,
            base: self.base,
            attributes: self.attributes,
            effects: self.effects,
            base_durability: self.base_durability,
            is_consumable: self.is_consumable,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Rune {
    pub item_id: Id,
    pub base: Arc<ItemBase>,
    pub spells: Arc<[Spell]>,
}

#[derive(Clone, Debug)]
pub struct RuneQuantity {
    pub rune: Arc<Rune>,
    pub quantity: usize,
}

pub const BASE_EQUIPMENT_SIZE: usize = 4;
pub const MAX_EQUIPMENT_SIZE: usize = 8;

#[derive(Clone, Debug)]
pub struct EquipmentSlot {
    pub slot_id: Id,
    pub equipment: Arc<Equipment>,
    pub durability: Durability,
    pub is_locked: bool,
}

impl EquipmentSlot {
    pub fn create(equipment: Equipment) -> Self {
        Self {
            slot_id: Id::new(uuid::Uuid::new_v4()),
            equipment: Arc::new(equipment.clone()),
            durability: equipment.base_durability,
            is_locked: false,
        }
    }

    pub fn is_broken(&self) -> bool {
        self.durability == 0
    }

    pub fn lock(&mut self) {
        self.is_locked = true;
    }

    pub fn unlock(&mut self) {
        self.is_locked = false;
    }
}

pub struct Gear {
    pub consumable: Option<EquipmentSlot>,
    pub equipment: Vec<EquipmentSlot>,
}

impl Gear {
    pub fn size(&self) -> usize {
        BASE_EQUIPMENT_SIZE - self.equipment.len()
    }

    pub fn is_full(&self) -> bool {
        self.size() == 0
    }

    pub fn equip(&mut self, slot: EquipmentSlot) -> Result<()> {
        if slot.equipment.is_consumable {
            self.consumable = Some(slot);
        } else if self.is_full() {
            bail!(Error::EquipmentIsFull);
        } else if slot.is_broken() {
            bail!(Error::BrokenEquipment);
        } else if slot.is_locked {
            bail!(Error::EquipmentIsLocked);
        } else {
            self.equipment.push(slot);
        }
        Ok(())
    }
}

#[derive(Default, Clone, Debug)]
pub struct RuneInventory {
    pub runes: HashMap<Id, RuneQuantity>,
}

impl RuneInventory {
    pub fn add(&mut self, rune: &Rune) {
        match self.runes.get_mut(&rune.item_id) {
            Some(rune_quantity) => {
                rune_quantity.quantity += 1;
            }
            None => {
                self.runes.insert(
                    rune.item_id.clone(),
                    RuneQuantity {
                        rune: Arc::new(rune.clone()),
                        quantity: 1,
                    },
                );
            }
        }
    }

    pub fn consume(&mut self, rune_id: &Id) -> Result<()> {
        match self.runes.get_mut(rune_id) {
            Some(rune) => {
                if rune.quantity > 0 {
                    bail!(Error::NotEnoughRune);
                } else {
                    rune.quantity -= 1;
                    Ok(())
                }
            }
            None => bail!(Error::NotEnoughRune),
        }
    }
}

#[derive(Default, Clone, Debug)]
pub struct EquipmentInventory {
    pub equipment: HashMap<Id, EquipmentSlot>,
}

impl EquipmentInventory {
    pub fn add(&mut self, slot: EquipmentSlot) {
        self.equipment.insert(slot.equipment.item_id.clone(), slot);
    }

    pub fn remove(&mut self, equipment_id: &Id) -> Result<EquipmentSlot> {
        match self.equipment.remove(equipment_id) {
            Some(slot) => Ok(slot),
            None => bail!(Error::EquipmentNotInInventory),
        }
    }
}
