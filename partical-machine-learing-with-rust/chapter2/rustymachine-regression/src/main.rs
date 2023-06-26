use std::env::set_var;

use log::info;

mod lin_reg;
fn main() {
    set_var("RUST_LOG", "trace");
    env_logger::init();
    info!("run rustymachine_regressin");
    lin_reg::run();
}
