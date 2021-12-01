use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn part_one() {
    let filename = "day-1/input.txt";
    let file = match File::open(filename) {
        Ok(f) => f,
        Err(e) => {
            println!("{:?}", e);
            return;
        }
    };
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut result = 0;
    let mut last_line = 99999;

    for line in lines {
        let value = line.unwrap().parse::<i32>().unwrap();
        if last_line < value {
            result += 1;
        }
        last_line = value;
    }
    println!("{}", result);
}

fn main() {
    let filename = "day-1/input.txt";
    let file = match File::open(filename) {
        Ok(f) => f,
        Err(e) => {
            println!("{:?}", e);
            return;
        }
    };
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut result = 0;

    let mut before_last_line = 999;
    let mut last_line = 9999;
    let mut last_sum = 99999;

    for line in lines {
        let value = line.unwrap().parse::<i32>().unwrap();
        let sum = value + last_line + before_last_line;

        if last_sum < sum {
            result += 1;
        }

        before_last_line = last_line;
        last_line = value;
        last_sum = sum;
    }

    println!("result: {}", result);
}
