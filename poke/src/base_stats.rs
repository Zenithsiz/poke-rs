//! Base stats

// IMports
use crate::{Ability, BodyColor, EggGroup, GrowthRate, Item, Type};

/// Stats
#[derive(PartialEq, Clone, Copy, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Stats {
	/// Hp
	pub hp: u8,

	/// Attack
	pub atk: u8,

	/// Defense
	pub def: u8,

	/// Speed
	pub speed: u8,

	/// Special attack
	pub sp_atk: u8,

	/// Special defense
	pub sp_def: u8,
}

/// Base stats
#[derive(PartialEq, Clone, Copy, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BaseStats {
	// Base stats
	pub stats: Stats,

	/// Types
	pub types: [Type; 2],

	/// Catch rate
	pub catch_rate: u8,

	/// Exp yield
	pub exp_yield: u8,

	// Ev yield
	pub ev_yield: Stats,

	/// Items
	pub items: [Option<Item>; 2],

	/// Gender
	pub gender: Gender,

	/// Egg cycles
	pub egg_cycles: u8,

	/// Friendship
	pub friendship: u8,

	/// Growth Rate
	pub growth_rate: GrowthRate,

	/// Egg groups
	pub egg_groups: [EggGroup; 2],

	/// Abilities
	pub abilities: [Option<Ability>; 2],

	/// Safari zone flee rate
	pub safari_zone_flee_rate: u8,

	/// Body color
	pub body_color: BodyColor,

	/// No flip
	pub no_flip: bool,
}

/// Pokemon gender
#[derive(PartialEq, Clone, Copy, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
pub enum Gender {
	PercentFemale { p: f32 },
	AlwaysMale,
	AlwaysFemale,
	Genderless,
}
