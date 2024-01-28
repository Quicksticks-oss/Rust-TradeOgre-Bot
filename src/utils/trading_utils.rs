////////////////////////////////////////////////////////////////
//
//  Utilities for general purpose and testing purposes.
//
////////////////////////////////////////////////////////////////
use crate::lib::vector_store::VectorStore;
use crate::net::csv_download::download_vec_csv_cache;
use crate::utils::reader::{load_csv, load_data};
use crate::logging::fancy_log;
use crate::MAX_STICKS;

use std::time::Instant;
use std::collections::BTreeMap;
use ndarray::Array1;
use csv::Writer;
use std::error::Error;
use std::fs::File;

pub fn test_vecdb(store: &VectorStore) {
    if let Some(vector) = store.get_vector("1") {
        let _ = store.find_similar_vectors(vector.clone(), 4);
    }
}

pub fn initialize_vector_db() -> VectorStore {
    let max_sticks_val = *MAX_STICKS.lock().unwrap();
    download_vec_csv_cache();
    fancy_log::good(&"Cache downloaded or found!".to_string());
    fancy_log::medium(&"Loading vecdb csv data...".to_string());
    let lines: BTreeMap<String, Vec<f32>> = load_csv(".vcache");

    fancy_log::medium(&"Initializing Vector DB...".to_string());
    // Create a vector db.
    let mut store: VectorStore = VectorStore::new();
    store = load_data(lines, store, max_sticks_val);
    fancy_log::good(&"Vector DB loaded.".to_string());

    let start = Instant::now();
    test_vecdb(&store);
    let duration = start.elapsed();
    fancy_log::good(&format!("Time taken to query vec db: {:?}", duration).to_string());

    store
}

pub fn do_vec_calc(
    recent_tickers: &Vec<f64>,
    store: &VectorStore,
    prediction_count: i32,
) -> (Vec<f64>, f64) {
    let prediction_count_usize: usize = prediction_count as usize;
    let vec_array: ndarray::prelude::ArrayBase<
        ndarray::OwnedRepr<_>,
        ndarray::prelude::Dim<[usize; 1]>,
    > = Array1::from_vec(recent_tickers.clone());
    let mut final_vec: Vec<f64> = Vec::new();
    let similar = &store.find_similar_vectors(vec_array.clone(), 1);
    for (id, sim) in similar {
        if let Some(_) = store.get_vector(id) {
            let next_id: i32 = id.parse().expect("Could not parse ID!");
            let next_id_string = &(next_id + prediction_count).to_string();
            if let Some(vector) = store.get_vector(next_id_string) {
                let real_vec = vector.clone();
                let vector_clean = real_vec.into_raw_vec();
                let last_sect: &[f64] =
                    &vector_clean[vector_clean.len() - prediction_count_usize..];
                final_vec = last_sect.to_vec();
                return (final_vec, sim.clone());
            }
        }
    }
    return (final_vec, 0.0);
}

pub fn translate(vector: &Vec<f64>) -> Vec<String> {
    let mut translated: Vec<String> = Vec::new();
    vector.iter().for_each(|value| {
        if *value == 0.0 {
            translated.push("Sell".to_string());
        } else {
            translated.push("Buy".to_string());
        }
    });
    translated
}

pub fn count_more_ones_than_zeros(vec: &Vec<f64>) -> (bool, i32, i32) {
    let mut ones_count: i32 = 0;
    let mut zeros_count: i32 = 0;

    for &item in vec {
        if item == 1.0 {
            ones_count += 1;
        } else if item == 0.0 {
            zeros_count += 1;
        }
    }

    (ones_count >= zeros_count, ones_count, zeros_count)
}

pub fn calculate_slope(data: &Vec<f64>) -> f64 {
    let n = data.len() as f64;
    let mut sum_x = 0.0;
    let mut sum_y = 0.0;
    let mut sum_xy = 0.0;
    let mut sum_x_squared = 0.0;

    for (i, &y) in data.iter().enumerate() {
        let x = i as f64;
        sum_x += x;
        sum_y += y;
        sum_xy += x * y;
        sum_x_squared += x * x;
    }

    let numerator = n * sum_xy - sum_x * sum_y;
    let denominator = n * sum_x_squared - sum_x * sum_x;

    if denominator == 0.0 {
        panic!("Division by zero in slope calculation");
    }

    numerator / denominator
}


pub fn convert_history(history: serde_json::Value) -> Vec<f64> {
    let mut history_tickers: Vec<f64> = Vec::new();
    //let mut last_high: f64 = 0.0;
    let mut last_price: f64 = 0.0;

    if let Some(array) = history.as_array() {
        // Loop through the array
        for item in array {
            let price = item["price"].to_string();
            let price_value = convert_price(price);

            if last_price == 0.0 {
                last_price = price_value;
            }

            let mut his_price = 0.0;
            if price_value >= last_price {
                his_price = 1.0;
            }
            history_tickers.push(his_price);
        }
    } else {
        // Handle the case where the data is not an array
        println!("The provided JSON is not an array.");
    }
    history_tickers
}

pub fn convert_price(price: String) -> f64 {
    let base_price = price.replace("\"", "");
    let price_val: f64 = base_price.parse().expect("Not a valid number");
    return price_val;
}

pub fn save_to_csv(
    prices: &Vec<f64>,
    actions: &Vec<String>,
    file_path: &str,
) -> Result<(), Box<dyn Error>> {
    if prices.len() != actions.len() {
        return Err(From::from(
            "Prices and actions vectors must be of the same length.",
        ));
    }

    let file = File::create(file_path)?;
    let mut wtr = Writer::from_writer(file);

    for (count, (price, action)) in prices.iter().zip(actions.iter()).enumerate() {
        wtr.write_record(&[count.to_string(), price.to_string(), action.clone()])?;
    }

    wtr.flush()?;
    Ok(())
}

