////////////////////////////////////////////////////////////////
//
//  Simple data processing script for CSVs and norminalization.
//
////////////////////////////////////////////////////////////////
use std::collections::BTreeMap;
use std::fs::File;

use ndarray::Array1;

use crate::lib::vector_store::VectorStore;

pub fn load_csv(path: &str) -> BTreeMap<String, Vec<f32>> {
    let file_path = path.to_string();
    let file = File::open(file_path).expect("File does not exist.");
    let mut rdr = csv::Reader::from_reader(file);
    let mut records: BTreeMap<String, Vec<f32>> = BTreeMap::new();
    for result in rdr.records() {
        match result {
            Ok(string_record) => {
                let date: &str = &string_record[0];

                let open: Result<f32, _> = string_record[1].parse();
                let high: Result<f32, _> = string_record[2].parse();
                let low: Result<f32, _> = string_record[3].parse();
                let close: Result<f32, _> = string_record[4].parse();

                let open_value: f32 = open.expect("open error");
                let high_value: f32 = high.expect("high error");
                let low_value: f32 = low.expect("low error");
                let close_value: f32 = close.expect("close error");

                let mut record_vec = Vec::new();

                record_vec.push(open_value);
                record_vec.push(high_value);
                record_vec.push(low_value);
                record_vec.push(close_value);

                records.insert(date.to_string(), record_vec);
            }
            Err(e) => {
                // Handle the error
                println!("An error occurred: {}", e);
            }
        }
    }
    records
}

pub fn load_data(
    lines: BTreeMap<String, Vec<f32>>,
    mut store: VectorStore,
    max_sticks: i32,
) -> VectorStore {
    let mut count: i32 = 0;
    let mut idx: i32 = 0;
    let mut section: Vec<f64> = Vec::new();
    let mut high_value: f64 = 0.0;
    let mut last_price: f64 = 0.0;
    let max_sticks_usize_value: usize = max_sticks as usize;
    for (_, value) in lines.iter() {
        let open: f64 = value[0].into();
        let high: f64 = value[1].into();

        if high_value == 0.0 {
            high_value = high
        }

        let mut his_price = 0.0;
        if open >= last_price {
            his_price = 1.0;
        }
        section.push(his_price);
        last_price = open;
        
        count += 1;
        if count >= max_sticks {
            section = section.iter().rev().take(max_sticks_usize_value).cloned().collect();
            section = section.into_iter().rev().collect();

            let vector = Array1::from_vec(section.clone());
            let name: String = idx.to_string();

            store.add_vector(name, vector);

            high_value = high;
            idx += 1;
            count = max_sticks-1;
        }
    }
    store
}
