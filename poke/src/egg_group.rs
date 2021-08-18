//! Egg group


/// Pokemon egg group
#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum EggGroup {
	Monster      = 1,
	Water1       = 2,
	Bug          = 3,
	Flying       = 4,
	Field        = 5,
	Fairy        = 6,
	Grass        = 7,
	HumanLike    = 8,
	Water3       = 9,
	Mineral      = 10,
	Amorphous    = 11,
	Water2       = 12,
	Ditto        = 13,
	Dragon       = 14,
	Undiscovered = 15,
}

impl EggGroup {
	/// Returns the `C` name of this item
	pub fn c_name(&self) -> &'static str {
		match self {
			EggGroup::Monster => "MONSTER",
			EggGroup::Water1 => "WATER_1",
			EggGroup::Bug => "BUG",
			EggGroup::Flying => "FLYING",
			EggGroup::Field => "FIELD",
			EggGroup::Fairy => "FAIRY",
			EggGroup::Grass => "GRASS",
			EggGroup::HumanLike => "HUMAN_LIKE",
			EggGroup::Water3 => "WATER_3",
			EggGroup::Mineral => "MINERAL",
			EggGroup::Amorphous => "AMORPHOUS",
			EggGroup::Water2 => "WATER_2",
			EggGroup::Ditto => "DITTO",
			EggGroup::Dragon => "DRAGON",
			EggGroup::Undiscovered => "UNDISCOVERED",
		}
	}
}
