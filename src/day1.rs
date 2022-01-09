pub fn solve(data: &mut String) {
    let sweep: Vec<i32> = data.split('\n').into_iter()
        .filter_map(|v| v.parse::<i32>().ok())
        .collect();

    // Part 1
    let mut counter = 0;
    let mut prev = sweep[0];
    for value in &sweep {
        if prev > *value {
            counter += 1;
        }

        prev = *value;
    }

    println!("1. Answer: {}", counter);

    // Part 2
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

    println!("2. Answer: {}", counter);
}

