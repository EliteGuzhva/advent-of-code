mod day1;
mod day2;

use std::fs::File;
use std::io::prelude::*;
use yaml_rust::YamlLoader;
use curl::easy::{Easy, List};

fn main() {
    // Read config
    let mut file = File::open("config.yaml").expect("Cannot open config.yaml");
    let mut config_str = String::new();

    file.read_to_string(&mut config_str).expect("Couldn't read config.yaml");

    let config = YamlLoader::load_from_str(&config_str).unwrap();
    let sesson_id = config[0]["session"].as_str().unwrap();
    let day = config[0]["day"].as_i64().unwrap();

    // Prepare input
    let mut headers = List::new();
    headers.append(std::format!("Cookie: session={}", sesson_id).as_str()).unwrap();

    let mut easy = Easy::new();
    easy.url(std::format!("https://adventofcode.com/2021/day/{}/input", day).as_str()).unwrap();
    easy.http_headers(headers).unwrap();

    let mut data = String::new();
    let mut transfer = easy.transfer();
    transfer.write_function(|raw_data| {
        data = String::from_utf8(raw_data.to_vec()).unwrap();
        Ok(raw_data.len())
    }).unwrap();
    transfer.perform().unwrap();
    drop(transfer);

    // Solve problems
    match day {
        1 => day1::solve(&mut data),
        2 => day2::solve(&mut data),
        _ => println!("Day {} is not implemented yet", day)
    }
}

