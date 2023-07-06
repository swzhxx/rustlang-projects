use fasttext::{Args, FastText};
use itertools::Itertools;
use rand::{seq::SliceRandom, thread_rng};
use rust_stemmers::{Algorithm, Stemmer};
use serde_derive::Deserialize;

use std::io::Write;
use std::{collections::HashSet, error::Error, fs::File, io};
use stopwords::{Language, Spark, Stopwords};
use vtext::tokenize::{Tokenizer, VTextTokenizer, VTextTokenizerParams};

const TRAIN_FILE: &str = "data.train";
const TEST_FILE: &str = "data.test";
const MODEL: &str = "model.bin";

#[derive(Debug, Deserialize)]
pub struct SpookAuthor {
    id: String,
    text: String,
    author: String,
}

impl SpookAuthor {
    pub fn into_tokens(&self) -> String {
        // convert all to lowercase
        let lc_text = self.text.to_lowercase();

        // tokenise the words
        let tok = VTextTokenizer {
            params: VTextTokenizerParams::default(),
        };
        let tokens: Vec<&str> = tok.tokenize(lc_text.as_str()).collect();
        // stem the words
        let en_stemmer = Stemmer::create(Algorithm::English);
        let tokens: Vec<String> = tokens
            .iter()
            .map(|x| en_stemmer.stem(x).into_owned())
            .collect();
        let mut tokens: Vec<&str> = tokens.iter().map(|x| x.as_str()).collect();

        // remove the stopwords
        let stops: HashSet<_> = Spark::stopwords(Language::English)
            .unwrap()
            .iter()
            .collect();

        tokens.retain(|s| !stops.contains(s));

        tokens.iter().join(" ")
    }

    fn into_labels(&self) -> String {
        match self.author.as_str() {
            "EAP" => String::from("__label__EAP"),
            "HPL" => String::from("__label__HPL"),
            "MWS" => String::from("__label__MWS"),
            l => panic!(
                "Not able to parse the target. Some other target got passed. {:?} ",
                l
            ),
        }
    }
}

fn push_training_data_to_file(
    train_data: &[SpookAuthor],
    filename: &str,
) -> Result<(), Box<dyn Error>> {
    let mut f = File::create(filename)?;
    for item in train_data {
        writeln!(f, "{} {}", item.into_labels(), item.into_tokens())?;
    }
    Ok(())
}

fn push_test_data_to_file(test_data: &[SpookAuthor], filename: &str) -> Result<(), Box<dyn Error>> {
    let mut f = File::create(filename)?;
    for item in test_data {
        writeln!(f, "{}", item.into_tokens())?;
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut data = vec![];
    for result in rdr.deserialize() {
        let r: SpookAuthor = result?;
        data.push(r)
    }
    data.shuffle(&mut thread_rng());

    let test_size: f32 = 0.2;
    let test_size = data.len() as f32 * test_size;
    let test_size = test_size.round() as usize;

    let (test_data, train_data) = data.split_at(test_size);
    push_training_data_to_file(train_data.to_owned(), TRAIN_FILE)?;
    push_test_data_to_file(test_data.to_owned(), TEST_FILE)?;

    // model initiation and training
    let mut args = Args::new();
    args.set_input(TRAIN_FILE);
    args.set_model(fasttext::ModelName::SUP);
    args.set_loss(fasttext::LossName::SOFTMAX);
    let mut ft_model = FastText::new();
    ft_model.train(&args).unwrap();

    // accuray
    let preds = test_data
        .iter()
        .map(|x| ft_model.predict(x.text.as_str(), 1, 0.));
    let test_labels = test_data.iter().map(|x| x.into_labels());
    let mut hits = 0;
    let mut correct_hits = 0;
    let preds_clone = preds.clone();
    for (predicted, actual) in preds.zip(test_labels) {
        let predicted = predicted?;
        let predicted = &predicted[0];
        if predicted.clone().label == actual {
            correct_hits += 1;
        }
        hits += 1;
    }

    assert_eq!(hits, preds_clone.len());
    println!(
        "accuracy={} ({}/{} correct)",
        correct_hits as f32 / hits as f32,
        correct_hits,
        preds_clone.len()
    );
    ft_model.save_model(MODEL)?;

    Ok(())
}
