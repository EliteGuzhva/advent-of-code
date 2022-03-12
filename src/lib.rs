use std::str::FromStr;

pub fn input_to_vec<T>(data: &String) -> Vec<T>
where
    T: FromStr,
{
    data.lines()
        .filter_map(|line| line.parse::<T>().ok())
        .collect()
}
