use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_file(filenam: &str) -> Vec<String> {
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
