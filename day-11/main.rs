#![allow(dead_code)]

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

type Board = Vec<Vec<i32>>;
const SIZE: usize = 10;

fn read_file() -> Vec<String> {
    let filename = "day-11/input.txt";
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

mod solution {
    use crate::Board;

    const DIRECTIONS: [(i32, i32); 8] = [
        (0, 1),
        (0, -1),
        (1, 0),
        (1, 1),
        (1, -1),
        (-1, 0),
        (-1, 1),
        (-1, -1),
    ];

    pub fn parse_input(lines: Vec<String>) -> Board {
        let iter = lines.iter();

        iter.map(|row| {
            row.chars()
                .map(|n| n.to_string().parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>()
    }
    pub fn make_step(board: &mut Board) -> i32 {
        let mut flashes = vec![];
        for (y, row) in board.iter_mut().enumerate() {
            for (x, cell) in row.iter_mut().enumerate() {
                *cell += 1;
                if *cell == 10 {
                    flashes.push((y, x));
                }
            }
        }

        make_flashes(board, &mut flashes);

        count_flashes(board)
    }

    fn count_flashes(board: &mut Board) -> i32 {
        let mut count = 0;
        for row in board {
            for cell in row {
                if *cell >= 10 {
                    *cell = 0;
                    count += 1;
                }
            }
        }
        count
    }

    fn make_flashes(board: &mut Board, flashes: &mut Vec<(usize, usize)>) {
        let mut visited = flashes.clone();

        while !flashes.is_empty() {
            let (y, x) = flashes.pop().unwrap();
            if !visited.iter().any(|&f| (y, x) == f) {
                visited.push((y, x));
            }

            for dir in DIRECTIONS {
                let dir_y = y as i32 + dir.0;
                let dir_x = x as i32 + dir.1;

                if let Some(r) = board.get_mut(dir_y as usize) {
                    if let Some(cell) = r.get_mut(dir_x as usize) {
                        if *cell < 10 {
                            *cell += 1;
                        }

                        if *cell >= 10
                            && !visited
                                .iter()
                                .any(|&f| (dir_y as usize, dir_x as usize) == f)
                        {
                            if !flashes
                                .iter()
                                .any(|&f| (dir_y as usize, dir_x as usize) == f)
                            {
                                flashes.push((dir_y as usize, dir_x as usize));
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn solve(lines: Vec<String>, times: i32) -> (Board, i32) {
        let mut input = parse_input(lines);
        let mut result = 0;

        for i in 0..times {
            let count = make_step(&mut input);
            if count == 100 {
                println!("ALL FLASHED: {}", i + 1);
            }
            result += count;
        }

        (input, result)
    }
}

fn print_board(board: &Board) {
    for row in board {
        println!(
            "{:?}",
            row.iter()
                .map(|c| {
                    if *c == 10 {
                        0
                    } else {
                        *c
                    }
                })
                .collect::<Vec<i32>>()
        );
    }
}

fn compare_boards(a: &Board, b: &Board) -> bool {
    for i in 0..SIZE {
        for j in 0..SIZE {
            if a[i][j] != b[i][j] {
                return false;
            }
        }
    }

    true
}

fn main() {
    let lines = read_file();
    let result = solution::solve(lines, 500);
    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_days() {
        let input = include_str!("input.txt");
        let lines: Vec<_> = input
            .split('\n')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();

        let result = solution::solve(lines, 2);

        let expected: Board = vec![
            vec![8, 8, 0, 7, 4, 7, 6, 5, 5, 5],
            vec![5, 0, 8, 9, 0, 8, 7, 0, 5, 4],
            vec![8, 5, 9, 7, 8, 8, 9, 6, 0, 8],
            vec![8, 4, 8, 5, 7, 6, 9, 6, 0, 0],
            vec![8, 7, 0, 0, 9, 0, 8, 8, 0, 0],
            vec![6, 6, 0, 0, 0, 8, 8, 9, 8, 9],
            vec![6, 8, 0, 0, 0, 0, 5, 9, 4, 3],
            vec![0, 0, 0, 0, 0, 0, 7, 4, 5, 6],
            vec![9, 0, 0, 0, 0, 0, 0, 8, 7, 6],
            vec![8, 7, 0, 0, 0, 0, 6, 8, 4, 8],
        ];

        assert_eq!(result.0, expected);
    }

    #[test]
    fn test_ten_days() {
        let input = include_str!("input.txt");
        let lines: Vec<_> = input
            .split('\n')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();

        let result = solution::solve(lines, 10);
        let expected = [
            [0, 4, 8, 1, 1, 1, 2, 9, 7, 6],
            [0, 0, 3, 1, 1, 1, 2, 0, 0, 9],
            [0, 0, 4, 1, 1, 1, 2, 5, 0, 4],
            [0, 0, 8, 1, 1, 1, 1, 4, 0, 6],
            [0, 0, 9, 9, 1, 1, 1, 3, 0, 6],
            [0, 0, 9, 3, 5, 1, 1, 2, 3, 3],
            [0, 4, 4, 2, 3, 6, 1, 1, 3, 0],
            [5, 5, 3, 2, 2, 5, 2, 3, 5, 0],
            [0, 5, 3, 2, 2, 5, 0, 6, 0, 0],
            [0, 0, 3, 2, 2, 4, 0, 0, 0, 0],
        ];

        assert_eq!(result.0, expected);
        assert_eq!(result.1, 204);
    }
}
