use std::process::exit;

#[macro_use] extern crate scan_fmt;
mod errors;
mod cli;
mod container;
mod config;

use errors::exit_with_retcode;

fn main() {
    match cli::parse_args(){
        Ok(args) => {
            log::info!("{:?}", args);
            exit_with_retcode(container::start(args))
        },
        Err(e) => {
            log::error!("Error while parsing arguments:\n\t{}", e);
            exit(e.get_retcode());
        }
    };
}
