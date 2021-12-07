#![allow(dead_code)]

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn read_file() -> Vec<String> {
    let filename = "day-7/input.txt";
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

fn parse_input(lines: Vec<String>) -> Vec<i32> {
    let input = lines.get(0).unwrap();

    input
        .split(',')
        .map(|n| n.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

mod day2 {
    fn get_sum(distance: i32) -> i32 {
        (distance * (distance + 1)) / 2
    }
    pub fn find_cheapest_path(input: &[i32], min: i32, max: i32) {
        let mut result = i32::MAX;

        for i in min..max {
            let mut cache: Vec<i32> = vec![];

            for n in input {
                cache.push(get_sum((n - i).abs()));
            }
            result = result.min(cache.iter().sum());
        }
        println!("min: {}", result);
    }
}

mod day1 {
    pub fn find_cheapest_path(input: &[i32], min: i32, max: i32) {
        let mut result = i32::MAX;
        for i in min..max {
            let mut cache: Vec<i32> = vec![];

            for n in input {
                cache.push((n - i).abs());
            }
            result = result.min(cache.iter().sum());
        }

        println!("min: {}", result);
    }
}

fn main() {
    let lines = read_file();
    let input = parse_input(lines);

    let max = *input.iter().max().unwrap();
    let min = *input.iter().min().unwrap();

    day2::find_cheapest_path(&input, min, max);
}
