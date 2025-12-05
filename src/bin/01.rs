advent_of_code::solution!(1);
use regex::Regex;

enum Operation {
    Add(i32),
    Sub(i32),
}

impl Operation {
    fn apply(&self, x: i32) -> i32 {
        let result = match self {
            Operation::Add(n) => x + n,
            Operation::Sub(n) => x - n,
        };

        result.rem_euclid(100)
    }


    fn apply_with_overflow(&self, x: i32) -> (i32, u64) {
        let mut current = x;
        let mut overflow = 0;

        match self {
            Operation::Add(n) => {
                for _ in 0..*n {
                    current = (current + 1) % 100;
                    if current == 0 {
                        overflow += 1;
                    }
                }
            }
            Operation::Sub(n) => {
                for _ in 0..*n {
                    current = (current - 1 + 100) % 100;
                    if current == 0 {
                        overflow += 1;
                    }
                }
            }
        };

        (current, overflow)
    }
}

fn rotate(rotation: &str) -> Operation {
    let re = Regex::new(r"^([LR])(\d*)").unwrap();
    let caps = re.captures(rotation).unwrap();
    let direction = &caps[1];
    let nb : i32 = caps[2].parse().unwrap();
    match direction {
        "L" => Operation::Sub(nb),
        "R" => Operation::Add(nb),
        _ => panic!("Invalid rotation direction"),
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut answer: u64 = 0;

    let mut dial: i32 = 50;
    for line in input.lines() {
        let operation = rotate(line.trim());
        dial = operation.apply(dial);
        if dial == 0 {answer += 1};
    }
    Some(answer)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut answer: u64 = 0;

    let mut dial: i32 = 50;
    for line in input.lines() {
        let operation = rotate(line.trim());
        let (new_dial, overflow) = operation.apply_with_overflow(dial);
        dial = new_dial;
        answer += overflow;
    }
    Some(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
