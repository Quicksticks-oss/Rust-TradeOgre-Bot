extern crate chrono;
extern crate colored;

use std::{fs::OpenOptions, io::Write};
use chrono::prelude::*;
use colored::*;

fn write_to_log(message: &str) {
    //let market_value = &*MARKET.lock().unwrap();
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("log.txt")
        .expect("Unable to open log file");

    file.write_all(format!("{}\n", message).as_bytes())
        .expect("Unable to write to log file");
}

pub fn info(content: &String) {
    let now = Utc::now();
    let formatted_time = now.format("%Y-%m-%d %H:%M:%S").to_string();
    let final_string = format!(" * {} [INFO] : {}", formatted_time, content);
    println!("{}", final_string);
    write_to_log(final_string.as_str());
}

pub fn warning(content: &String) {
    let now = Utc::now();
    let formatted_time = now.format("%Y-%m-%d %H:%M:%S").to_string();
    let final_string = format!(" * {} [WARN] : {}", formatted_time.yellow(), content.yellow());
    println!("{}", final_string);
    write_to_log(final_string.as_str());
}

pub fn error(content: &String) {
    let now = Utc::now();
    let formatted_time = now.format("%Y-%m-%d %H:%M:%S").to_string();
    let final_string = format!(" * {} [EROR] : {}", formatted_time, content.red());
    println!("{}", final_string);
    write_to_log(final_string.as_str());
}

pub fn good(content: &String) {
    let now = Utc::now();
    let formatted_time = now.format("%Y-%m-%d %H:%M:%S").to_string();
    let final_string = format!(" * {} [POSI] : {}", formatted_time.green(), content.green());
    println!("{}", final_string);
    write_to_log(final_string.as_str());
}

pub fn medium(content: &String) {
    let now = Utc::now();
    let formatted_time = now.format("%Y-%m-%d %H:%M:%S").to_string();
    let final_string = format!(" * {} [MEDI] : {}", formatted_time.yellow(), content.yellow());
    println!("{}", final_string);
    write_to_log(final_string.as_str());
}

pub fn bad(content: &String) {
    let now = Utc::now();
    let formatted_time = now.format("%Y-%m-%d %H:%M:%S").to_string();
    let final_string = format!(" * {} [NEGA] : {}", formatted_time.red(), content.red());
    println!("{}", final_string);
    write_to_log(final_string.as_str());
}
