//! Growth rate

/// Pokemon growth rate
#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum GrowthRate {
	MediumFast  = 0,
	Erratic     = 1,
	Fluctuating = 2,
	MediumSlow  = 3,
	Fast        = 4,
	Slow        = 5,
}

impl GrowthRate {
	/// Returns the `C` name of this growth rate
	pub fn c_name(&self) -> &'static str {
		match self {
			GrowthRate::MediumFast => "MEDIUM_FAST",
			GrowthRate::Erratic => "ERRATIC",
			GrowthRate::Fluctuating => "FLUCTUATING",
			GrowthRate::MediumSlow => "MEDIUM_SLOW",
			GrowthRate::Fast => "FAST",
			GrowthRate::Slow => "SLOW",
		}
	}
}
