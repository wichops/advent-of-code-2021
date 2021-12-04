use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const BOARD_SIZE: usize = 5;
type Board = Vec<Vec<(i32, bool)>>;

fn read_file() -> Vec<String> {
    let filename = "day-4/input.txt";
    let file = match File::open(filename) {
        Ok(f) => f,
        Err(e) => {
            println!("{:?}", e);
            return Vec::new();
        }
    };
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|x| x.unwrap()).collect();

    lines
}

fn parse_input(lines: Vec<String>) -> (Vec<i32>, Vec<Board>) {
    let mut iter = lines.iter();

    let drawed_numbers: Vec<i32> = iter
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let mut board_index = 0;
    iter.next();
    let boards = iter.fold(vec![], |mut acc: Vec<Board>, line| {
        if line.trim() == "" {
            board_index += 1;
            return acc;
        }

        if acc.get(board_index).is_none() {
            acc.push(vec![]);
        }

        let numbers: Vec<(i32, bool)> = line
            .split_whitespace()
            .map(|s| (s.parse::<i32>().unwrap(), false))
            .collect();

        acc[board_index].push(numbers);

        acc
    });

    (drawed_numbers, boards)
}
fn main() {
    let lines = read_file();

    let (drawed_numbers, mut boards) = parse_input(lines);

    println!(
        "Winner: {:?}",
        call_numbers_part_one(drawed_numbers, &mut boards)
    );
}

fn call_numbers_part_one(numbers: Vec<i32>, boards: &mut Vec<Board>) -> Option<i32> {
    for n in numbers.iter() {
        for (index, b) in boards.iter_mut().enumerate() {
            mark_cell(b, *n);

            if is_winning_board(b) {
                let score = calculate_score(&boards[index]) * n;

                return Some(score);
            }
        }
    }

    None
}

#[allow(dead_code)]
fn call_numbers(numbers: Vec<i32>, boards: &mut Vec<Board>) -> Option<i32> {
    let mut winning_boards: Vec<usize> = vec![0; boards.len()];
    let board_count = boards.len();

    for n in numbers.iter() {
        for (index, b) in boards.iter_mut().enumerate() {
            mark_cell(b, *n);

            if is_winning_board(b) {
                winning_boards[index] = 1;

                if winning_boards.iter().sum::<usize>() == board_count {
                    let score = calculate_score(&boards[index]) * n;

                    return Some(score);
                }
            }
        }
    }

    None
}

#[allow(dead_code)]
fn print_board(boards: &Board) {
    boards.iter().for_each(|b| {
        println!("board");
        println!();
        b.iter().for_each(|r| println!("{:?}", r));
    });
}

fn calculate_score(board: &Board) -> i32 {
    board.iter().fold(0, |sum, row| {
        sum + row
            .iter()
            .filter(|(_, drawed)| !drawed)
            .map(|(n, _)| n)
            .sum::<i32>()
    })
}

fn mark_cell(board: &mut Board, number: i32) {
    for row in board.iter_mut() {
        for cell in row.iter_mut() {
            let (cell_number, _) = cell;

            if *cell_number == number {
                *cell = (*cell_number, true);
            }
        }
    }
}

fn is_winning_board(board: &Board) -> bool {
    for i in 0..BOARD_SIZE {
        let mut column_matches_count = 0;
        let mut row_matches_count = 0;
        for j in 0..BOARD_SIZE {
            if board[j][i].1 {
                column_matches_count += 1;
            }
            if board[i][j].1 {
                row_matches_count += 1;
            }
        }

        if column_matches_count == BOARD_SIZE || row_matches_count == BOARD_SIZE {
            return true;
        }
    }

    false
}
