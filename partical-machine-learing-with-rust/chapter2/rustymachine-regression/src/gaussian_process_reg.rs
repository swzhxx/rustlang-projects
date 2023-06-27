use std::error::Error;

use log::info;
use ml_utils::{dataset::get_boston_records_from_file, sup_metrics::r_squared_score};
use rand::{seq::SliceRandom, thread_rng};
use rusty_machine::{
    analysis::score::neg_mean_squared_error,
    learning::{
        gp::{ConstMean, GaussianProcess},
        toolkit::kernel,
    },
    prelude::{Matrix, SupModel, Vector},
};

pub fn run() -> Result<(), Box<dyn Error>> {
    let filename = "data/housing.csv";
    let mut data = get_boston_records_from_file(&filename);

    // shuffle the data.
    data.shuffle(&mut thread_rng());

    // separate out to train and test datasets.
    let test_size: f64 = 0.2;
    let test_size: f64 = data.len() as f64 * test_size;
    let test_size = test_size.round() as usize;
    let (test_data, train_data) = data.split_at(test_size);
    let train_size = train_data.len();
    let test_size = test_data.len();

    // differentiate the features and the targets.
    let boston_x_train: Vec<f64> = train_data
        .iter()
        .flat_map(|r| r.into_feature_vector())
        .collect();
    let boston_y_train: Vec<f64> = train_data.iter().map(|r| r.into_targets()).collect();

    let boston_x_test: Vec<f64> = test_data
        .iter()
        .flat_map(|r| r.into_feature_vector())
        .collect();
    let boston_y_test: Vec<f64> = test_data.iter().map(|r| r.into_targets()).collect();

    // Convert the data into matrices for rusty machine
    let boston_x_train = Matrix::new(train_size, 13, boston_x_train);
    let boston_y_train = Vector::new(boston_y_train);
    let boston_x_test = Matrix::new(test_size, 13, boston_x_test);
    // let boston_y_test = Vector::new(boston_y_test);
    let boston_y_test = Matrix::new(test_size, 1, boston_y_test);

    // the kernel function
    let ker = kernel::SquaredExp::new(2., 1.);
    // the mean function
    let zero_mean = ConstMean::default();
    // model definition
    let mut gaus_model = GaussianProcess::new(ker, zero_mean, 10f64);
    gaus_model.train(&boston_x_train, &boston_y_train)?;

    let predictions = gaus_model.predict(&boston_x_test).unwrap();
    let predictions = Matrix::new(test_size, 1, predictions);

    let acc = neg_mean_squared_error(&predictions, &boston_y_test);
    info!("gaussian process regression error :{:?}", acc);
    info!(
        "gaussian process regression R2 scrore:{:?}",
        r_squared_score(&boston_y_test.data(), &predictions.data())
    );

    Ok(())
}
