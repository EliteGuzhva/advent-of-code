pub fn solve(data: &mut String) {
    let numbers = aoc::input_to_vec2(data, |c| char::to_digit(c, 2));
    let number_length = numbers[0].len();
    let half = numbers.len() as u32 / 2;

    let mut sums = vec![0; number_length];
    numbers.iter().for_each(|number| {
        for i in 0..number_length {
            sums[i] += number[i];
        }
    });

    let gamma: u32 = sums
        .iter()
        .map(|d| if d > &half { 1 } else { 0 })
        .fold(0, |res, digit| (res << 1) + digit);

    let epsilon = !gamma << 32 - number_length >> 32 - number_length;
    let res = gamma * epsilon;
    println!("Part 1: {}", res);

    let filter_numbers = |f: fn(u32, u32) -> bool| {
        let mut values = numbers.clone();
        for i in 0..number_length {
            let current_bit_values: Vec<u32> = values.iter().map(|digits| digits[i]).collect();
            let current_half = (current_bit_values.len() as u32 + 1) / 2;
            let bit_sum: u32 = current_bit_values.iter().sum();
            let most_common = if bit_sum >= current_half { 1 } else { 0 };

            values = values
                .into_iter()
                .filter(|value| f(value[i], most_common))
                .collect();

            if values.len() == 1 {
                break;
            }
        }

        values[0].iter().fold(0, |res, digit| (res << 1) + digit)
    };

    let oxygen_res = filter_numbers(|d, c| d == c);
    let co2_res = filter_numbers(|d, c| d != c);

    let res = oxygen_res * co2_res;

    println!("Part 2: {}", res);
}
