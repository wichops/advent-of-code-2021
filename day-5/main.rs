use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Vector {
    y: i32,
    x: i32,
}

impl Vector {
    fn get_direction(&self) -> Self {
        let mut x = 0;
        let mut y = 0;

        if self.x != 0 {
            x = self.x / self.x.abs();
        }

        if self.y != 0 {
            y = self.y / self.y.abs();
        }

        Vector { x, y }
    }
}
type Board = Vec<Vec<i32>>;

fn read_file() -> Vec<String> {
    let filename = "day-5/input.txt";
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

#[allow(dead_code)]
fn parse_input(lines: Vec<String>) -> Vec<(Vector, Vector)> {
    let re = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
    let iter = lines.iter();

    iter.map(|line| {
        let caps = re.captures(line).unwrap();

        let start = Vector {
            x: caps[1].parse::<i32>().unwrap(),
            y: caps[2].parse::<i32>().unwrap(),
        };
        let end = Vector {
            x: caps[3].parse::<i32>().unwrap(),
            y: caps[4].parse::<i32>().unwrap(),
        };

        if start.x > end.x || start.y > end.y {
            return (end, start);
        }
        (start, end)
    })
    .filter(|(start, end)| start.x == end.x || start.y == end.y)
    .collect::<Vec<(Vector, Vector)>>()
}

fn parse_input_part_two(lines: Vec<String>) -> Vec<(Vector, Vector)> {
    // 0,9 -> 5,9
    let re = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
    let iter = lines.iter();

    iter.map(|line| {
        let caps = re.captures(line).unwrap();

        let start = Vector {
            x: caps[1].parse::<i32>().unwrap(),
            y: caps[2].parse::<i32>().unwrap(),
        };
        let end = Vector {
            x: caps[3].parse::<i32>().unwrap(),
            y: caps[4].parse::<i32>().unwrap(),
        };

        if start.x > end.x || start.y > end.y {
            return (end, start);
        }
        (start, end)
    })
    .collect::<Vec<(Vector, Vector)>>()
}

fn paint_lines(vectors: Vec<(Vector, Vector)>) -> Board {
    let mut board = vec![vec![0; 1000]; 1000];

    for (start, end) in vectors {
        let delta = Vector {
            x: end.x - start.x,
            y: end.y - start.y,
        };

        let direction = delta.get_direction();

        let mut x = start.x;
        let mut y = start.y;

        while x != end.x || y != end.y {
            board[y as usize][x as usize] += 1;
            x += direction.x;
            y += direction.y;
        }
        board[y as usize][x as usize] += 1;
    }

    board
}
fn count_overlapping_spaces(board: Board) -> i32 {
    board.iter().fold(0, |acc, row| {
        acc + row.iter().filter(|c| c > &&1).count() as i32
    })
}

fn main() {
    let lines = read_file();
    let vectors = parse_input_part_two(lines);
    let board = paint_lines(vectors);

    for row in &board {
        println!("{:?}", row);
    }
    println!("Result: {}", count_overlapping_spaces(board));
}
