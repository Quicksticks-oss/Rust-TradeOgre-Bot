use reqwest;
use std::fs::File;
use std::path::Path;

static VEC_CSV: &str = "https://raw.githubusercontent.com/Quicksticks-oss/stock-csv/main/converted_aapl.csv";

pub fn download_file(url: &str, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Send a GET request
    let response = reqwest::blocking::get(url)?;

    // Ensure the request was successful
    let mut content = response.error_for_status()?;

    // Open a file in write-only mode
    let mut file = File::create(file_path)?;

    // Write the response content to the file
    std::io::copy(&mut content, &mut file)?;

    Ok(())
}

pub fn download_vec_csv_cache() {
    let file_path = ".vcache";
    if !Path::new(file_path).exists() {
        download_file(VEC_CSV, file_path).expect("Could not download file.");
    }
}
