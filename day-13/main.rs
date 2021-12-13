#![allow(dead_code)]

fn read_input(input: &str) -> Vec<String> {
    input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect::<Vec<_>>()
}

mod solution {
    type Coord = (usize, usize);
    type Grid = Vec<Vec<char>>;
    type Fold = (char, usize);

    pub fn parse_input(lines: &[String]) -> (Vec<Coord>, Vec<Fold>) {
        let iter = lines.iter();
        let mut coords = vec![];
        let mut folds = vec![];

        iter.for_each(|line| {
            let split = line.split(',');

            if split.count() == 2 {
                let split: Vec<_> = line
                    .split(',')
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect();

                if let [x, y] = split[..] {
                    coords.push((x, y));
                }
            } else {
                let fold = &line.as_str()[11..];
                let split: Vec<_> = fold.split('=').collect();

                if let [axis, position] = split[..] {
                    folds.push((
                        axis.chars().next().unwrap(),
                        position.parse::<usize>().unwrap(),
                    ));
                }
            }
        });
        (coords, folds)
    }

    fn get_max(coords: &[Coord]) -> (usize, usize) {
        let mut max_x = 0;
        let mut max_y = 0;
        for (x, y) in coords {
            if x > &max_x {
                max_x = *x;
            }
            if y > &max_y {
                max_y = *y;
            }
        }

        (max_x + 1, max_y + 1)
    }

    fn print_grid(grid: &Grid) {
        for row in grid {
            println!();
            for cell in row {
                print!("{}", cell);
            }
        }
    }

    fn fold_y(grid: &mut Grid, row: usize) {
        let grid_copy = grid.clone();
        for i in row..grid.len() {
            let fold_index = row - (i - row);
            for j in 0..grid[i].len() {
                if grid_copy[i][j] == '#' {
                    grid[fold_index][j] = grid_copy[i][j];
                }
            }
        }
        grid.truncate(row);
    }

    fn fold_x(grid: &mut Grid, column: usize) {
        let grid_copy = grid.clone();
        for i in 0..grid.len() {
            for j in column..grid[i].len() {
                let fold_index = column - (j - column);
                if grid_copy[i][j] == '#' {
                    grid[i][fold_index] = grid_copy[i][j];
                }
            }
        }

        for row in grid {
            row.truncate(column);
        }
    }

    fn fold(grid: &mut Grid, fold: &Fold) {
        if let ('y', row) = fold {
            fold_y(grid, *row);
        } else {
            fold_x(grid, fold.1);
        }
    }

    fn count_dots(grid: &Grid) -> i32 {
        let mut n = 0;
        for row in grid {
            for &p in row {
                if p == '#' {
                    n += 1;
                }
            }
        }
        n
    }
    pub fn solve(lines: &[String]) -> i32 {
        let (coords, folds) = parse_input(lines);

        let (max_x, max_y) = get_max(&coords);
        let mut grid: Grid = vec![vec![' '; max_x]; max_y];

        coords.iter().for_each(|&(x, y)| {
            grid[y][x] = '#';
        });

        for f in folds {
            fold(&mut grid, &f);
            println!();
            print_grid(&grid);
        }

        count_dots(&grid)
    }
}

fn main() {
    let file = include_str!("input.txt");
    let lines = read_input(file);
    let result = solution::solve(&lines);

    println!("Number of dots: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let file = include_str!("test_input_1.txt");
        let lines = read_input(file);

        let result = solution::solve(&lines);
        let expected = 17;
        assert_eq!(expected, result);
    }

    // #[test]
    // fn test_input_2() {
    //     let file = include_str!("test_input_2.txt");
    //     let lines = read_input(file);

    //     let result = solution::solve(&lines);
    //     let expected = 103;

    //     assert_eq!(expected, result);
    // }
}
