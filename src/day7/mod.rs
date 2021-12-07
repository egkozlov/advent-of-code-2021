use std::collections::HashMap;
use std::fs;

pub fn task1() {
    let crabs_positions: Vec<i32> = get_crabs_positions();

    let min_position = crabs_positions.iter().min().unwrap();
    let max_position = crabs_positions.iter().max().unwrap();

    let mut fuel_map: HashMap<i32, i32> = HashMap::new();
    for i in *min_position..*max_position {
        let mut fuel = 0;

        for crab_position in crabs_positions.iter() {
            fuel += (i - crab_position).abs();
        }

        fuel_map.insert(i, fuel);
    }

    let result = fuel_map.iter().min_by_key(|&(_, v)| v).unwrap();

    println!("Day7 - task1: {:?}", result);
}

pub fn task2() {
    let crabs_positions: Vec<i32> = get_crabs_positions();
    let min_position = crabs_positions.iter().min().unwrap();
    let max_position = crabs_positions.iter().max().unwrap();

    let mut fuel_map: HashMap<i32, i32> = HashMap::new();
    for i in *min_position..*max_position {
        let mut fuel = 0;

        for crab_position in crabs_positions.iter() {
            let steps_count = (i - crab_position).abs();
            fuel += arithmetic_progression(steps_count);
        }

        fuel_map.insert(i, fuel);
    }

    let result = fuel_map.iter().min_by_key(|&(_, v)| v).unwrap();

    println!("Day7 - task2: {:?}", result);
}

fn get_crabs_positions() -> Vec<i32> {
    let file_data = fs::read_to_string("./src/day7/data.txt").expect("Error reading file");
    return file_data
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
}

fn arithmetic_progression(length: i32) -> i32 {
    let mut result = 0;
    for i in 0..length {
        result += i + 1;
    }

    return result;
}
