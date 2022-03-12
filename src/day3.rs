// use std::str::FromStr;

pub fn solve(data: &mut String) {
    // *data = String::from_str("1000\n1101\n0111\n1111\n").unwrap();

    let numbers: Vec<&str> = data.lines().collect();
    let number_length = numbers[0].len();

    let mut num_ones: Vec<i32> = Vec::new();
    num_ones.resize(number_length, 0);

    for number in &numbers {
        let mut i = 0;
        for c in number.chars().into_iter() {
            if c == '1' {
                num_ones[i] += 1;
            }

            i += 1;
        }
    }

    let mut gamma = String::new();
    let mut epsilon = String::new();

    for o in num_ones {
        if o > (number_length / 2).try_into().unwrap() {
            gamma += "1";
            epsilon += "0";
        } else {
            gamma += "0";
            epsilon += "1";
        }
    }

    println!("{}\n{}", gamma, epsilon);

    let g = i32::from_str_radix(&gamma, 2).unwrap();
    let e = i32::from_str_radix(&epsilon, 2).unwrap();
    let res = g * e;

    println!("{} * {} = {}", g, e, res);
}
