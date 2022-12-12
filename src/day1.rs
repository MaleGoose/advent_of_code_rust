use std::fs;

pub(crate) fn solution() -> u32{
    let file_string: String = fs::read_to_string("src/input_files/day1.txt").unwrap();
    let vector = file_string.split("\n\r").collect::<Vec<&str>>();

    let mut values: Vec<u32> = vector
        .iter()
        .map(|x| sum(x))
        .collect::<Vec<u32>>();

    values.sort_by(|a, b| b.cmp(a));

    let mut value = 0;
    for i in 0..=2{
        value = value + values.get(i).unwrap();
    }
    return value;
}

fn sum(lines: &str) -> u32{
    let vector: Vec<&str> = lines.lines().collect();
    vector.iter()
        .filter(|s| !s.is_empty())
        .map(|x| x.parse::<u32>().unwrap())
        .sum()
}