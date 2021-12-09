#![allow(dead_code)]

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn read_file() -> Vec<String> {
    let filename = "day-8/input.txt";
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

fn parse_input(lines: Vec<String>) -> Vec<(Vec<String>, Vec<String>)> {
    let iter = lines.iter();

    iter.map(|line| {
        let splitted = line.split('|');

        splitted.fold(vec![], |mut result, current| {
            let processed = current
                .trim()
                .split(' ')
                .map(|r| {
                    let mut x = r.trim().chars().collect::<Vec<char>>();

                    x.sort();

                    x.into_iter().collect::<String>()
                })
                .collect::<Vec<String>>();

            result.push(processed);
            result
        })
    })
    .map(|entry| (entry[0].clone(), entry[1].clone()))
    .collect::<Vec<(Vec<String>, Vec<String>)>>()
}

mod part1 {
    pub fn count_numbers(input: &[String]) -> i32 {
        input.iter().fold(0, |acc, x| {
            acc + match x.len() {
                2 => 1,
                4 => 1,
                7 => 1,
                _ => 0,
            }
        })
    }
}

mod part2 {
    use std::collections::HashMap;

    #[derive(Default, Debug)]
    struct State {
        positions: HashMap<String, char>,
        digits: HashMap<char, String>,
    }

    impl State {
        pub fn new() -> Self {
            Self::default()
        }

        fn set_top_position(&mut self) {
            let seven = self.digits.get(&'7').unwrap();
            let one = self.digits.get(&'1').unwrap();

            self.positions.insert(
                "top".to_string(),
                seven.replace(one, "").chars().next().unwrap(),
            );
        }

        fn find_nine(&mut self, input: &Vec<String>) {
            let four = self.digits.get(&'4').unwrap();

            let mut nine: String = "x".to_string();
            for value in input.iter() {
                let rest = value.replace(four, "");

                println!("value: {}", value);
                if rest.len() == 2 && value.len() == 6 {
                    nine = value.to_string();
                }
            }

            self.digits.insert('9', nine);
        }

        fn find_three(&mut self, input: &Vec<String>) {
            let seven = self.digits.get(&'7').unwrap();

            let mut three: String = "x".to_string();

            for value in input.iter() {
                let rest = value.replace(seven, "");
                if rest.len() == 2 && value.len() == 5 {
                    three = value.to_string();
                }
            }

            self.digits.insert('3', three);
        }

        fn find_digits(&mut self, line: &(Vec<String>, Vec<String>)) {
            let (mut mixed_digits, b) = line.clone();
            mixed_digits.extend(b);

            for entry in &mixed_digits {
                let digit = match entry.len() {
                    2 => '1',
                    3 => '7',
                    4 => '4',
                    7 => '8',
                    _ => '-',
                };

                if digit != '-' {
                    self.digits.entry(digit).or_insert(entry.to_string());
                }
            }

            self.set_top_position();
            self.find_nine(&mixed_digits);
            self.find_three(&mixed_digits);

            println!("{:?}", self.positions);
            println!("{:?}", self.digits);
        }
    }

    pub fn solve(input: &[(Vec<String>, Vec<String>)]) -> i32 {
        for line in input {
            let mut state = State::new();
            state.find_digits(line);

            println!("{:?}", state);
        }

        0
    }
}

fn main() {
    let lines = read_file();
    let input = parse_input(lines);

    println!("{:?}", input);
    println!("result: {}", part2::solve(&input));
}
