use std::str::FromStr;

pub fn input_to_vec<T>(data: &String) -> Vec<T>
where
    T: FromStr,
{
    data.lines()
        .filter_map(|line| line.parse::<T>().ok())
        .collect()
}

pub fn input_to_vec2<T, F>(data: &String, f: F) -> Vec<Vec<T>>
where
    T: FromStr,
    F: Fn(char) -> Option<T>,
{
    data.lines()
        .map(|line| line.chars().map(|c| f(c).unwrap()).collect())
        .collect()
}
