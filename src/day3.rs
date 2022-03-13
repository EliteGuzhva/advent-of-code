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
        .into_iter()
        .map(|d| if d > half { 1 } else { 0 })
        .fold(0, |res, digit| (res << 1) + digit);

    let epsilon = !gamma << 32 - number_length >> 32 - number_length;
    let res = gamma * epsilon;
    println!("Part 1: {}", res);
}
