////////////////////////////////////////////////////////////////
//
//  A script to control traiding apis like Trade Ogre and HTMW.
//
////////////////////////////////////////////////////////////////
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(deprecated)]
use reqwest;
use reqwest::header::AUTHORIZATION;
use serde_json::{json, Value};
use std::{thread, time};
use csv::ReaderBuilder;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use base64;


const MAX_RETRIES: u32 = 5; // Maximum number of retries
const RETRY_DELAY: u64 = 3000; // Delay between retries in milliseconds

#[derive(Debug, Deserialize, Serialize)]
struct Record {
    Open: String,  // Assuming 'Open' is a string field in your CSV
    // Add other fields from your CSV here
}

#[derive(Serialize)]
struct SimpleRecord {
    price: String,
}

pub struct BackTest {
    data: Vec<Record>,
    current_index: usize,
}

#[derive(Serialize)]
pub struct BuyOrder {
    market: String,
    quantity: f64,
    price: f64,
}

#[derive(Deserialize)]
pub struct OrderResponse {
    success: bool,
    uuid: Option<String>,
    bnewbalavail: Option<String>,
    snewbalavail: Option<String>,
}

pub struct TradeOgre {}

static BASE_URL: &str = "https://tradeogre.com/api/v1";
//static BASE_URL: &str = "http://127.0.0.1:5000";

impl TradeOgre {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_balances(&self, currency: &str, public_key: &str, private_key: &str) -> Value {
        let url = format!("{BASE_URL}/account/balances");
        let client = reqwest::blocking::Client::new();
        // Prepare the basic authentication header
        let auth_header_value = format!("Basic {}", base64::encode(format!("{}:{}", public_key, private_key)));

        let max_attempts = 5;
        for attempt in 0..max_attempts {
            match client.get(&url).header(AUTHORIZATION, &auth_header_value).send() {
                Ok(response) => {
                    match response.json::<Value>() {
                        Ok(json) => {
                            for (key, value) in json["balances"].as_object().unwrap() {
                                if key == currency {
                                    return value.clone();
                                }
                            }
                            return json
                        },
                        Err(_) => {
                            if attempt == max_attempts - 1 {
                                return json!({ "error": "Failed to parse JSON response" });
                            }
                        }
                    }
                },
                Err(e) => {
                    eprintln!("Request failed: {}. Attempt {} of {}", e, attempt + 1, max_attempts);
                    if attempt == max_attempts - 1 {
                        return json!({ "error": format!("Failed to get ticker: {}", e) });
                    }
                }
            }
            thread::sleep(time::Duration::from_secs(RETRY_DELAY));
        }
    
        json!({ "price": "0.0" })
    }

    pub fn sell(&self, market: &str, quantity: &str, price: &str, public_key: &str, private_key: &str) -> Value {
        let url = format!("{BASE_URL}/order/sell");
        let client = reqwest::blocking::Client::new();
        // Prepare the basic authentication header
        let auth_header_value = format!("Basic {}", base64::encode(format!("{}:{}", public_key, private_key)));
        // Define the POST data
        let data = vec![
            ("market", market),
            ("quantity", quantity),
            ("price", price),
        ];
        let max_attempts = 5;
        for attempt in 0..max_attempts {
            match client.post(&url).header(AUTHORIZATION, &auth_header_value).form(&data).send() {
                Ok(response) => {
                    match response.json::<Value>() {
                        Ok(json) => {
                            return json
                        },
                        Err(_) => {
                            if attempt == max_attempts - 1 {
                                return json!({ "error": "Failed to parse JSON response" });
                            }
                        }
                    }
                },
                Err(e) => {
                    eprintln!("Request failed: {}. Attempt {} of {}", e, attempt + 1, max_attempts);
                    if attempt == max_attempts - 1 {
                        return json!({ "error": format!("Failed to get ticker: {}", e) });
                    }
                }
            }
            thread::sleep(time::Duration::from_secs(RETRY_DELAY));
        }
    
        json!({ "price": "0.0" })
    }

    pub fn buy(&self, market: &str, quantity: &str, price: &str, public_key: &str, private_key: &str) -> Value {
        let url = format!("{BASE_URL}/order/buy");
        let client = reqwest::blocking::Client::new();
        // Prepare the basic authentication header
        let auth_header_value = format!("Basic {}", base64::encode(format!("{}:{}", public_key, private_key)));
        // Define the POST data
        println!("{}", market);
        let data = format!("market={}&price={}&quantity={}", market, price, quantity);
    
        let max_attempts = 5;
        for attempt in 0..max_attempts {
            match client.post(&url).body(data.clone()).header("Content-Type", "application/x-www-form-urlencoded").header(AUTHORIZATION, &auth_header_value).send() {
                Ok(response) => {
                    match response.json::<Value>() {
                        Ok(json) => {
                            return json
                        },
                        Err(_) => {
                            if attempt == max_attempts - 1 {
                                return json!({ "error": "Failed to parse JSON response" });
                            }
                        }
                    }
                },
                Err(e) => {
                    eprintln!("Request failed: {}. Attempt {} of {}", e, attempt + 1, max_attempts);
                    if attempt == max_attempts - 1 {
                        return json!({ "error": format!("Failed to get ticker: {}", e) });
                    }
                }
            }
            thread::sleep(time::Duration::from_secs(RETRY_DELAY));
        }
    
        json!({ "price": "0.0" })
    }

    pub fn get_ticker(&self, market: &str) -> Value {
        let url = format!("{BASE_URL}/ticker/{market}");
        let client = reqwest::blocking::Client::new();
    
        let max_attempts = 5;
        for attempt in 0..max_attempts {
            match client.get(&url).send() {
                Ok(response) => {
                    match response.json::<Value>() {
                        Ok(json) => return json,
                        Err(_) => {
                            if attempt == max_attempts - 1 {
                                return json!({ "error": "Failed to parse JSON response" });
                            }
                        }
                    }
                },
                Err(e) => {
                    eprintln!("Request failed: {}. Attempt {} of {}", e, attempt + 1, max_attempts);
                    if attempt == max_attempts - 1 {
                        return json!({ "error": format!("Failed to get ticker: {}", e) });
                    }
                }
            }
            thread::sleep(time::Duration::from_secs(RETRY_DELAY));
        }
    
        json!({ "price": "0.0" })
    }

    pub fn get_history(&self, market: &str) -> Result<Value, reqwest::Error> {
        let url: String = format!("{BASE_URL}/history/{market}");
        let client = reqwest::blocking::Client::new();
    
        for attempt in 0..MAX_RETRIES {
            match client.get(&url).send() {
                Ok(response) => {
                    match response.json::<Value>() {
                        Ok(json) => return Ok(json),
                        Err(e) => return Err(e),
                    }
                }
                Err(e) => {
                    if attempt < MAX_RETRIES - 1 {
                        println!("Attempt {} failed, retrying in {} seconds...", attempt + 1, RETRY_DELAY / 1000);
                        thread::sleep(time::Duration::from_millis(RETRY_DELAY));
                    } else {
                        return Err(e);
                    }
                }
            }
        }
    
        unreachable!() // This should never be reached due to the return statements in the loop
    }
}

impl BackTest {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let file_path = "BTC.csv";  // replace 'BTC.csv' with your CSV file path
        let data = load_csv_data(file_path)?;
        Ok(Self {
            data,
            current_index: 25,
        })
    }

    pub fn get_ticker(&mut self, _: &str) -> Value {
        if self.current_index < self.data.len() {
            let record = &self.data[self.current_index];
            self.current_index += 1;

            serde_json::to_value(&SimpleRecord { price: record.Open.clone() })
                .unwrap_or(Value::Null)
        } else {
            println!("The script will now exit it has completed back testing...");
            panic!();
        }
    }

    pub fn get_history(&self, _: &str) -> Result<Value, Box<dyn Error>> {
        let start_index = if self.current_index >= 50 { self.current_index - 50 } else { 0 };
        let history: Vec<SimpleRecord> = self.data[start_index..self.current_index]
            .iter()
            .map(|record| SimpleRecord { price: record.Open.clone() })
            .collect();

        serde_json::to_value(&history)
            .map_err(|e| e.into())
    }

    // You can add more methods as needed
}

// Load CSV data
fn load_csv_data(file_path: &str) -> Result<Vec<Record>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().from_reader(file);
    let mut records = Vec::new();
    for result in rdr.deserialize() {
        let record: Record = result?;
        records.push(record);
    }
    Ok(records)
}