use std::env::set_var;

use env_logger::{builder, Builder};
use log::info;
mod gaussian_process_reg;
mod glm;
mod lin_reg;

fn main() {
    set_var("RUST_LOG", "trace");
    let mut builder = Builder::from_default_env();
    builder.init();
    info!("run rustymachine_regressin");
    lin_reg::run();
    gaussian_process_reg::run();
    glm::run();
}
