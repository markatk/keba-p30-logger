#![allow(clippy::bool_comparison)]

use std::env;
use fern::Dispatch;
use log::{LevelFilter, info};
use chrono::Local;

fn main() {
    // load env variables from .env file
    dotenv::dotenv().ok();

    // configure logger
    configure_logger();

    info!("Starting Keba P30 Logger v{}", env!("CARGO_PKG_VERSION"));
}

fn configure_logger() {
    let log_level = env::var("LOG_LEVEL")
        .unwrap_or_else(|_| "INFO".into())
        .parse::<LevelFilter>()
        .expect("Unknown log level");

    let mut dispatch = Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}] {}",
                Local::now().format("[%Y-%m-%d][%T%.3f]"),
                record.level(),
                message
            ))
        })
        .level(log_level)
        .chain(std::io::stdout());

    // add file to logger if any is specified
    if let Ok(log_file) = env::var("LOG_FILE") {
        if log_file.trim().is_empty() == false {
            dispatch = dispatch.chain(fern::log_file(log_file).unwrap());
        }
    }

    dispatch
        .apply()
        .unwrap();
}
