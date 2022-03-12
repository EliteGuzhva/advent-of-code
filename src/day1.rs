fn descending_window(size: usize, sweep: &Vec<u32>) -> usize {
    sweep
        .windows(size)
        .filter(|window| window[0] < window[size - 1])
        .collect::<Vec<&[u32]>>()
        .len()
}

pub fn solve(data: &mut String) {
    let sweep = aoc::input_to_vec::<u32>(data);

    let res1 = descending_window(2, &sweep);
    println!("Part 1: {}", res1);

    let res2 = descending_window(4, &sweep);
    println!("Part 2: {}", res2);
}
