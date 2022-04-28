use std::process::exit;
use log::{LevelFilter, error};
use simple_logger::SimpleLogger;
use types::Cli;
use infra::writer::Fs;
use infra::time::Chrono;
use handler::Handler;
use clap::Parser;

mod types;
mod handler;
mod infra;

fn main() {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();
    let args = Cli::parse();
    let time_gen = Box::new(Chrono {});
    let fs_writer = Box::new(Fs {});
    let handler = Handler::new(fs_writer, time_gen);

    match handler.handle(args) {
        Ok(..) => exit(exitcode::OK),
        Err(e) => {
            error!("Something went wrong. Error: {}", e);
            exit(exitcode::USAGE)
        }
    }
}