use std::fs;

pub fn task1() {
    let file_data = fs::read_to_string("./src/day3/data.txt").expect("Error reading file");
    let bit_list: Vec<&str> = file_data.split("\n").collect::<Vec<&str>>();
    let bit_line_length = convert_bit_line_to_array(bit_list[0]).len();
    let mut ones_counts: Vec<i32> = Vec::new();
    let mut zero_counts: Vec<i32> = Vec::new();

    for _i in 0..bit_line_length {
        ones_counts.push(0);
        zero_counts.push(0);
    }

    for bit_list_item in bit_list {
        let bits = convert_bit_line_to_array(bit_list_item);

        for i in 0..bit_line_length {
            if bits[i] == 1 {
                ones_counts[i] += 1;
            } else {
                zero_counts[i] += 1;
            }
        }
    }

    let mut gamma_rate = "".to_owned();
    let mut epsilon_rate = "".to_owned();
    for i in 0..bit_line_length {
        if ones_counts[i] > zero_counts[i] {
            gamma_rate.push_str("1");
            epsilon_rate.push_str("0");
        } else {
            gamma_rate.push_str("0");
            epsilon_rate.push_str("1");
        }
    }

    let gamma_rate_int = i32::from_str_radix(&gamma_rate, 2).unwrap();
    let epsilon_rate_int = i32::from_str_radix(&epsilon_rate, 2).unwrap();

    println!("Day 3 - task1: {:?}", gamma_rate_int * epsilon_rate_int);
}

fn convert_bit_line_to_array(bit_line: &str) -> Vec<i32> {
    let mut bit_line_array: Vec<i32> = Vec::new();
    for bit in bit_line.split("").filter(|&x| !x.is_empty()) {
        if bit == "1" {
            bit_line_array.push(1);
        } else {
            bit_line_array.push(0);
        }
    }
    return bit_line_array;
}


pub fn task2() {
    let file_data = fs::read_to_string("./src/day3/data.txt").expect("Error reading file");
    let bit_list: Vec<&str> = file_data.split("\n").collect::<Vec<&str>>();
    let bit_line_length = convert_bit_line_to_array(bit_list[0]).len();
    let mut new_list: Vec<Vec<i32>> = Vec::new();
    for bit_list_item in bit_list {
        let bits = convert_bit_line_to_array(bit_list_item);

        new_list.push(bits);
    }

    let mut counter = 0;
    let mut to_fill = new_list;

    while to_fill.len() > 1 {
        let res = convert(to_fill, counter);
        to_fill = Vec::new();

        for bit_list_item in res {
            to_fill.push(bit_list_item);
        }

        counter += 1;
    }

    let mut gamma_rate = "".to_owned();
    for i in 0..bit_line_length {
        if to_fill[0][i] == 1 {
            gamma_rate.push_str("1");
        } else {
            gamma_rate.push_str("0");
        }
    }

    let gamma_rate_int = i32::from_str_radix(&gamma_rate, 2).unwrap();

    // 3583
    // 1601
    println!("Day 3 - task2: {:?}", gamma_rate_int);
    // let mut gamma_rate = "".to_owned();
    // let mut epsilon_rate = "".to_owned();
    // for i in 0..bit_line_length {
    //     if ones_counts[i] > zero_counts[i] {
    //         gamma_rate.push_str("1");
    //         epsilon_rate.push_str("0");
    //     } else {
    //         gamma_rate.push_str("0");
    //         epsilon_rate.push_str("1");
    //     }
    // }

    // let gamma_rate_int = i32::from_str_radix(&gamma_rate, 2).unwrap();
    // let epsilon_rate_int = i32::from_str_radix(&epsilon_rate, 2).unwrap();

    // println!("Day 3 - task1: {:?}", gamma_rate_int * epsilon_rate_int);
}

fn convert(list: Vec<Vec<i32>>, index: i32) -> Vec<Vec<i32>> {
    let mut ones_counts = 0;
    let mut zero_counts = 0;

    for bit_list_item in list.iter() {
        if bit_list_item[index as usize] == 1 {
            ones_counts += 1;
        } else {
            zero_counts += 1;
        }
    }

    let winner_index = if ones_counts >= zero_counts { 0 } else { 1 };
    let mut new_list: Vec<Vec<i32>> = Vec::new();

    for bit_list_item in list.iter() {
        if bit_list_item[index as usize] == winner_index {
            new_list.push(bit_list_item.to_vec());
        }
    }

    return new_list;
}

//gamma_rate 3583
//epsilon_rate 1601
