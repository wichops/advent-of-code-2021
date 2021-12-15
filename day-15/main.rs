#![allow(dead_code)]

type Board = Vec<Vec<i32>>;
fn read_input(input: &str) -> Vec<String> {
    input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect::<Vec<_>>()
}

fn print_board(board: &Board) {
    for row in board {
        println!("{:?}", row);
    }
}

mod solution {
    use std::{
        cmp::Ordering,
        collections::{BinaryHeap, HashMap, HashSet},
    };

    use crate::{print_board, read_input, Board};

    const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (0, -1), (-1, 0), (1, 0)];
    type Pos = (isize, isize);

    #[derive(Eq, Debug)]
    struct Node {
        pos: Pos,
        weight: i32,
        distance: i32,
        parent: Option<Pos>,
    }

    impl Ord for Node {
        fn cmp(&self, other: &Self) -> Ordering {
            other.distance.cmp(&self.distance)
        }
    }

    impl PartialOrd for Node {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl PartialEq for Node {
        fn eq(&self, other: &Self) -> bool {
            self.distance == other.distance
        }
    }

    pub fn parse_input(lines: Vec<String>) -> Board {
        let iter = lines.iter();

        iter.map(|row| {
            row.chars()
                .map(|n| n.to_string().parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>()
    }

    fn dikjstra(board: &Board, pos: Pos) -> i32 {
        let mut heap: BinaryHeap<Node> = BinaryHeap::new();
        let mut visited = HashSet::new();
        let mut distances = HashMap::new();

        let weight = board[pos.0 as usize][pos.1 as usize];
        let node: Node = Node {
            pos,
            weight,
            distance: 0,
            parent: None,
        };
        heap.push(node);
        distances.insert(pos, 0);

        while !heap.is_empty() {
            let current = heap.pop().unwrap();
            visited.insert(current.pos);

            let size = board.len() as isize - 1;
            if current.pos == (size, size) {
                return distances[&(size, size)];
            }

            for dir in DIRECTIONS {
                let current_pos = current.pos;
                let dir_x = current_pos.0 + dir.0;
                let dir_y = current_pos.1 + dir.1;

                if let Some(row) = &board.get(dir_x as usize) {
                    if let Some(&cell) = &row.get(dir_y as usize) {
                        if !visited.contains(&(dir_x, dir_y)) {
                            let copy = distances.clone();
                            let v = distances.entry((dir_x, dir_y)).or_insert(i32::MAX);

                            if *v > copy[&current.pos] + cell {
                                *v = copy[&current.pos] + cell;

                                let node = Node {
                                    pos: (dir_x, dir_y),
                                    weight: cell,
                                    distance: *v,
                                    parent: Some(current.pos),
                                };

                                heap.push(node);
                            }
                        }
                    }
                }
            }
        }

        let (right, bottom) = distances.keys().max().unwrap();

        distances[&(*right, *bottom)]
    }

    pub fn solve(file: &str) -> i32 {
        let lines = read_input(file);
        let input = parse_input(lines);

        dikjstra(&input, (0isize, 0isize))
    }
}

fn main() {
    let input = include_str!("input.txt");
    let result = solution::solve(input);

    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_days() {
        let input = include_str!("test_input.txt");

        let result = solution::solve(input);
        let expected = 40;

        assert_eq!(result, expected);
    }
}
