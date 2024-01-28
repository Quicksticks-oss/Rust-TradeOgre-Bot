use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::process;
use std::fs;
use std::io;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub threshold: f32,
    pub slope_threshold: f64,
    pub slope_max_sticks: usize,
    pub max_sticks: i32,
    pub prediction_count: i32,
    pub api_sleep: u64, // Assuming this is in milliseconds or similar
    pub market: String,
    pub api_key: String,
    pub api_secret: String,
}

pub fn load_toml(config_file: String) -> Config {
    let toml_str = fs::read_to_string(config_file).expect("Failed to read TOML file");
    let config: Config = toml::from_str(&toml_str).expect("Failed to deserialize TOML");
    config
}

pub fn generate_toml() {
    let config = Config {
        threshold: 0.95,
        slope_threshold: -10.0,
        slope_max_sticks: 5,
        max_sticks: 16,
        prediction_count: 7,
        api_sleep: 512,
        market: "USDT-BTC".to_string(),
        api_key: "Change this to your API key which can be found in your \"https://tradeogre.com/account/settings\".Follow \"https://tradeogre.com/help/api\" for more help.".to_string(),
        api_secret: "Change this to your API secret which can be found in your \"https://tradeogre.com/account/settings\".Follow \"https://tradeogre.com/help/api\" for more help.".to_string(),
    };

    let toml_string = toml::to_string(&config).expect("Failed to serialize");

    let mut file = File::create("config.toml").expect("Failed to create file");
    file.write_all(toml_string.as_bytes()).expect("Failed to write to file");
    println!("Saved config to config.toml, use config.toml as the argument to start.");
    println!("Press Enter to quit...");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    process::exit(0);
}
