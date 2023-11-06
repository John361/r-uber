use thiserror::Error;

#[derive(Debug)]
pub struct Argument {
    pub config_path: String,
}

#[derive(Error, Debug)]
pub enum ArgumentError {
    #[error("Missing argument: {0}")]
    MissingArgument(String),
}

impl Argument {
    pub fn build() -> Result<Self, ArgumentError> {
        let cmd = clap::Command::new("r-uber")
            .bin_name("r-uber")
            .subcommand_required(true)
            .subcommand(
                clap::command!("start").arg(
                    clap::arg!(-c --config <PATH> "configuration full file path")
                        .value_parser(clap::value_parser!(std::path::PathBuf)),
                ),
            );

        let matches = cmd.get_matches();
        let matches = match matches.subcommand() {
            Some(("start", matches)) => matches,
            _ => unreachable!("clap should ensure we don't get here"),
        };

        match matches.get_one::<std::path::PathBuf>("config") {
            Some(config_path) => Ok(Argument {
                config_path: config_path.as_path().display().to_string(),
            }),

            None => Err(ArgumentError::MissingArgument("config".to_string())),
        }
    }
}
