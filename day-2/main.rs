use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn part_one() {
    let filename = "day-2/input.txt";
    let file = match File::open(filename) {
        Ok(f) => f,
        Err(e) => {
            println!("{:?}", e);
            return;
        }
    };
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let result = lines.fold((0, 0), |acc, line| {
        let value = line.unwrap();
        let instruction = value.split_whitespace().collect::<Vec<&str>>();

        let mut result = acc;
        if let [direction, value] = instruction[..] {
            let magnitude = value.parse::<i32>().unwrap();
            match direction {
                "forward" => result.0 += magnitude,
                "up" => result.1 -= magnitude,
                "down" => result.1 += magnitude,
                _ => {}
            }
        }

        result
    });

    println!("{:?} -> {}", result, result.0 * result.1);
}

fn main() {
    let filename = "day-2/input.txt";
    let file = match File::open(filename) {
        Ok(f) => f,
        Err(e) => {
            println!("{:?}", e);
            return;
        }
    };
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let result = lines.fold((0, 0, 0), |acc, line| {
        let value = line.unwrap();
        let instruction = value.split_whitespace().collect::<Vec<&str>>();

        let mut result = acc;
        if let [direction, value] = instruction[..] {
            let magnitude = value.parse::<i32>().unwrap();
            match direction {
                "forward" => {
                    result.0 += magnitude;
                    result.1 += result.2 * magnitude;
                }
                "up" => result.2 -= magnitude,
                "down" => result.2 += magnitude,
                _ => {}
            }
        }

        result
    });

    println!("{:?} -> {}", result, result.1 * result.0);
}
