#![allow(dead_code)]

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn read_file() -> Vec<String> {
    let filename = "day-9/input.txt";
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

fn parse_input(lines: Vec<String>) -> Vec<Vec<i32>> {
    let iter = lines.iter();

    iter.map(|row| {
        row.chars()
            .map(|n| n.to_string().parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
    })
    .collect::<Vec<Vec<i32>>>()
}

mod solution {
    #[derive(Debug)]
    pub struct Point {
        pub row: usize,
        pub column: usize,
        pub number: i32,
    }
    const DIRECTIONS: [(i32, i32); 4] = [(0i32, -1i32), (0i32, 1i32), (-1i32, 0i32), (1i32, 0i32)];

    pub fn get_lowest_points(input: &[Vec<i32>]) -> Vec<Point> {
        let mut result: Vec<Point> = vec![];

        for (y, row) in input.iter().enumerate() {
            for (x, number) in row.iter().enumerate() {
                let mut lowest = true;

                for dir in DIRECTIONS {
                    let dir_x = x as i32 + dir.0;
                    let dir_y = y as i32 + dir.1;

                    if let Some(r) = input.get(dir_y as usize) {
                        if let Some(cell) = r.get(dir_x as usize) {
                            if cell <= number {
                                lowest = false;
                            }
                        }
                    }
                }

                if lowest {
                    result.push(Point {
                        row: y,
                        column: x,
                        number: *number,
                    });
                }
            }
        }

        result
    }

    fn do_find_basins(grid: &[Vec<i32>], cache: &mut [Vec<bool>], row: i32, column: i32) -> i32 {
        if cache[row as usize][column as usize] {
            return 0;
        }

        if grid[row as usize][column as usize] == 9 {
            return 0;
        }

        let mut size = 1;
        cache[row as usize][column as usize] = true;

        for dir in DIRECTIONS {
            let dir_x = row + dir.0;
            let dir_y = column + dir.1;

            if let Some(x) = grid.get(dir_x as usize) {
                if x.get(dir_y as usize).is_some() {
                    size += do_find_basins(grid, cache, dir_x, dir_y);
                }
            }
        }

        size
    }

    pub fn find_basins(grid: &[Vec<i32>], lowest_points: Vec<Point>) {
        let mut cache = vec![vec![false; grid[0].len()]; grid.len()];
        let mut basins: Vec<i32> = lowest_points
            .iter()
            .map(|point| do_find_basins(grid, &mut cache, point.row as i32, point.column as i32))
            .collect();

        basins.sort_unstable();

        let result: i32 = basins.iter().rev().take(3).product();

        println!("result: {:?}", result);
    }
}

fn main() {
    let lines = read_file();
    let input = parse_input(lines);

    let lowest_points = solution::get_lowest_points(&input);
    let result1 = lowest_points.iter().fold(0, |acc, x| acc + x.number + 1);

    solution::find_basins(&input, lowest_points);
}
