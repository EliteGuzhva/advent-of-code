use curl::easy::{Easy, List};

fn main() {
    let mut headers = List::new();
    headers.append("Cookie: session=53616c7465645f5f598d394b71e013a53dffdcaf219d3b0a5703e40be38845f15204acf20f0b9c9d6d81807cecfb54fb").unwrap();

    let mut easy = Easy::new();
    easy.url("https://adventofcode.com/2021/day/1/input").unwrap();
    easy.http_headers(headers).unwrap();

    let mut sweep = Vec::<i32>::new();

    let mut transfer = easy.transfer();
    transfer.write_function(|data| {
        let data_str = String::from_utf8(data.to_vec()).unwrap();
        sweep = data_str.split('\n').into_iter()
            .filter_map(|v| v.parse::<i32>().ok())
            .collect();
        Ok(data.len())
    }).unwrap();
    transfer.perform().unwrap();
    drop(transfer);

    let mut counter = 0;
    let mut prev = sweep[0];
    for value in sweep {
        if value > prev {
            counter += 1;
        }
        prev = value;
    }

    println!("Answer: {}", counter);
}
