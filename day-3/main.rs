use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn part_one() {
    let filename = "day-3/input.txt";
    let file = match File::open(filename) {
        Ok(f) => f,
        Err(e) => {
            println!("{:?}", e);
            return;
        }
    };
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|x| x.unwrap()).collect();

    let one_counts = lines.iter().fold(vec![0; lines[0].len()], |mut acc, line| {
        for (index, c) in line.chars().enumerate() {
            if c == '1' {
                acc[index] += 1;
            }
        }

        acc
    });

    let max_digit_count = lines.len() as i32 / 2;

    let (gamma, epsillon) =
        one_counts
            .iter()
            .fold((String::new(), String::new()), |mut acc, current| {
                let (gamma, epsillon) = if current > &max_digit_count {
                    ('1', '0')
                } else {
                    ('0', '1')
                };

                acc.0.push_str(&gamma.to_string());
                acc.1.push_str(&epsillon.to_string());

                acc
            });

    println!(
        "gamma: {}, epsillon: {} ---> {}",
        gamma,
        epsillon,
        i32::from_str_radix(&gamma, 2).unwrap() * i32::from_str_radix(&epsillon, 2).unwrap()
    );
}

fn main() {
    let filename = "day-3/input.txt";
    let file = match File::open(filename) {
        Ok(f) => f,
        Err(e) => {
            println!("{:?}", e);
            return;
        }
    };

    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|x| x.unwrap()).collect();

    let generator_rating = get_generator_rating(lines.clone(), 0);
    let scrubber_rating = get_scrubber_rating(lines, 0);
    println!("generator: {:?}", generator_rating);
    println!("scrubber: {:?}", scrubber_rating);

    println!(
        "Result: {}",
        i32::from_str_radix(&generator_rating, 2).unwrap()
            * i32::from_str_radix(&scrubber_rating, 2).unwrap()
    );
}

fn get_generator_rating(lines: Vec<String>, index: usize) -> String {
    let mut one_lines = vec![];
    let mut zero_lines = vec![];

    if lines.len() == 1 {
        return lines[0].clone();
    }
    lines.iter().for_each(|line| {
        if line.chars().nth(index).unwrap() == '1' {
            one_lines.push(line.clone());
        } else {
            zero_lines.push(line.clone());
        }
    });

    if one_lines.len() >= zero_lines.len() {
        return get_generator_rating(one_lines, index + 1);
    }

    get_generator_rating(zero_lines, index + 1)
}

fn get_scrubber_rating(lines: Vec<String>, index: usize) -> String {
    let mut one_lines = vec![];
    let mut zero_lines = vec![];

    if lines.len() == 1 {
        return lines[0].clone();
    }
    lines.iter().for_each(|line| {
        if line.chars().nth(index).unwrap() == '1' {
            one_lines.push(line.clone());
        } else {
            zero_lines.push(line.clone());
        }
    });

    if one_lines.len() >= zero_lines.len() {
        return get_scrubber_rating(zero_lines, index + 1);
    }

    get_scrubber_rating(one_lines, index + 1)
}
