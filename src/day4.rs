use std::fs;
use std::ops::{RangeInclusive};

pub(crate) fn solution() -> u32 {
    return read_file().lines().map(|x| calculate_score(x)).sum()
}

pub(crate) fn solution2() -> u32 {
    return read_file().lines().map(|x| calculate_score2(x)).sum()
}

fn read_file() -> String{
    fs::read_to_string("src/input_files/day4.txt").unwrap()
}

fn calculate_score(line: &str) -> u32 {
    return if contained_entirely(line) {
        1
    } else {
        0
    }
}

fn calculate_score2(line: &str) -> u32 {
    return if contained_partly(line) {
        1
    } else {
        0
    }
}

fn contained_entirely(line: &str) -> bool{
    let split: Vec<&str> = line.split(",").collect();
    let first_range: RangeInclusive<u32> = to_range(split.get(0).unwrap().split("-").collect());
    let second_range: RangeInclusive<u32> = to_range(split.get(1).unwrap().split("-").collect());
    return if
        first_range.contains(&second_range.clone().min().unwrap()) &&
        first_range.contains(&second_range.clone().max().unwrap()) ||
        second_range.contains(&first_range.clone().min().unwrap()) &&
        second_range.contains(&first_range.clone().max().unwrap()) {
        true
    } else {
        false
    }
}
fn contained_partly(line: &str) -> bool{
    let split: Vec<&str> = line.split(",").collect();
    let first_range: RangeInclusive<u32> = to_range(split.get(0).unwrap().split("-").collect());
    let second_range: RangeInclusive<u32> = to_range(split.get(1).unwrap().split("-").collect());

    for first_value in first_range.clone(){
        if second_range.contains(&first_value){
            return true
        }
    }

    for second_value in second_range.clone(){
        if first_range.contains(&second_value){
            return true
        }
    }
    return false;

}

fn to_range(vector: Vec<&str>) -> RangeInclusive<u32> {
    vector.get(0).unwrap().parse::<u32>().unwrap()
    ..=
    vector.get(1).unwrap().parse::<u32>().unwrap()
}