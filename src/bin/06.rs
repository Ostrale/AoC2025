advent_of_code::solution!(6);

enum Operation {
    Add(Vec<u64>),
    Mul(Vec<u64>),
}

impl Operation {
    fn apply(&self) -> u64 {
        match self {
            Operation::Add(n) => n.iter().sum(),
            Operation::Mul(n) => n.iter().product(),
        }
    }
}

fn input_to_operation_1(input: &str) -> Vec<Operation> {
    let rows: Vec<Vec<_>> = input
        .lines()
        .map(|line| line.split_ascii_whitespace().collect())
        .collect();

    let num_cols = rows.get(0).map_or(0, |row| row.len());

    let mut vv_str: Vec<Vec<&str>> = (0..num_cols)
        .map(|col_idx| {
            rows.iter()
                .filter_map(|row| row.get(col_idx).copied())
                .collect()
        })
        .collect();

    vv_str
        .iter_mut()
        .filter_map(|v| {
            let mat_op = (*v).pop();

            let number: Vec<u64> = v.iter().filter_map(|&c| c.parse::<u64>().ok()).collect();

            match mat_op.unwrap() {
                "+" => Some(Operation::Add(number)),
                "*" => Some(Operation::Mul(number)),
                _ => None,
            }
        })
        .collect()
}

fn input_to_operation_2(input: &str) -> Vec<Operation> {
    let lines: Vec<&str> = input.lines().collect();
    let max_len = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    let mut cols: Vec<String> = (0..max_len)
        .map(|col_idx| {
            lines
                .iter()
                .map(|line| line.chars().nth(col_idx).unwrap_or(' '))
                .collect()
        })
        .collect();

    cols.reverse();

    let mut ops = Vec::new();
    let mut current: Vec<String> = Vec::new();

    for col in cols {
        if col.trim().is_empty() {
            if !current.is_empty() {
                if let Some(op) = parse_problem(&current) {
                    ops.push(op);
                }
                current.clear();
            }
        } else {
            current.push(col);
        }
    }

    if !current.is_empty() {
        if let Some(op) = parse_problem(&current) {
            ops.push(op);
        }
    }

    ops
}

fn parse_problem(cols: &[String]) -> Option<Operation> {
    let last_col = cols.last()?;
    let op_char = last_col.chars().last()?;

    let numbers: Vec<u64> = cols
        .iter()
        .map(|col| {
            col.chars()
                .take(col.len().saturating_sub(1))
                .filter(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse::<u64>()
                .unwrap_or(0)
        })
        .filter(|&n| n > 0)
        .collect();

    match op_char {
        '+' => Some(Operation::Add(numbers)),
        '*' => Some(Operation::Mul(numbers)),
        _ => None,
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input_to_operation_1(input)
            .iter()
            .map(|op| op.apply())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input_to_operation_2(input)
            .iter()
            .map(|op| op.apply())
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
