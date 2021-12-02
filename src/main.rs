use std::fs;

fn main() {
    task_1();
    task_2();
}

fn task_1() {
    let numbers_list = get_numbers_from_file();
    let increases_count = calculate_increses_count(numbers_list);

    println!("Count task1: {}", increases_count); // correct answer: 1791
}

fn task_2() {
    let numbers_list = get_numbers_from_file();
    let mut new_numbers_list: Vec<i32> = Vec::new();
    for (i, _x) in numbers_list.iter().enumerate() {
        if i < numbers_list.len() - 2 {
            let first = numbers_list[i];
            let second = numbers_list[i + 1];
            let third = numbers_list[i + 2];
            new_numbers_list.push(first + second + third);
        }
    }

    let increases_count = calculate_increses_count(new_numbers_list);

    println!("Count task2: {}", increases_count); // correct answer: 1822
}

fn get_numbers_from_file() -> Vec<i32> {
    let file_data = fs::read_to_string("./src/day1/data.txt").expect("Error reading file");
    return file_data
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| s.parse().unwrap())
        .collect();
}

fn calculate_increses_count(numbers_list: Vec<i32>) -> i32 {
    let mut counts = 0;
    for (i, _x) in numbers_list.iter().enumerate() {
        if i > 0 {
            let previous = numbers_list[i - 1];
            let current = numbers_list[i];
            if previous < current {
                counts += 1;
            }
        }
    }
    return counts;
}