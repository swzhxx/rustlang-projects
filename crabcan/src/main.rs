use std::process::exit;

use container::{check_linux_version, Container};
use errors::Errcode;

use crate::errors::exit_with_retcode;
mod child;
mod cli;
mod config;
mod container;
mod errors;
mod hostname;
mod ipc;

fn main() -> Result<(), Errcode> {
    let args = cli::parse_args();

    match args {
        Ok(args) => {
            log::info!("{:?}", args);
            exit_with_retcode(container::start(args))
        }
        Err(e) => {
            log::error!("Error while parsing arguments:\n\t{}", e);
            exit(e.get_retcode());
        }
    };

    Ok(())
}
