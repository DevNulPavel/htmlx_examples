use clap::Parser;
use std::path::PathBuf;

///////////////////////////////////////////////////////////////////////////////////////////////

/// Spider
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub(crate) struct AppArgs {
    /// Users file path
    #[clap(long, value_parser)]
    pub(crate) users_file_path: PathBuf,

    /// Events file path
    #[clap(long, value_parser)]
    pub(crate) events_file_path: PathBuf,
}
