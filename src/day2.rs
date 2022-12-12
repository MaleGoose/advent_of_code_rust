use std::fs;

pub(crate) fn solution() -> u32 {
    return read_file().lines().map(|x| calculate_score(x)).sum()
}

pub(crate) fn solution2() -> u32 {
    return read_file().lines().map(|x| calculate_score2(x)).sum()
}

fn read_file() -> String{
    fs::read_to_string("src/input_files/day2.txt").unwrap()
}
fn calculate_score(term: &str) -> u32{
    let vector: Vec<&str> = term.split_whitespace().collect();
    let my_pick = vector.get(1).unwrap();
    let battle_val = battle(vector.first().unwrap(), my_pick);
    return match my_pick {
        &"X" => 1 + battle_val,
        &"Y" => 2 + battle_val,
        &"Z" => 3 + battle_val,
        _ => 0
    }
}

fn calculate_score2(term: &str) -> u32{
    let vector: Vec<&str> = term.split_whitespace().collect();
    let outcome = vector.get(1).unwrap();
    let my_pick = get_my_pick(vector.first().unwrap() ,outcome);
    match outcome{
        &"X" => 0 + my_pick,
        &"Y" => 3 + my_pick,
        &"Z" => 6 + my_pick,
        _ => 0,
    }
}

fn battle(first: &str, second:&str) -> u32{
    if first == "A" && second == "X" ||
        first == "B" && second == "Y" ||
        first == "C" && second == "Z"{
        return 3;
    }
    if first == "A" && second == "Y" ||
        first == "B" && second == "Z" ||
        first == "C" && second == "X" {
        return 6;
    }
    return 0;
}

fn get_my_pick(other: &str, outcome: &str) -> u32{
    if outcome.eq("X") {
        return match other{
            "A" => 3,
            "B" => 1,
            "C" => 2,
            _ => 0,
        }
    }
    else if outcome.eq("Y") {
        return match other{
            "A" => 1,
            "B" => 2,
            "C" => 3,
            _ => 0,
        }
    }
    else if outcome.eq("Z") {
        return match other{
            "A" => 2,
            "B" => 3,
            "C" => 1,
            _ => 0,
        }
    }
    return 0;
}