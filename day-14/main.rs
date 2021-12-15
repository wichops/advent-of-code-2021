#![allow(dead_code)]

fn read_input(input: &str) -> Vec<String> {
    input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect::<Vec<_>>()
}

mod solution {
    use std::collections::{BTreeMap, HashMap};

    type Insertion<'a> = (&'a str, char);

    pub fn parse_input(lines: &[String]) -> (String, Vec<Insertion>) {
        let mut iter = lines.iter();

        let template = iter.next().unwrap();
        let mut pairs = vec![];

        for line in iter {
            let split: Vec<&str> = line.split(" -> ").collect();

            pairs.push((split[0], split[1].chars().next().unwrap()));
        }

        (template.to_string(), pairs)
    }

    pub fn make_iteration(template: &str, pairs: &[Insertion]) -> (String, HashMap<char, i64>) {
        let mut updates: BTreeMap<usize, char> = BTreeMap::new();
        let mut ocurrences: HashMap<char, i64> = HashMap::new();

        let template_chars: Vec<char> = template.chars().collect();

        for i in &template_chars {
            let entry = ocurrences.entry(*i).or_insert(0);
            *entry += 1;
        }

        for i in 0..template.len() - 1 {
            let j = i + 1;
            for (pair, output) in pairs {
                if let [first, second] = pair.chars().collect::<Vec<char>>()[..] {
                    if first == template_chars[i] && second == template_chars[j] {
                        updates.insert(j, *output);
                    }
                }
            }
        }

        let mut result = template.to_string();

        for (offset, (index, output)) in updates.iter().enumerate() {
            let copy = result.clone();
            let left = &mut copy[0..*index + offset].to_string();
            let right = &copy[*index + offset..];

            left.push(*output);
            left.push_str(right);

            let entry = ocurrences.entry(*output).or_insert(0);
            *entry += 1;
            result = left.clone();
        }

        (result, ocurrences)
    }

    pub fn solve(lines: &[String], times: i64) -> (String, i64) {
        let (template, pairs) = parse_input(lines);

        let mut chain = template;
        let mut ocurrences: HashMap<char, i64> = HashMap::new();
        for _ in 0..times {
            let result = make_iteration(&chain, &pairs);
            chain = result.0;
            ocurrences = result.1;
        }

        let max = ocurrences.values().max().unwrap();
        let min = ocurrences.values().min().unwrap();

        (chain, max - min)
    }
}

fn main() {
    let file = include_str!("input.txt");
    let lines = read_input(file);
    let result = solution::solve(&lines, 10);

    println!("Result: {:?}", result.1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let file = include_str!("test_input_1.txt");
        let lines = read_input(file);

        let (chain, _) = solution::solve(&lines, 3);

        let expected = "NBBBCNCCNBBNBNBBCHBHHBCHB";
        assert_eq!(expected, chain);
    }

    #[test]
    fn test_input_ten() {
        let file = include_str!("test_input_1.txt");
        let lines = read_input(file);

        let (_, result) = solution::solve(&lines, 10);

        let expected = 1588;
        assert_eq!(expected, result);
    }

    // #[test]
    // fn test_input_fourty() {
    //     let file = include_str!("test_input_1.txt");
    //     let lines = read_input(file);

    //     let (_, result) = solution::solve(&lines, 40);

    //     let expected = 2188189693529;
    //     assert_eq!(expected, result);
    // }
}
