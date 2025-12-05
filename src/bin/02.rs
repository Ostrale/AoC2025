advent_of_code::solution!(2);
use regex::Regex;
use std::collections::HashSet;

trait DigitOperations {
    fn count_digits(&self) -> u32;
    fn split_number(&self) -> (u64, u64);
    fn double_number(&self) -> u64;
    fn repeat_pattern(&self, n: u32) -> u64;
}

impl DigitOperations for u64 {
    fn count_digits(&self) -> u32 {
        if self == &0 {
            return 1;
        }
        (*self).ilog10() + 1
    }

    fn split_number(&self) -> (u64, u64) {
        let nb_digit = self.count_digits();
        let half_digits = nb_digit / 2;
        let divisor = 10u64.pow(half_digits);
        (self / divisor, self % divisor)
    }

    fn double_number(&self) -> u64 {
        let nb_digits = self.count_digits();
        let multiplier = 10u64.pow(nb_digits);
        self * multiplier + self
    }

    fn repeat_pattern(&self, n: u32) -> u64 {
        let nb_digits = self.count_digits();
        let multiplier = 10u64.pow(nb_digits);

        let mut result = 0u64;
        for _ in 0..n {
            result = result * multiplier + self;
        }
        result
    }
}

fn get_range(range: &str) -> Result<(u64, u64), Box<dyn std::error::Error>> {
    let re = Regex::new(r"(\d+)-(\d+)")?;
    let caps = re.captures(range)
        .ok_or("Format invalide, attendu: nombre-nombre")?;

    let start = caps[1].parse::<u64>()?;
    let end = caps[2].parse::<u64>()?;

    Ok((start, end))
}

fn sum_mirror_numbers(range: (u64, u64)) -> Option<u64> {
    let mut start = range.0;
    let last = range.1;

    let mut nb_digit = start.count_digits();

    // Si le nombre de chiffres est impair, on ne peut pas avoir de miroirs
    // On modifie le start pour qu'il soit pair en l'incrémentant
    if nb_digit % 2 != 0 {
        nb_digit += 1;
        start = 10u64.pow(nb_digit - 1);
        if start > last {
            return None;
        }
    }

    let mut answer = 0u64;
    let mut number = start.split_number().0.double_number();

    while number <= last {
        if number >= range.0 {
            answer += number;
        }
        number = (number.split_number().0 + 1).double_number();
    }

    Some(answer)
}

fn sum_repeating_patterns(range: (u64, u64)) -> Option<u64> {
    let start = range.0;
    let last = range.1;

    let min_digits = start.count_digits();
    let max_digits = last.count_digits();

    let mut found_numbers = HashSet::new();

    for nb_digit in min_digits..=max_digits {
        for pattern_size in 1..=nb_digit.div_ceil(2) {
            if nb_digit % pattern_size != 0 || nb_digit / pattern_size < 2 {
                continue;
            }

            let repetitions = nb_digit / pattern_size;
            let min_pattern = 10u64.pow(pattern_size - 1);
            let max_pattern = 10u64.pow(pattern_size);

            for pattern in min_pattern..max_pattern {
                let number = pattern.repeat_pattern(repetitions);

                if number >= start && number <= last {
                    found_numbers.insert(number);
                }
            }
        }
    }

    Some(found_numbers.iter().sum())
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut answer = 0u64;

    for str_range in input.split(',') {
        let range = get_range(str_range).ok()?;
        if let Some(partial) = sum_mirror_numbers(range) {
            answer += partial;
        }
    }
    Some(answer)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut answer = 0u64;

    for str_range in input.split(',') {
        let range = get_range(str_range).ok()?;
        if let Some(partial) = sum_repeating_patterns(range) {
            answer += partial;
        }
    }
    Some(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}

