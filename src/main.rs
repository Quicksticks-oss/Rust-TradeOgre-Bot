mod lib {
    pub mod vector_store;
}

mod logging {
    pub mod fancy_log;
}

mod net {
    pub mod csv_download;
    pub mod trading_api;
}

mod utils {
    pub mod load_config;
    pub mod reader;
    pub mod trading_utils;
}

use lib::vector_store::VectorStore;
use logging::fancy_log;
use net::trading_api::TradeOgre;
use utils::load_config::{generate_toml, load_toml};
use utils::trading_utils::*;

use once_cell::sync::Lazy;
use std::env;
use std::process;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use std::io;

pub static THRESHOLD: Mutex<f32> = Mutex::new(0.95);
pub static SLOPE_THRESHOLD: Mutex<f64> = Mutex::new(-10.0);
pub static SLOPE_MAX_STICKS: Mutex<usize> = Mutex::new(5);
pub static MAX_STICKS: Mutex<i32> = Mutex::new(16);
pub static PREDICTION_COUNT: Mutex<i32> = Mutex::new(7);
pub static API_SLEEP: Mutex<u64> = Mutex::new(512);
pub static MARKET: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::from("USDT-BTC")));
pub static API_KEY: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::from("null")));
pub static API_SECRET: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::from("null")));

static LOGO: &str = "

$$$$$$$$\\                       $$\\            $$$$$$\\                                      $$$$$$$\\             $$\\     
\\__$$  __|                      $$ |          $$  __$$\\                                     $$  __$$\\            $$ |    
   $$ | $$$$$$\\  $$$$$$\\   $$$$$$$ | $$$$$$\\  $$ /  $$ | $$$$$$\\   $$$$$$\\   $$$$$$\\        $$ |  $$ | $$$$$$\\ $$$$$$\\   
   $$ |$$  __$$\\ \\____$$\\ $$  __$$ |$$  __$$\\ $$ |  $$ |$$  __$$\\ $$  __$$\\ $$  __$$\\       $$$$$$$\\ |$$  __$$\\\\_$$  _|  
   $$ |$$ |  \\__|$$$$$$$ |$$ /  $$ |$$$$$$$$ |$$ |  $$ |$$ /  $$ |$$ |  \\__|$$$$$$$$ |      $$  __$$\\ $$ /  $$ | $$ |    
   $$ |$$ |     $$  __$$ |$$ |  $$ |$$   ____|$$ |  $$ |$$ |  $$ |$$ |      $$   ____|      $$ |  $$ |$$ |  $$ | $$ |$$\\ 
   $$ |$$ |     \\$$$$$$$ |\\$$$$$$$ |\\$$$$$$$\\  $$$$$$  |\\$$$$$$$ |$$ |      \\$$$$$$$\\       $$$$$$$  |\\$$$$$$  | \\$$$$  |
   \\__|\\__|      \\_______| \\_______| \\_______| \\______/  \\____$$ |\\__|       \\_______|      \\_______/  \\______/   \\____/ 
                                                        $$\\   $$ |                                                       
                                                        \\$$$$$$  |                                                       
                                                         \\______/      

                                                    By: Quicksticks-oss             

";

fn get_first_arg() -> String {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("No argument provided. Please use the program like this:");
        println!("[\"config file path\", --generate-config]");
        println!("Press Enter to quit...");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        process::exit(0);
    }

    let first_arg = &args[1];
    first_arg.to_string()
}

fn config_loader() {
    let config_file = get_first_arg();

    if config_file == "--generate-config" {
        generate_toml();
        fancy_log::good(&"Saved config to config.toml".to_string());
    }
    let config = load_toml(config_file);

    let mut market = MARKET.lock().unwrap();
    *market = config.market;

    let mut api_key_val = API_KEY.lock().unwrap();
    *api_key_val = config.api_key;
    let mut api_secret_val = API_SECRET.lock().unwrap();
    *api_secret_val = config.api_secret;


    let mut api_slp = API_SLEEP.lock().unwrap();
    *api_slp = config.api_sleep;
    let mut max_sticks_val = MAX_STICKS.lock().unwrap();
    *max_sticks_val = config.max_sticks;
    let mut slope_max_sticks_val = SLOPE_MAX_STICKS.lock().unwrap();
    *slope_max_sticks_val = config.slope_max_sticks;
    let mut slope_threshold_val = SLOPE_THRESHOLD.lock().unwrap();
    *slope_threshold_val = config.slope_threshold;
    let mut threshold_val = THRESHOLD.lock().unwrap();
    *threshold_val = config.threshold;

    fancy_log::info(&format!("Market: {}", *market).to_string());
    fancy_log::info(&format!("API Sleep: {}", *api_slp).to_string());
    fancy_log::info(&format!("Max Sticks: {}", *max_sticks_val).to_string());
    fancy_log::info(&format!("Max Slope Sticks: {}", *slope_max_sticks_val).to_string());
    fancy_log::info(&format!("Slope Threshold: {}", *slope_threshold_val).to_string());
    fancy_log::info(&format!("Threshold: {}", *threshold_val).to_string());
}

fn main() {
    fancy_log::good(&LOGO.to_string());
    fancy_log::good(&"Starting TradeOgre Bot...".to_string());

    config_loader();

    let store: VectorStore = initialize_vector_db();

    fancy_log::medium(&"Initializing trading API...".to_string());

    let market_mul = &*MARKET.lock().unwrap();
    let market_val: &str = market_mul.as_str();

    let prediction_val = *PREDICTION_COUNT.lock().unwrap();
    let max_sticks_val = *MAX_STICKS.lock().unwrap();
    let slope_max_sticks_val = *SLOPE_MAX_STICKS.lock().unwrap();
    let slope_threshold = *SLOPE_THRESHOLD.lock().unwrap();
    let threshold = *THRESHOLD.lock().unwrap();
    let api_key_val = &*API_KEY.lock().unwrap();
    let api_sec_val = &*&API_SECRET.lock().unwrap();

    let api_key_priv: &str = api_key_val.as_str();
    let api_key_secret : &str = api_sec_val.as_str();

    let trade_api = TradeOgre::new();
    //let mut trade_api = BackTest::new().expect("ad");

    let curencys: Vec<&str> = market_val.split("-").collect();
    let curency_buy = curencys.get(0).expect("Could get currency for purchase.");
    let curency_sell = curencys.get(1).expect("Could get currency for selling.");

    let history: serde_json::Value = trade_api
        .get_history(market_val)
        .expect("Error getting history!");

    let history_tickers = convert_history(history);
    let max_sticks_usize_value: usize = max_sticks_val as usize;
    let mut recent_tickers: Vec<f64> = history_tickers
        .iter()
        .rev()
        .take(max_sticks_usize_value)
        .cloned()
        .collect();

    fancy_log::good(&"Trading will now begin!".to_string());
    let mut last_price = 0.0;
    let mut last_action: String = String::new();
    let mut last_purchase_price = 0.0;

    let mut fake_balance: f64 = 60.0;
    let mut fake_balance_absolute: f64 = 60.0;
    let mut fake_stocks: f64 = 0.0;

    let mut all_prices: Vec<f64> = Vec::new();
    let mut all_actions: Vec<String> = Vec::new();

    let mut trades_count: i32 = 0;
    fancy_log::good(&"=========================".to_string());
    loop {
        let ticker = &trade_api.get_ticker(market_val);
        let price: String = ticker["price"].to_string();
        let price_fp: f64 = convert_price(price);
        if last_price != price_fp {
            if fake_balance_absolute == 60.0 {
                fake_balance_absolute = fake_balance_absolute / price_fp;
                last_purchase_price = price_fp + 1.0;
            }

            let mut his_price = 0.0;
            if price_fp >= last_price {
                his_price = 1.0;
            }

            recent_tickers.push(his_price);
            recent_tickers = recent_tickers
                .iter()
                .rev()
                .take(max_sticks_usize_value)
                .cloned()
                .collect();
            recent_tickers = recent_tickers.into_iter().rev().collect();

            let (pred_vec, mut accuracy) = do_vec_calc(&recent_tickers, &store, prediction_val);
            let (mut buy_sell, _, _): (bool, i32, i32) = count_more_ones_than_zeros(&pred_vec);
            let _ = translate(&pred_vec);

            let last_slope: Vec<f64> = if all_prices.len() > slope_max_sticks_val {
                all_prices[all_prices.len() - slope_max_sticks_val..].to_vec()
            } else {
                all_prices.clone()
            };

            let mut slope = 0.0;
            if all_prices.len() > 1 {
                slope = calculate_slope(&last_slope);
            }

            let mut _buy_sell_hold: String = "hold".to_string();
            if buy_sell && slope > slope_threshold {
                _buy_sell_hold = "buy".to_string();
            } else if price_fp > last_purchase_price {
                _buy_sell_hold = "sell".to_string();
            }

            let msg = &format!(
                "Price: {:.2}, Similarities: {:.4}, Slope: {:.0}, Action: {}",
                price_fp, accuracy, slope, _buy_sell_hold
            );
            fancy_log::info(msg);

            if price_fp > last_purchase_price*1.15 && accuracy < threshold as f64 && last_action == "buy"{
                fancy_log::good(&"Selling because of bull!".to_string());
                accuracy = 1.0;
                _buy_sell_hold = "sell".to_string();
                buy_sell = false;
            } else if price_fp < last_purchase_price*0.85 && accuracy < threshold as f64 && last_action == "buy"{
                fancy_log::bad(&"Selling because of bear!".to_string());
                accuracy = 1.0;
                _buy_sell_hold = "sell".to_string();
                buy_sell = false;
            }

            if last_action != _buy_sell_hold && accuracy > threshold as f64 {
                if accuracy == 0.0 {
                    fancy_log::error(&"Could not process similarities!".to_string());
                }
                if buy_sell {
                    last_purchase_price = price_fp;
                    if fake_balance > 0.0 {
                        fake_stocks = fake_balance / price_fp;
                        fake_balance = 0.0;
                    }
                    fancy_log::good(
                        &format!(
                            "Trades: {}, Balance: {:.2}, Ride: {:.2}",
                            trades_count,
                            fake_stocks * price_fp,
                            fake_balance_absolute * price_fp
                        )
                        .to_string(),
                    );
                    trades_count += 1;
                } else {
                    last_purchase_price = 0.0;
                    if fake_stocks > 0.0 {
                        fake_balance = fake_stocks * price_fp;
                        fake_stocks = 0.0;
                    }
                    fancy_log::info(
                        &format!(
                            "Trades: {}, Balance: {:.2}, Ride: {:.2}",
                            trades_count,
                            fake_balance,
                            fake_balance_absolute * price_fp
                        )
                        .to_string(),
                    );
                }

                if slope == 0.0 {
                    fancy_log::warning(&"Slope is 0.0, most likely start of program.".to_string());
                }

                trades_count += 1;

                if _buy_sell_hold == "buy".to_string() {
                    let msg = format!("Purchased {} at price {:.2}!", market_val, price_fp);
                    fancy_log::good(&msg.to_string());

                    let bal = trade_api.get_balances(&curency_buy, api_key_priv, api_key_secret);
                    let ask_price: String = ticker["ask"].to_string();
                    let ask_fp: f64 = convert_price(ask_price);
                    let bal_num = (((convert_price(bal.to_string())/ask_fp)*100000.0).floor())/100000.0;
                    let bal_val = &format!("{}", bal_num);
                    let mkt = &format!("{}-{}", curency_sell, curency_buy);
                    let ret = trade_api.buy(mkt, bal_val, &ask_fp.to_string(), &api_key_val, &api_sec_val);
                    
                    fancy_log::good(&format!("{}, {:?}, Quant: {}", ret, bal_num, bal_val));
                } else if _buy_sell_hold == "hold".to_string() {
                    let msg = format!("Holding {} at price {:.2}!", market_val, price_fp).to_string();
                    fancy_log::medium(
                        &msg,
                    );
                } else if _buy_sell_hold == "sell".to_string() {
                    let msg = format!("Selling {} at price {:.2}!", market_val, price_fp).to_string();
                    fancy_log::bad(&msg);

                    let bal = trade_api.get_balances(&curency_sell, api_key_priv, api_key_secret);
                    let ask_price: String = ticker["bid"].to_string();
                    let ask_fp: f64 = convert_price(ask_price);
                    let mkt = &format!("{}-{}", curency_sell, curency_buy);
                    let ret = trade_api.sell(mkt, &bal.to_string(), &ask_fp.to_string(), &api_key_val, &api_sec_val);
                    
                    fancy_log::bad(&format!("{},{}", ret, bal));
                }

                last_action = _buy_sell_hold.clone();
            } else {
                _buy_sell_hold = "hold".to_string();
            }
            all_actions.push(_buy_sell_hold.clone());
            all_prices.push(price_fp);
            save_to_csv(&all_prices, &all_actions, "log.csv").expect("Issue saving to csv.");
        }
        last_price = price_fp;

        let delay: u64 = *API_SLEEP.lock().unwrap();
        thread::sleep(Duration::from_millis(delay.clone()));
    }
}
