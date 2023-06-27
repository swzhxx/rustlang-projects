use std::{error::Error, io};

use log::info;
use ml_utils::dataset::Flower;
use rand::{seq::SliceRandom, thread_rng};
use rustlearn::linear_models::sgdclassifier::Hyperparameters as logistic_regression;
use rustlearn::metrics::accuracy_score;
use rustlearn::prelude::*;

pub fn run() -> Result<(), Box<dyn Error>> {
    info!("logistic reg run");
    let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut data = Vec::new();
    for result in rdr.deserialize() {
        let r: Flower = result?;
        data.push(r);
    }
    data.shuffle(&mut thread_rng());

    // separate out to train and test datasets
    let test_size: f32 = 0.2;
    let test_size: f32 = data.len() as f32 * test_size;
    let test_size = test_size.round() as usize;

    let (test_data, train_data) = data.split_at(test_size);
    let train_size = train_data.len();
    let test_size = test_data.len();

    // differenetiate the features and the labels
    let flower_x_train: Vec<f32> = train_data
        .iter()
        .flat_map(|r| r.into_feature_vector())
        .collect();

    let flower_y_train: Vec<f32> = train_data.iter().map(|r| r.into_labels()).collect();

    let flower_x_test: Vec<f32> = test_data
        .iter()
        .flat_map(|r| r.into_feature_vector())
        .collect();
    let flower_y_test: Vec<f32> = test_data.iter().map(|r| r.into_labels()).collect();

    // Convert the vectors to a dense matrix or a sparse matrix
    let mut flower_x_train = Array::from(flower_x_train);
    flower_x_train.reshape(train_size, 4);
    let flower_y_train = Array::from(flower_y_train);

    let mut flower_x_test = Array::from(flower_x_test);
    flower_x_test.reshape(test_size, 4);
    let flower_y_test = Array::from(flower_y_test);
    // working with Stochastic Gradient descent.
    // uses adaptive per parameter learning rate Adagrad
    let mut model = logistic_regression::new(4)
        .learning_rate(0.1)
        .l2_penalty(0.5)
        .l1_penalty(0.)
        .one_vs_rest();

    let num_epochs = 100;
    info!("data collect complete");
    for i in 0..num_epochs {
        model.fit(&flower_x_train, &flower_y_train).unwrap();
        info!("num epoches {:?}", i + 1);
    }

    let prediction = model.predict(&flower_x_test).unwrap();
    let acc1 = accuracy_score(&flower_y_test, &prediction);
    info!("Logistic Regression : accuracy: {:?}", acc1);
    Ok(())
}
