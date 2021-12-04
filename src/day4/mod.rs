use array2d::Array2D;
use std::fs;

struct BingoBoard {
    board: Array2D<String>,
    marked_board: Array2D<bool>,
    is_won: bool,
}

pub fn task1() {
    let file_data = fs::read_to_string("./src/day4/data.txt").expect("Error reading file");
    let bingo_data: Vec<&str> = file_data.split("\n\n").collect::<Vec<&str>>();
    let bingo_numbers: Vec<&str> = bingo_data[0].split(",").collect::<Vec<&str>>();

    let mut boards = prepare_boards_list(&bingo_data);

    let mut winner_board_index = 0;
    let mut last_bingo_number = 0;

    for bingo_number in bingo_numbers.iter() {
        if boards.iter().any(|x| x.is_won) {
            break;
        }

        for (z, bingo_board) in boards.iter_mut().enumerate() {
            for (i, row_iter) in bingo_board.board.rows_iter().enumerate() {
                for (y, element) in row_iter.enumerate() {
                    if element == bingo_number {
                        bingo_board.marked_board[(i, y)] = true;
                    }
                }
            }

            let rows = bingo_board.marked_board.as_rows();
            for row in rows.iter() {
                if row.iter().all(|&x| x) {
                    winner_board_index = z;
                    bingo_board.is_won = true;
                    last_bingo_number = bingo_number.parse::<i32>().unwrap();
                    break;
                }
            }
            let columns = bingo_board.marked_board.as_columns();
            for column in columns.iter() {
                if column.iter().all(|&x| x) {
                    winner_board_index = z;
                    bingo_board.is_won = true;
                    last_bingo_number = bingo_number.parse::<i32>().unwrap();
                    break;
                }
            }
        }
    }

    let winner_board = &boards[winner_board_index];
    let sum = calculate_unmarked_board_sum(winner_board);

    println!("Day 4 - task 1: {}", sum * last_bingo_number); //28082
}

pub fn task2() {
    let file_data = fs::read_to_string("./src/day4/data.txt").expect("Error reading file");
    let bingo_data: Vec<&str> = file_data.split("\n\n").collect::<Vec<&str>>();
    let bingo_numbers: Vec<&str> = bingo_data[0].split(",").collect::<Vec<&str>>();

    let mut boards = prepare_boards_list(&bingo_data);

    let mut winner_board_index = 0;
    let mut last_bingo_number = 0;

    for bingo_number in bingo_numbers.iter() {
        for (z, bingo_board) in boards.iter_mut().enumerate() {
            if !bingo_board.is_won {
                for (i, row_iter) in bingo_board.board.rows_iter().enumerate() {
                    for (y, element) in row_iter.enumerate() {
                        if element == bingo_number {
                            bingo_board.marked_board[(i, y)] = true;
                        }
                    }
                }

                let rows = bingo_board.marked_board.as_rows();
                for row in rows.iter() {
                    if row.iter().all(|&x| x) {
                        winner_board_index = z;
                        bingo_board.is_won = true;
                        last_bingo_number = bingo_number.parse::<i32>().unwrap();
                        break;
                    }
                }

                let columns = bingo_board.marked_board.as_columns();
                for column in columns.iter() {
                    if column.iter().all(|&x| x) {
                        winner_board_index = z;
                        bingo_board.is_won = true;
                        last_bingo_number = bingo_number.parse::<i32>().unwrap();
                        break;
                    }
                }
            }
        }
    }

    let winner_board = &boards[winner_board_index];
    let sum = calculate_unmarked_board_sum(winner_board);
    println!("Day 4 - task2: {}", sum * last_bingo_number);  // 8224
}

fn prepare_boards_list(bingo_data: &Vec<&str>) -> Vec<BingoBoard> {
    let mut boards = Vec::<BingoBoard>::new();
    for i in 1..bingo_data.len() {
        let rows = bingo_data[i].split("\n").collect::<Vec<&str>>();

        let mapped_rows = rows
            .iter()
            .map(|x| {
                x.split(" ")
                    .filter(|&y| !y.is_empty())
                    .map(|y| String::from(y))
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<Vec<String>>>();

        let bingo_board = Array2D::from_rows(&mapped_rows);
        let marked_board = Array2D::filled_with(false, 5, 5);

        let board = BingoBoard {
            board: bingo_board,
            marked_board: marked_board,
            is_won: false,
        };
        boards.push(board);
    }

    return boards;
}

fn calculate_unmarked_board_sum(winner_board: &BingoBoard) -> i32 {
    let mut sum = 0;

    for (i, row_iter) in winner_board.marked_board.rows_iter().enumerate() {
        for (y, element) in row_iter.enumerate() {
            if !element {
                let value = winner_board.board[(i, y)].parse::<i32>().unwrap();
                sum += value;
            }
        }
    }

    return sum;
}
