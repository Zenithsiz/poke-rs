//! Pokemon

// Features
#![feature(format_args_capture, array_methods)]

// Modules
mod args;

// Imports
use anyhow::Context;
use poke::{Ability, BaseStats, EggGroup, Gender, Item, Species};
use std::{collections::BTreeMap, fs, path::Path};

fn main() -> Result<(), anyhow::Error> {
	// Initialize logger
	simplelog::TermLogger::init(
		log::LevelFilter::Debug,
		simplelog::Config::default(),
		simplelog::TerminalMode::Stderr,
		simplelog::ColorChoice::Auto,
	)
	.context("Unable to initialize logger")?;

	// Get arguments
	let args = args::parse().context("Unable to parse arguments")?;

	match args {
		args::Args::SerializeC { input_file } => {
			self::serialize_c(&input_file)
				.with_context(|| format!("Unable to serialize file {input_file:?} to `C`"))?;
		},
	}

	Ok(())
}

/// Serialize a json to `C` and prints it
pub fn serialize_c(input_file: &Path) -> Result<(), anyhow::Error> {
	// Open the file
	let file = fs::File::open(&input_file).context("Unable to open file")?;

	// Then parse it
	let base_stats =
		serde_json::from_reader::<_, BTreeMap<Species, BaseStats>>(file).context("Unable to parse file")?;

	// Then emmit a `C` array element for each species
	for (species, base_stats) in base_stats {
		let species = species.c_name();

		let base_hp = base_stats.stats.hp;
		let base_atk = base_stats.stats.atk;
		let base_def = base_stats.stats.def;
		let base_speed = base_stats.stats.speed;
		let base_sp_atk = base_stats.stats.sp_atk;
		let base_sp_def = base_stats.stats.sp_def;

		let [type1, type2] = base_stats.types.map(|ty| ty.c_name());

		let catch_rate = base_stats.catch_rate;
		let exp_yield = base_stats.exp_yield;

		let ev_yield_hp = base_stats.ev_yield.hp;
		let ev_yield_atk = base_stats.ev_yield.atk;
		let ev_yield_def = base_stats.ev_yield.def;
		let ev_yield_speed = base_stats.ev_yield.speed;
		let ev_yield_sp_atk = base_stats.ev_yield.sp_atk;
		let ev_yield_sp_def = base_stats.ev_yield.sp_def;

		let [item1, item2] = base_stats.items.map(|item| item.as_ref().map_or("NONE", Item::c_name));

		let gender_ratio = zutil::DisplayWrapper::new(|f| match base_stats.gender {
			Gender::PercentFemale { p } => write!(f, "PERCENT_FEMALE({p})"),
			Gender::AlwaysMale => write!(f, "MON_MALE"),
			Gender::AlwaysFemale => write!(f, "MON_FEMALE"),
			Gender::Genderless => write!(f, "MON_GENDERLESS"),
		});

		let egg_cycles = base_stats.egg_cycles;
		let friendship = base_stats.friendship;

		let growth_rate = base_stats.growth_rate.c_name();

		let [egg_group1, egg_group2] = base_stats.egg_groups.each_ref().map(EggGroup::c_name);

		let [ability1, ability2] = base_stats
			.abilities
			.map(|ability| ability.as_ref().map_or("NONE", Ability::c_name));

		let safari_zone_flee_rate = base_stats.safari_zone_flee_rate;
		let body_color = base_stats.body_color.c_name();
		let no_flip = match base_stats.no_flip {
			true => "TRUE",
			false => "FALSE",
		};

		println!(
			r#"[SPECIES_{species}] = {{
	.baseHP = {base_hp},
	.baseAttack = {base_atk},
	.baseDefense = {base_def},
	.baseSpeed = {base_speed},
	.baseSpAttack = {base_sp_atk},
	.baseSpDefense = {base_sp_def},
	.type1 = TYPE_{type1},
	.type2 = TYPE_{type2},
	.catchRate = {catch_rate},
	.expYield = {exp_yield},
	.evYield_HP = {ev_yield_hp},
	.evYield_Attack = {ev_yield_atk},
	.evYield_Defense = {ev_yield_def},
	.evYield_Speed = {ev_yield_speed},
	.evYield_SpAttack = {ev_yield_sp_atk},
	.evYield_SpDefense = {ev_yield_sp_def},
	.item1 = ITEM_{item1},
	.item2 = ITEM_{item2},
	.genderRatio = {gender_ratio},
	.eggCycles = {egg_cycles},
	.friendship = {friendship},
	.growthRate = GROWTH_{growth_rate},
	.eggGroup1 = EGG_GROUP_{egg_group1},
	.eggGroup2 = EGG_GROUP_{egg_group2},
	.abilities = {{ABILITY_{ability1}, ABILITY_{ability2}}},
	.safariZoneFleeRate = {safari_zone_flee_rate},
	.bodyColor = BODY_COLOR_{body_color},
	.noFlip = {no_flip},
}},"#,
		);
	}

	Ok(())
}
