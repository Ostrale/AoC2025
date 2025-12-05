advent_of_code::solution!(5);

fn parse(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    // 1) découper en deux blocs autour de la ligne vide
    let (ranges_block, ids_block) = input.split_once("\n\n").unwrap();

    // 2) parser les intervalles "a-b" -> (a, b)
    let ranges: Vec<(u64, u64)> = ranges_block
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            (
                start.trim().parse::<u64>().unwrap(),
                end.trim().parse::<u64>().unwrap(),
            )
        })
        .collect();

    // 3) parser les IDs (une valeur par ligne)
    let ids: Vec<u64> = ids_block
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| line.trim().parse::<u64>().unwrap())
        .collect();

    (ranges, ids)
}

fn merge_intervals(mut intervals: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    intervals.sort_unstable_by(|left, right| left.cmp(right));
    let mut merged = Vec::new();
    let mut current = intervals[0];

    for (start, end) in intervals.into_iter().skip(1) {
        // si ça chevauche ou touche : on étend l'intervalle courant
        if start <= current.1 + 1 {
            current.1 = current.1.max(end);
        } else {
            // pas de chevauchement : on push l'ancien, on repart sur un nouveau
            merged.push(current);
            current = (start, end);
        }
    }

    // ne pas oublier le dernier
    merged.push(current);

    merged
}

fn is_fresh(id: &u64, ranges: &Vec<(u64, u64)>) -> bool {
    ranges
        .iter()
        .any(|(start, end)| *start <= *id && *id <= *end)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (ranges, ids) = parse(input);
    let merged_ranges = merge_intervals(ranges);
    Some(ids.iter().filter(|id| is_fresh(id, &merged_ranges)).count() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (ranges, _ids) = parse(input);
    let merged_ranges = merge_intervals(ranges);
    Some(
        merged_ranges
            .iter()
            .fold(0, |accumul, (left, right)| accumul + (right - left + 1)),
    )
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
        assert_eq!(result, Some(14));
    }
}
