#![feature(type_alias_impl_trait)]
// Permet d’utiliser `impl Trait` dans un alias de type.
// Exemple : `type Iter<'a> = impl Iterator<Item = u8>;`

#![feature(impl_trait_in_assoc_type)]
// Permet d’utiliser `impl Trait` dans un type associé d’un trait.
// Exemple : `type Iter<'a> = impl Iterator<Item = u8> + 'a;`

advent_of_code::solution!(3);

trait Digit {
    type Iter<'a> : Iterator<Item = u8>  // type MonType<'a>: TraitQueJeuxImplementer where Self: 'a;
    where
        Self: 'a;  // "Self doit vivre au moins aussi longtemps que 'a".

    fn digit<'a>(&'a self) -> Self::Iter<'a>;
}

impl Digit for str {
    // Spécifier le type concret pour Iter<'a>
    type Iter<'a> = impl Iterator<Item = u8> where Self: 'a;  // Pas encore stable [feature]

    fn digit<'a>(&'a self) -> Self::Iter<'a> {
        self.chars()
            .filter_map(|c| c.to_digit(10).map(|d| d as u8))
    }
}

#[allow(dead_code)]
fn get_max_jolts_k_batteries(bank: &str, k: usize) -> u64 {
    let count = bank.digit().count();
    let mut voltage: Vec<u8> = Vec::with_capacity(k);
    let mut checking_range: (usize, usize) = (0, count.saturating_sub(k - 1)); // On retire les k derniers

    for _ in 0..k {
        let (up_idx, current_digit) = bank.digit()
            .take(checking_range.1)
            .skip(checking_range.0)
            .enumerate()
            .fold(None, |accumul, (idx, value)| {
                match accumul {
                    None                                        => Some((idx, value)),
                    Some((_, best_value)) if value > best_value => Some((idx,value)),
                    Some(_) => accumul,
                }
            }).unwrap();
        voltage.push(current_digit);


        // mise à jour de la plage de recherche
        checking_range.0 = checking_range.0 + up_idx + 1;
        checking_range.1 += 1;
    }

    voltage.iter()
        .fold(0u64, |acc, &digit| acc * 10 + digit as u64)
}

#[allow(dead_code)]
fn get_max_jolts_k_batteries_monotonic(bank: &str, k: usize) -> u64 {
    let digits: Vec<u8> = bank.digit().collect();
    let to_remove = digits.len().saturating_sub(k);

    let (stack, _) = digits.iter()
        .fold((Vec::new(), 0), |(mut stack, mut removed), &digit| {
            // Retire les chiffres plus petits tant qu'on peut
            while stack.last().map_or(false, |&top| top < digit && removed < to_remove) {
                stack.pop();
                removed += 1;
            }
            stack.push(digit);
            (stack, removed)
        });

    // Garde seulement k éléments et convertit en u64
    stack.iter()
        .take(k)
        .fold(0u64, |acc, &digit| acc * 10 + digit as u64)
}


pub fn part_one(input: &str) -> Option<u64> {
    input.lines()
        .map(|line| get_max_jolts_k_batteries_monotonic(line, 2))
        .sum::<u64>()
        .into()
}

pub fn part_two(input: &str) -> Option<u64> {
    input.lines()
        .map(|line| get_max_jolts_k_batteries_monotonic(line, 12))
        .sum::<u64>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
