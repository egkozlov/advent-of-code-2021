use std::collections::HashMap;
use std::fs;

struct Point {
    x: i32,
    y: i32,
}

struct Line {
    start: Point,
    end: Point,
}

pub fn task1() {
    let vents_lines: Vec<Line> = get_vents_lines();
    let horizontal_vertical_lines = vents_lines
        .iter()
        .filter(|line| line.start.x == line.end.x || line.start.y == line.end.y)
        .collect::<Vec<&Line>>();

    let mut map: HashMap<String, i32> = HashMap::new();

    for line in horizontal_vertical_lines {
        let mut x = line.start.x;
        let mut y = line.start.y;
        let direction_x = line.start.x - line.end.x;
        let direction_y = line.start.y - line.end.y;

        while (direction_x >= 1 && x >= line.end.x)
            || (direction_x <= -1 && x <= line.end.x)
            || (direction_y >= 1 && y >= line.end.y)
            || (direction_y <= -1 && y <= line.end.y)
        {
            let key = format!("{},{}", x, y);
            if map.contains_key(&key) {
                let value = *map.get(&key).unwrap();
                map.insert(key, value + 1);
            } else {
                map.insert(key, 1);
            }

            if direction_x < -1 {
                x += 1;
            } else if direction_x > 1 {
                x -= 1;
            } else if direction_y < -1 {
                y += 1;
            } else {
                y -= 1;
            }
        }
    }

    let res = map.iter().filter(|(_, v)| **v >= 2).count();
    println!("Day5 - task1: {}", res);
}

pub fn task2() {
    let vents_lines: Vec<Line> = get_vents_lines();
    let horizontal_vertical_lines = vents_lines
        .iter()
        .filter(|line| line.start.x == line.end.x || line.start.y == line.end.y)
        .collect::<Vec<&Line>>();

    let mut map: HashMap<String, i32> = HashMap::new();

    for line in horizontal_vertical_lines {
        let mut x = line.start.x;
        let mut y = line.start.y;
        let direction_x = line.start.x - line.end.x;
        let direction_y = line.start.y - line.end.y;

        while (direction_x >= 1 && x >= line.end.x)
            || (direction_x <= -1 && x <= line.end.x)
            || (direction_y >= 1 && y >= line.end.y)
            || (direction_y <= -1 && y <= line.end.y)
        {
            let key = format!("{},{}", x, y);
            if map.contains_key(&key) {
                let value = *map.get(&key).unwrap();
                map.insert(key, value + 1);
            } else {
                map.insert(key, 1);
            }

            if direction_x < -1 {
                x += 1;
            } else if direction_x > 1 {
                x -= 1;
            } else if direction_y < -1 {
                y += 1;
            } else {
                y -= 1;
            }
        }
    }

    let diagonal_lines = vents_lines
        .iter()
        .filter(|line| (line.start.x - line.end.x).abs() == (line.start.y - line.end.y).abs())
        .collect::<Vec<&Line>>();

    for line in diagonal_lines {
        let mut x = line.start.x;
        let mut y = line.start.y;
        let step_x = if x - line.end.x < 0 { 1 } else { -1 };
        let step_y = if y - line.end.y < 0 { 1 } else { -1 };

        while (step_x == 1 && x <= line.end.x) || (step_x == -1 && x >= line.end.x) {
            let key = String::from(format!("{},{}", x, y));
            if map.contains_key(&key) {
                let value = *map.get(&key).unwrap();
                map.insert(key, value + 1);
            } else {
                map.insert(key, 1);
            }

            x += step_x;
            y += step_y;
        }
    }

    let res = map.iter().filter(|(_, v)| **v >= 2).count();
    println!("Day5 - task2: {:?}", res);
}

fn get_vents_lines() -> Vec<Line> {
    let file_data = fs::read_to_string("./src/day5/data.txt").expect("Error reading file");
    let raw_vents_lines: Vec<&str> = file_data.split("\n").collect::<Vec<&str>>();

    let mut vents_lines: Vec<Line> = Vec::new();
    for raw_vent_line in raw_vents_lines {
        let line_points = raw_vent_line.split("->").collect::<Vec<&str>>();
        let vent_line_start = line_points[0].trim().split(",").collect::<Vec<&str>>();
        let vent_line_end = line_points[1].trim().split(",").collect::<Vec<&str>>();
        let start = Point {
            x: vent_line_start[0].parse::<i32>().unwrap(),
            y: vent_line_start[1].parse::<i32>().unwrap(),
        };
        let end = Point {
            x: vent_line_end[0].parse::<i32>().unwrap(),
            y: vent_line_end[1].parse::<i32>().unwrap(),
        };

        let line = Line {
            start: start,
            end: end,
        };
        vents_lines.push(line);
    }
    return vents_lines;
}
