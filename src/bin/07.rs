advent_of_code::solution!(7);

use grid::Grid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    Splitter,
    Start,
}

struct Playground {
    grid: Grid<Cell>,
    start: (usize, usize), // (row, col)
}

fn parse_input(input: &str) -> Playground {
    let lines: Vec<&str> = input.lines().collect();
    let rows = lines.len();
    let cols = lines[0].len();

    let mut cells = Vec::with_capacity(rows * cols);
    let mut start = None;

    for (row, line) in lines.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            let cell = match ch {
                '.' => Cell::Empty,
                '^' => Cell::Splitter,
                'S' => {
                    start = Some((row, col));
                    Cell::Start
                }
                _ => panic!("Invalid character: {}", ch),
            };
            cells.push(cell);
        }
    }

    Playground {
        grid: Grid::from_vec(cells, cols),
        start: start.expect("No start position found"),
    }
}

fn split(playground: &Playground) -> (u64, Vec<u64>) {
    let mut count = 0u64;
    let mut step: Vec<u64> = vec![0; playground.grid.cols()];
    step[playground.start.1] = 1;

    for row_idx in 0..playground.grid.rows() {
        let mut next_step = vec![0u64; playground.grid.cols()];

        for (col, &cell) in playground.grid.iter_row(row_idx).enumerate() {
            if step[col] == 0 {
                continue;
            }

            match cell {
                Cell::Splitter => {
                    count += 1;

                    if col > 0 {
                        next_step[col - 1] += step[col];
                    }
                    if col < playground.grid.cols() - 1 {
                        next_step[col + 1] += step[col];
                    }
                }
                Cell::Empty | Cell::Start => {
                    next_step[col] += step[col];
                }
            }
        }

        step = next_step;
    }

    (count, step)
}

pub fn part_one(input: &str) -> Option<u64> {
    let playground = parse_input(input);
    Some(split(&playground).0)
}

pub fn part_two(input: &str) -> Option<u64> {
    let playground = parse_input(input);
    Some(split(&playground).1.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
