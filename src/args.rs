use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    #[arg(short, long, value_name = "FILE_PATH")]
    pub path: PathBuf,
    #[arg(short, long, value_name = "SHEET_NAME")]
    pub sheet: String,
}
