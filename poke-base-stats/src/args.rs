//! Cmd arguments

// Imports
use std::path::PathBuf;

/// Arguments
#[derive(Clone, Debug)]
pub enum Args {
	/// Serialize to C
	SerializeC {
		/// Input file
		input_file: PathBuf,
	},
}

/// Parses arguments
pub fn parse() -> Result<Args, anyhow::Error> {
	const SERIALIZE_C_SUBCMD: &str = "serialize-c";
	const INPUT_FILE_STR: &str = "input-file";

	// Serialize to C subcommand
	let serialize_c_subcmd = clap::SubCommand::with_name(SERIALIZE_C_SUBCMD)
		.help("Serialize the json base stats to C")
		.arg(
			clap::Arg::with_name(INPUT_FILE_STR)
				.takes_value(true)
				.required(true)
				.index(1),
		);

	let matches = clap::App::new("poke-base-stats")
		.subcommands([serialize_c_subcmd])
		.get_matches();

	let args = match matches.subcommand() {
		(SERIALIZE_C_SUBCMD, Some(matches)) => {
			let input_file = matches
				.value_of_os(INPUT_FILE_STR)
				.map(PathBuf::from)
				.expect("Required argument missing");

			Args::SerializeC { input_file }
		},

		(subcmd, _) => anyhow::bail!("Unknown subcommand {}", subcmd),
	};

	Ok(args)
}
