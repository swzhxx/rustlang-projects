use std::error::Error;

use log::{debug, trace};
use ml_utils::dataset::get_boston_records_from_file;
use rand::{seq::SliceRandom, thread_rng};
use rusty_machine::prelude::Matrix;

pub fn run() -> Result<(), Box<dyn Error>> {
    let filename = "data/housing.csv";
    let mut data = get_boston_records_from_file(&filename);

    data.shuffle(&mut thread_rng());

    // separate out to train and test datasets
    let test_size: f64 = 0.2;
    let test_size: f64 = data.len() as f64 * test_size;
    let test_size = test_size.round() as usize;
    let (test_data, train_data) = data.split_at(test_size);
    let train_size = train_data.len();
    let test_size = test_data.len();

    trace!("train_size {} , test_size {}", train_size, test_size);
    Ok(())
}
