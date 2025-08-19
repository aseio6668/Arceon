use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Item component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    pub description: String,
    pub item_type: ItemType,
    pub rarity: Rarity,
    pub value: u32,
    pub stack_size: u32,
    pub properties: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ItemType {
    Weapon(WeaponType),
    Armor(ArmorType),
    Consumable,
    Material,
    Tool,
    Book,
    Currency,
    Quest,
    Misc,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WeaponType {
    Sword,
    Axe,
    Mace,
    Dagger,
    Bow,
    Crossbow,
    Staff,
    Wand,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArmorType {
    Head,
    Chest,
    Legs,
    Feet,
    Hands,
    Arms,
    Shoulders,
    Back,
    Waist,
    Ring,
    Neck,
    Shield,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Artifact,
}
