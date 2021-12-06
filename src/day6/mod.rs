use std::fs;
use std::collections::HashMap;

pub fn task1() {
    let file_data = fs::read_to_string("./src/day6/data.txt").expect("Error reading file");
    let mut fish_days: Vec<i32> = file_data
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut counter = 0;

    while counter < 80 {
        let mut new_fishes = 0;        
        for i in 0..fish_days.len() {
            if fish_days[i] > 0 {
                fish_days[i] -= 1;
            } else {
                new_fishes += 1;
                fish_days[i] = 6;
            }
        }

        for _i in 0..new_fishes {
            fish_days.push(8);
        }

        counter += 1;
    }


    println!("Day6 - task1: {}", fish_days.len());
}


pub fn task2() {
    let file_data = fs::read_to_string("./src/day6/data.txt").expect("Error reading file");
    let fish_days: Vec<i32> = file_data
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    
    let mut fish_days_map: HashMap<i32, i64> = HashMap::new();
    for fish_day in fish_days.iter() {
        if fish_days_map.contains_key(fish_day) {
            let value = *fish_days_map.get(fish_day).unwrap();
            fish_days_map.insert(*fish_day, value + 1);
        } else {
            fish_days_map.insert(*fish_day, 1);
        }
    }

    for i in 0..9 {
        if !fish_days_map.contains_key(&i) {
            fish_days_map.insert(i, 0);
        }
    }


    let mut counter = 0;
    while counter < 256 {
        let new_fishes = *fish_days_map.get(&0).unwrap();
        for i in 0..8 {
            let mut next_value = *fish_days_map.get(&(i + 1)).unwrap();
            if i==6 {
                next_value += new_fishes;
            }
            fish_days_map.insert(i, next_value);
        }

        fish_days_map.insert(8, new_fishes);
        counter += 1;
    }

    let mut total = 0;
    for (_key, value) in fish_days_map.iter() {
        total += value;
    }

    println!("Day6 - task2: {}", total);
}