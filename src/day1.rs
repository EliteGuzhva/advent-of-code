pub fn solve(data: &mut String) {
    let sweep: Vec<i32> = data.split('\n').into_iter()
        .filter_map(|v| v.parse::<i32>().ok())
        .collect();

    let mut counter = 0;
    let mut prev = 0;
    for (i, value) in sweep.iter().enumerate() {
        if i < 3 {
            prev += *value;
        } else {
            let cur = prev - sweep[i - 3] + *value;
            if cur > prev {
                counter += 1;
            }

            prev = cur;
        }
    }

    println!("Answer: {}", counter);
}

