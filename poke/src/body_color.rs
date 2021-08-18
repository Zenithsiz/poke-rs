//! Body color

/// Pokemon body color
#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum BodyColor {
	Red    = 0,
	Blue   = 1,
	Yellow = 2,
	Green  = 3,
	Black  = 4,
	Brown  = 5,
	Purple = 6,
	Gray   = 7,
	White  = 8,
	Pink   = 9,
}

impl BodyColor {
	/// Returns the `C` name of this body color
	pub fn c_name(&self) -> &'static str {
		match self {
			BodyColor::Red => "RED",
			BodyColor::Blue => "BLUE",
			BodyColor::Yellow => "YELLOW",
			BodyColor::Green => "GREEN",
			BodyColor::Black => "BLACK",
			BodyColor::Brown => "BROWN",
			BodyColor::Purple => "PURPLE",
			BodyColor::Gray => "GRAY",
			BodyColor::White => "WHITE",
			BodyColor::Pink => "PINK",
		}
	}
}
