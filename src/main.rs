use std::time::Duration;

use app::App;
use args::Cli;
use clap::Parser;

mod app;
mod args;
mod cell;
mod col;
mod colors;
mod keymap;
mod row;
mod ui;

pub(crate) static EVENT_TIMEOUT: Duration = Duration::from_millis(1000);
fn main() {
    let cli = Cli::parse();

    let path = cli.path;
    let sheet = cli.sheet;

    let app = App::new(path, sheet).unwrap();

    ui::run(app).unwrap();
}
