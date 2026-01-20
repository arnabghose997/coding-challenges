mod cli;
mod server;

use clap::Parser;
use cli::Cli;
use server::run_server;

fn main() {
    let _ = Cli::parse();

    run_server()
}
