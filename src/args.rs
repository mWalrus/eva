use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    #[arg(short, long, value_name = "FILE_PATH")]
    pub name: PathBuf,
    #[arg(short, long, value_name = "SHEET_NAME")]
    pub sheet: String,
    #[arg(short, long)]
    pub row: usize,
}
