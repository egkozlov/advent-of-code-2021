use std::fs;

pub fn task1() {
    let mut horizontal_position = 0;
    let mut depth = 0;
    let file_data = fs::read_to_string("./src/day2/data.txt").expect("Error reading file");
    let actions_list: Vec<&str> = file_data.split("\n").collect::<Vec<&str>>();
    for actions_list_line in actions_list {
        let line_data = actions_list_line.split(" ").collect::<Vec<&str>>();
        let action = line_data[0];
        let range = line_data[1].parse::<i32>().unwrap();

        if action == "forward" {
            horizontal_position += range;
        } else if action == "up" {
            depth -= range;
        } else if action == "down" {
            depth += range;
        }
    }

    println!("Day2 - task1: {}", depth * horizontal_position);
}

pub fn task2() {
    let mut horizontal_position = 0;
    let mut depth = 0;
    let mut aim = 0;
    let file_data = fs::read_to_string("./src/day2/data.txt").expect("Error reading file");
    let actions_list: Vec<&str> = file_data.split("\n").collect::<Vec<&str>>();

    for actions_list_line in actions_list {
        let line_data = actions_list_line.split(" ").collect::<Vec<&str>>();
        let action = line_data[0];
        let range = line_data[1].parse::<i32>().unwrap();

        if action == "forward" {
            horizontal_position += range;
            depth += aim * range;
        } else if action == "up" {
            aim -= range;
        } else if action == "down" {
            aim += range;
        }
    }

    println!("Day2 - task2: {}", depth * horizontal_position);
}
