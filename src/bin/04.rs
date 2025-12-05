advent_of_code::solution!(4);

use grid::*;

fn text_to_grid(input: &str) -> Grid<char> {
    let lines: Vec<&str> = input.lines().collect();
    let height = lines.len();
    let width = lines.iter().map(|line| line.len()).max().unwrap_or(0);
    let mut grid = Grid::new(height, width);

    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            grid[(y, x)] = ch;
        }
    }
    grid
}

fn count_neighbors(grid: &Grid<char>, row: usize, col: usize) -> u32 {
    const DIRECTIONS: [(i32, i32); 8] = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),           (0, 1),
        (1, -1),  (1, 0),  (1, 1)
    ];

    DIRECTIONS.iter()
        // filter_map : pour chaque direction, on renvoie Some(true/false) ou None
        .filter_map(|&(dy, dx)| {
            // checked_add_signed : ajoute un décalage signé en restant dans les bornes
            // ex : 0.checked_add_signed(-1) => None, 3.checked_add_signed(-1) => Some(2)
            let new_row = row.checked_add_signed(dy as isize)?;
            let new_col = col.checked_add_signed(dx as isize)?;

            // .then(...) : si la condition est vraie, renvoie Some(...)
            (new_row < grid.rows() && new_col < grid.cols())
                .then(|| grid[(new_row, new_col)] == '@')
        })
        // Ici on ne garde que les voisins qui sont vraiment '@'
        .filter(|&is_occupied| is_occupied == true)
        // count : compte le nombre de true restants
        .count() as u32
}

fn get_accessible(map: &Grid<char>) -> Vec<(usize, usize)> {
    (0..map.rows())
        // flat_map : pour chaque row, on crée un itérateur sur toutes les cols,
        // puis on "aplatit" tout en une seule suite (row, col)
        // ex simple :
        //   (0..2).flat_map(|r| (0..3).map(move |c| (r, c)))
        //   => (0,0),(0,1),(0,2),(1,0),(1,1),(1,2)
        .flat_map(|row| (0..map.cols()).map(move |col| (row, col)))
        // filter : on garde seulement les cases qui nous intéressent
        .filter(|&(row, col)| {
            // On ne regarde que les '@' dont le nombre de voisins < 4
            map[(row, col)] == '@' && count_neighbors(&map, row, col) < 4
        })
        // count : combien de cases respectent la condition
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let map = text_to_grid(input);

    Some(get_accessible(&map)
        .len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut map = text_to_grid(input);
    let mut total_removed = 0;

    loop{
        let to_remove: Vec<(usize, usize)> = get_accessible(&map);

        if to_remove.is_empty() {
            break;
        }

        for (row, col) in to_remove.iter() {
            map[(*row, *col)] = '.';
            total_removed += 1;
        }
    }
    Some(total_removed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
