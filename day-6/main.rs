use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const LIMIT: usize = 256;
fn read_file() -> Vec<String> {
    let filename = "day-6/input.txt";
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

fn parse_input(lines: Vec<String>) -> Vec<u64> {
    let input = lines.iter().next().unwrap();

    input
        .split(',')
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
}

fn make_cache(birthdays: &Vec<Vec<u64>>, current: u64, cache: &mut [u64; LIMIT]) -> u64 {
    if let Some(value) = cache.get(current as usize) {
        if *value > 0 {
            return *value;
        }
    }

    if let Some(children) = birthdays.get(current as usize) {
        cache[current as usize] += children.len() as u64;
        for c in children {
            cache[current as usize] += make_cache(birthdays, c + 8, cache);
        }
        cache[current as usize]
    } else {
        0
    }
}

fn main() {
    let lines = read_file();
    let mut input = parse_input(lines);

    let limit = LIMIT;
    let mut steps = 1;
    let mut new_fish_count = 0;
    let mut birthdays: Vec<Vec<u64>> = vec![];

    for i in 0..limit {
        let mut step = i;
        let mut children = vec![];
        while step < limit {
            children.push((step as u64) + 1);
            step += 7;
        }
        birthdays.push(children);
    }

    let mut cache = [0; LIMIT];
    let mut total = input.len() as u64;

    for i in 1..6 {
        make_cache(&birthdays, i, &mut cache);
    }

    for i in &input {
        total += cache[*i as usize];
    }
    println!("cache {:?}", cache);
    println!("total {}", total);

    // println!("Initial State: {:?}", input);
    // while steps <= limit {
    //     for i in input.iter_mut() {
    //         if *i == 0 {
    //             new_fish_count += 1;
    //         }
    //         if *i == 0 {
    //             *i = 6;
    //         } else {
    //             *i -= 1;
    //         }
    //     }

    //     for _ in 0..new_fish_count {
    //         input.push(8);
    //     }
    //     new_fish_count = 0;
    //     steps += 1;
    // }

    // let mut steps2 = 1;
    // let limit2 = 128;
    // new_fish_count = 0;

    // while steps2 <= limit2 {
    //     for i in cache.iter_mut() {
    //         if *i == 0 {
    //             new_fish_count += 1;
    //         }
    //         if *i == 0 {
    //             *i = 6;
    //         } else {
    //             *i -= 1;
    //         }
    //     }

    //     for _ in 0..new_fish_count {
    //         cache.push(8);
    //     }
    //     new_fish_count = 0;
    //     steps2 += 1;
    // }
}
