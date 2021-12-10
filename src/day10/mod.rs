use stats::median;
use std::fs;

pub fn task1() {
    let lines: Vec<Vec<String>> = get_bracket_lines();
    let opening_brackets: [&str; 4] = ["(", "{", "[", "<"];
    let closign_brackets: [&str; 4] = [")", "}", "]", ">"];
    let mut invalid_brackets: Vec<String> = Vec::new();
    for line in lines {
        let mut brackets_stack: Vec<String> = Vec::new();
        for bracket in line {
            if opening_brackets.iter().any(|x| x == &bracket) {
                brackets_stack.push(bracket);
            } else {
                let last_bracket = &brackets_stack[brackets_stack.len() - 1];
                let last_bracket_index = opening_brackets
                    .iter()
                    .position(|x| x == &last_bracket)
                    .unwrap();
                let expected_closing_bracket = closign_brackets[last_bracket_index];
                if &expected_closing_bracket == &bracket {
                    brackets_stack.pop();
                } else {
                    invalid_brackets.push(bracket);
                    break;
                }
            }
        }
    }

    let mut sum = 0;
    for invalid_bracket in invalid_brackets {
        if invalid_bracket == ")" {
            sum += 3;
        } else if invalid_bracket == "}" {
            sum += 1197;
        } else if invalid_bracket == "]" {
            sum += 57;
        } else {
            sum += 25137;
        }
    }

    println!("Day10 - task1: {}", sum);
}

pub fn task2() {
    let lines: Vec<Vec<String>> = get_bracket_lines();
    let opening_brackets: [&str; 4] = ["(", "{", "[", "<"];
    let closign_brackets: [&str; 4] = [")", "}", "]", ">"];

    let mut result_sum: Vec<i64> = Vec::new();

    for line in lines {
        let mut brackets_stack: Vec<&str> = Vec::new();
        let mut has_invalid_bracket = false;
        for bracket in line.iter() {
            if opening_brackets.iter().any(|x| x == bracket) {
                brackets_stack.push(bracket);
            } else {
                let last_bracket = brackets_stack[brackets_stack.len() - 1];
                let last_bracket_index = opening_brackets
                    .iter()
                    .position(|x| x == &last_bracket)
                    .unwrap();
                let expected_closing_bracket = closign_brackets[last_bracket_index];
                if &expected_closing_bracket == bracket {
                    brackets_stack.pop();
                } else {
                    has_invalid_bracket = true;
                    break;
                }
            }
        }

        if !has_invalid_bracket && !brackets_stack.is_empty() {
            let mut sum: i64 = 0;
            brackets_stack.reverse();
            for i in 0..brackets_stack.len() {
                sum = sum * 5;

                if brackets_stack[i] == "(" {
                    sum += 1;
                } else if brackets_stack[i] == "[" {
                    sum += 2;
                } else if brackets_stack[i] == "{" {
                    sum += 3;
                } else {
                    sum += 4;
                }
            }

            result_sum.push(sum);
        }
    }

    println!(
        "Day10 - task2: {:?}",
        median(result_sum.iter().map(|x| *x as f64))
    );
}

fn get_bracket_lines() -> Vec<Vec<String>> {
    let file_data = fs::read_to_string("./src/day10/data.txt").expect("Error reading file");
    return file_data
        .split("\n")
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .iter()
        .map(|x| {
            x.split("")
                .filter(|&y| !y.is_empty())
                .map(|y| y.to_string())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();
}
