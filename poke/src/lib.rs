//! Pokemon

// Features
#![feature(format_args_capture, array_methods)]

// Modules
mod ability;
mod base_stats;
mod body_color;
mod egg_group;
mod growth_rate;
mod item;
mod species;
mod ty;

// Exports
pub use ability::Ability;
pub use base_stats::{BaseStats, Gender};
pub use body_color::BodyColor;
pub use egg_group::EggGroup;
pub use growth_rate::GrowthRate;
pub use item::Item;
pub use species::Species;
pub use ty::Type;
