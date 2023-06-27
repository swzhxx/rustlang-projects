use std::env::set_var;

use env_logger::Builder;
mod logistic_reg;
mod tree;
fn main() {
    set_var("RUST_LOG", "trace");
    let mut builder = Builder::from_default_env();
    builder.init();
    logistic_reg::run();
}
