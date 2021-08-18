//! Pokemon type

/// Pokemon type
#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Type {
	Normal   = 0x00,
	Fighting = 0x01,
	Flying   = 0x02,
	Poison   = 0x03,
	Ground   = 0x04,
	Rock     = 0x05,
	Bug      = 0x06,
	Ghost    = 0x07,
	Steel    = 0x08,
	Mystery  = 0x09,
	Fire     = 0x0a,
	Water    = 0x0b,
	Grass    = 0x0c,
	Electric = 0x0d,
	Psychic  = 0x0e,
	Ice      = 0x0f,
	Dragon   = 0x10,
	Dark     = 0x11,
}

impl Type {
	/// Returns the `C` name of this type
	pub fn c_name(&self) -> &'static str {
		match self {
			Type::Normal => "NORMAL",
			Type::Fighting => "FIGHTING",
			Type::Flying => "FLYING",
			Type::Poison => "POISON",
			Type::Ground => "GROUND",
			Type::Rock => "ROCK",
			Type::Bug => "BUG",
			Type::Ghost => "GHOST",
			Type::Steel => "STEEL",
			Type::Mystery => "MYSTERY",
			Type::Fire => "FIRE",
			Type::Water => "WATER",
			Type::Grass => "GRASS",
			Type::Electric => "ELECTRIC",
			Type::Psychic => "PSYCHIC",
			Type::Ice => "ICE",
			Type::Dragon => "DRAGON",
			Type::Dark => "DARK",
		}
	}
}
