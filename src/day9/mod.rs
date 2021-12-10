use array2d::Array2D;
use std::fs;

pub fn task1() {
    let file_data = fs::read_to_string("./src/day9/data.txt").expect("Error reading file");
    let rows = file_data.split("\n").collect::<Vec<&str>>();

    let mapped_rows = rows
        .iter()
        .map(|x| {
            x.split("")
                .filter(|&y| !y.is_empty())
                .map(|y| y.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let height_map = Array2D::from_rows(&mapped_rows);
    let mut result = 0;

    for (i, row_iter) in height_map.rows_iter().enumerate() {
        for (y, element) in row_iter.enumerate() {
            let mut top = 10;
            let mut bottom = 10;
            let mut left = 10;
            let mut right = 10;

            if y > 0 {
                top = height_map[(i, y - 1)];
            }

            if y < height_map.row_len() - 1 {
                bottom = height_map[(i, y + 1)];
            }

            if i > 0 {
                left = height_map[(i - 1, y)];
            }

            if i < height_map.column_len() - 1 {
                right = height_map[(i + 1, y)];
            }

            if element < &top && element < &bottom && element < &left && element < &right {
                result += element + 1;
            }
        }
    }
    
    println!("Day8 - task1: {:?}", result);
}
