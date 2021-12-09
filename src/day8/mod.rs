use std::collections::HashMap;
use std::fs;

pub fn task1() {
    let file_data = fs::read_to_string("./src/day8/data.txt").expect("Error reading file");
    let lines = file_data
        .split("\n")
        .map(|x| {
            let value = x.split("|").collect::<Vec<&str>>();
            return value[1].trim();
        })
        .collect::<Vec<&str>>();
    let mut output_count = 0;

    for line in lines {
        let words = line.split(" ").collect::<Vec<&str>>();
        for word in words {
            let length = word.len();
            if length == 2 || length == 3 || length == 7 || length == 4 {
                output_count += 1;
            }
        }
    }

    println!("Day8 - task1: {:?}", output_count);
}

struct DigitsLine {
    digits: Vec<String>,
    output: Vec<String>,
}

struct Digit {
    code: String,
    code_vec: Vec<String>,
}

pub fn task2() {
    let file_data = fs::read_to_string("./src/day8/data.txt").expect("Error reading file");
    let lines = file_data
        .split("\n")
        .map(|x| {
            let value = x.split("|").collect::<Vec<&str>>();

            let line = DigitsLine {
                digits: value[0]
                    .trim()
                    .split(" ")
                    .map(|y| String::from(y))
                    .collect::<Vec<String>>(),
                output: value[1]
                    .trim()
                    .split(" ")
                    .map(|y| String::from(y))
                    .collect::<Vec<String>>(),
            };
            return line;
        })
        .collect::<Vec<DigitsLine>>();

    for line in lines {
        let mut digits_map: HashMap<i32, Digit> = HashMap::new();
        let mut six_nine_zero_digits: Vec<&str> = Vec::new();
        let mut two_three_five_digits: Vec<&str> = Vec::new();
        for digit in line.digits.iter() {
            let code_vec = digit
                .split("")
                .filter(|&x| !x.is_empty())
                .map(|x| String::from(x))
                .collect::<Vec<String>>();

            let digit_value = Digit {
                code: digit.clone(),
                code_vec: code_vec,
            };
            if digit.len() == 2 {
                digits_map.insert(1, digit_value);
            } else if digit.len() == 3 {
                digits_map.insert(7, digit_value);
            } else if digit.len() == 7 {
                digits_map.insert(8, digit_value);
            } else if digit.len() == 4 {
                digits_map.insert(4, digit_value);
            } else if digit.len() == 6 {
                six_nine_zero_digits.push(digit);
            } else if digit.len() == 5 {
                two_three_five_digits.push(digit);
            }
        }

        let seven_digit = digits_map.get(&7).unwrap();
        for six_nine_zero_digit in six_nine_zero_digits.iter() {
            let vec = six_nine_zero_digit
                .split("")
                .filter(|&x| !x.is_empty())
                .map(|x| String::from(x))
                .collect::<Vec<String>>();

            if seven_digit.code_vec.iter().any(|x| !vec.contains(x)) {
                let code_vec = six_nine_zero_digit
                    .split("")
                    .filter(|&x| !x.is_empty())
                    .map(|x| String::from(x))
                    .collect::<Vec<String>>();
                let digit_value = Digit {
                    code: six_nine_zero_digit.to_string(),
                    code_vec: code_vec,
                };

                // digits_map.insert(6, digit_value);
            }
        }
    }
    // println!("Day7 - task2: {:?}", lines[0].output);
    // println!("Day7 - task2: {:?}", lines[0].digits);
    // println!("Day7 - task2: {:?}", result);
}
