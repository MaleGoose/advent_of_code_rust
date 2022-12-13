use std::fs;
use std::str::Lines;

pub(crate) fn solution() -> u32 {
    return read_file().lines().map(|x| calculate_priority(x)).sum()
}

pub(crate) fn solution2() -> u32 {
    return calculate_priority2(read_file().lines())
}

fn read_file() -> String{
    fs::read_to_string("src/input_files/day3.txt").unwrap()
}

fn calculate_priority(line: &str) -> u32{
    let half_point = line.len()/2;
    let first = &line[..half_point].as_bytes();
    let second = &line[half_point..].as_bytes();
    let mut vec: Vec<&u8> = first
        .iter()
        .filter(|x| second.contains(x))
        .collect();
    vec.dedup();
    vec.iter().map(|x| get_priority(x)).sum()
}

fn get_priority(byte: &u8) -> u32{
    if byte.gt(&96){
        return (byte - 96) as u32;
    }
    else if byte.lt(&91) {
        return (byte - 38) as u32;
    }
    return 0;
}

fn calculate_priority2(lines: Lines) -> u32{
    let mut outer_vector = Vec::new();
    let mut inner_vector = Vec::with_capacity(3);
    let mut counter = 0;
    for line in lines {
        if counter == 0{
            counter = counter + 1;
            inner_vector.push(line);
        }
        else if counter == 1 {
            counter = counter + 1;
            inner_vector.push(line);
        }
        else if counter == 2 {
            counter = 0;
            inner_vector.push(line);
            outer_vector.push(inner_vector);
            inner_vector = Vec::with_capacity(3);
        }
    }
    outer_vector.iter().map(|x| get_priority_common(x)).sum()
}

fn get_priority_common(group: &Vec<&str>) -> u32{
    let first = group.get(0).unwrap().as_bytes();
    let second = group.get(1).unwrap().as_bytes();
    let third =group.get(2).unwrap().as_bytes();
    let mut vec: Vec<&u8> = first
        .iter()
        .filter(|x| second.contains(x))
        .filter(|y| third.contains(y))
        .collect();
    vec.dedup();
    vec.iter().map(|x| get_priority(x)).sum()
}
