#![allow(dead_code)]

fn read_input(input: &str) -> Vec<String> {
    input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect::<Vec<_>>()
}

mod solution {
    use std::collections::HashMap;

    #[derive(Debug)]
    pub enum Size {
        Small,
        Big,
    }

    #[derive(Debug)]
    pub struct Node<'a> {
        name: &'a str,
        size: Size,
        edges: Vec<&'a str>,
    }

    pub fn find_paths<'a>(
        graph: &HashMap<&'a str, Node<'a>>,
        current: &'a str,
        visited: &mut HashMap<&'a str, i32>,
        end: &'a str,
        count: &mut i32,
    ) -> i32 {
        let current_node = graph.get(current).unwrap();
        let mut visited_copy = visited.clone();

        // part 2
        if visited.values().sum::<i32>() > visited.len() as i32 + 2 {
            return 0;
        }

        if let Some(&v) = visited.get(current) {
            // part 2
            if v == 2 {
                return *count;
            }
        }

        if current == end {
            println!("count: {}", count);
            *count += 1;
            return *count;
        }
        if matches!(current_node.size, Size::Small) {
            let v = visited_copy.entry(current).or_insert(0);
            *v += 1;

            if current == "start" {
                *v = 2;
            }
        }

        for node in current_node.edges.clone() {
            find_paths(graph, node, &mut visited_copy, end, count);
        }

        *count
    }

    pub fn parse_input<'a>(lines: &'a [String]) -> HashMap<&'a str, Node<'a>> {
        let mut nodes: HashMap<&str, Node<'a>> = HashMap::new();
        lines.iter().for_each(|row| {
            if let [from, to] = row.split('-').collect::<Vec<&str>>()[..] {
                let first_char: char = from.chars().next().unwrap();
                let size = if first_char.is_ascii_uppercase() || from == "end" {
                    Size::Big
                } else {
                    Size::Small
                };

                let new_node: Node<'a> = Node {
                    name: from,
                    size,
                    edges: vec![],
                };

                let node = nodes.entry(from).or_insert(new_node);
                node.edges.push(to);

                let first_char: char = to.chars().next().unwrap();
                let size = if first_char.is_ascii_uppercase() || to == "end" {
                    Size::Big
                } else {
                    Size::Small
                };

                let new_node: Node<'a> = Node {
                    name: to,
                    size,
                    edges: vec![],
                };

                let node = nodes.entry(to).or_insert(new_node);
                node.edges.push(from);
            }
        });

        println!("{:#?}", nodes);
        nodes
    }

    pub fn solve<'a>(lines: &'a [String]) -> i32 {
        let nodes = parse_input(lines);

        find_paths(&nodes, "start", &mut HashMap::new(), "end", &mut 0)
    }
}

fn print_paths<'a>(paths: &Vec<Vec<&'a str>>) {
    for path in paths {
        println!("{:?}", path);
    }
}
fn main() {
    let file = include_str!("input.txt");
    let lines = read_input(file);
    let result = solution::solve(&lines);

    println!("Number of paths: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let file = include_str!("test_input_1.txt");
        let lines = read_input(file);

        let result = solution::solve(&lines);
        let expected = 36;

        assert_eq!(expected, result);
    }

    #[test]
    fn test_input_2() {
        let file = include_str!("test_input_2.txt");
        let lines = read_input(file);

        let result = solution::solve(&lines);
        let expected = 103;

        assert_eq!(expected, result);
    }

    #[test]
    fn test_input_3() {
        let file = include_str!("test_input_3.txt");
        let lines = read_input(file);

        let result = solution::solve(&lines);
        let expected = 3509;

        assert_eq!(expected, result);
    }
}
