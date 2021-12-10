#![allow(dead_code)]

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn read_file() -> Vec<String> {
    let filename = "day-10/input.txt";
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

    #[derive(Debug)]
    enum ScoreType<I64> {
        Illegal(I64),
        Completion(I64),
    }

    fn find_line_score(line: &str) -> ScoreType<i64> {
        let mut stack = vec![];

        for c in line.chars() {
            let last = stack.last().unwrap_or(&' ');
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' => {
                    if last != &'(' {
                        return ScoreType::Illegal(3);
                    }
                    stack.pop();
                }
                ']' => {
                    if last != &'[' {
                        return ScoreType::Illegal(57);
                    }
                    stack.pop();
                }
                '}' => {
                    if last != &'{' {
                        return ScoreType::Illegal(1197);
                    }
                    stack.pop();
                }
                '>' => {
                    if last != &'<' {
                        return ScoreType::Illegal(25137);
                    }
                    stack.pop();
                }
                _ => {}
            }
        }

        let completion_score = stack.iter().rev().fold(0, |result, c| {
            let mut new_result = result * 5;
            new_result += match c {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => 0,
            };

            new_result
        });

        println!("{:?}", stack);
        println!("{:?}", completion_score);
        ScoreType::Completion(completion_score)
    }

    pub fn solve(input: Vec<String>) -> i64 {
        let results = input
            .iter()
            .map(|line| find_line_score(line))
            .collect::<Vec<ScoreType<i64>>>();

        let mut completion_result: Vec<i64> = results
            .iter()
            .filter(|r| matches!(r, ScoreType::Completion(_)))
            .map(|r| match r {
                ScoreType::Completion(n) => *n,
                _ => 0,
            })
            .collect();

        completion_result.sort_unstable();

        let index = completion_result.len() as i64 / 2;

        completion_result[index as usize]
    }
}

fn main() {
    let lines = read_file();

    let result = solution::solve(lines);

    println!("result: {}", result);
}
