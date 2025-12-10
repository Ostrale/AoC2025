advent_of_code::solution!(8);

use std::collections::HashMap;

#[derive(Debug)]
struct Coordinate3D {
    x: u64,
    y: u64,
    z: u64,
}
#[derive(Debug, Clone)]
struct Line {
    idx: (usize, usize),
    distance: f64,
}

trait StraightLineDistance {
    fn distance(&self, other: &Self) -> f64;
}

impl StraightLineDistance for Coordinate3D {
    fn distance(&self, other: &Self) -> f64 {
        let (dx, dy, dz) = (
            self.x as f64 - other.x as f64,
            self.y as f64 - other.y as f64,
            self.z as f64 - other.z as f64,
        );
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

#[derive(Debug)]
pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    /// Trouve la racine (représentant) du cluster contenant x
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    /// Fusionne les clusters contenant x et y
    /// Retourne true si fusion effectuée, false s'ils étaient déjà dans le même cluster
    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false;
        }

        if self.rank[root_x] < self.rank[root_y] {
            self.parent[root_x] = root_y;
        } else if self.rank[root_x] > self.rank[root_y] {
            self.parent[root_y] = root_x;
        } else {
            self.parent[root_y] = root_x;
            self.rank[root_x] += 1;
        }

        true
    }
}

fn parse(input: &str) -> Vec<Coordinate3D> {
    input
    .lines()
    .map(|line| {
        let mut it = line.split(",");
        let x = it.next().unwrap().parse::<u64>().unwrap();
        let y = it.next().unwrap().parse::<u64>().unwrap();
        let z = it.next().unwrap().parse::<u64>().unwrap();
        Coordinate3D { x, y, z }
    })
    .collect()
}

fn get_all_lines(coords: &[Coordinate3D]) -> Vec<Line> {
    let mut lines = Vec::new();
    for i in 0..coords.len() {
        for j in i + 1..coords.len() {
            let distance = coords[i].distance(&coords[j]);
            lines.push(Line {
                idx: (i, j),
                       distance,
            });
        }
    }
    lines
}

#[cfg(test)]
const LIMIT: usize = 10;

#[cfg(not(test))]
const LIMIT: usize = 1000;

pub fn part_one(input: &str) -> Option<u64> {
    let coords = parse(input);
    let mut lines = get_all_lines(&coords);
    lines.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());

    let mut clusters = UnionFind::new(coords.len());

    for line in lines.iter().take(LIMIT) {
        clusters.union(line.idx.0, line.idx.1);
    }

    let mut cluster_sizes: HashMap<usize, usize> = HashMap::new();
    for idx in 0..coords.len() {
        let root = clusters.find(idx);
        *cluster_sizes.entry(root).or_insert(0) += 1;
    }

    let mut sizes: Vec<usize> = cluster_sizes.iter().map(|(_, size)| *size).collect();
    sizes.sort_unstable_by(|a, b| b.cmp(a));
    Some(sizes.iter().take(3).product::<usize>() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let coords = parse(input);
    let mut lines = get_all_lines(&coords);
    lines.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());

    let mut clusters = UnionFind::new(coords.len());

    let mut last: &Line = &lines[0];

    for line in lines.iter() {
        clusters.union(line.idx.0, line.idx.1);

        let mut cluster_sizes: HashMap<usize, usize> = HashMap::new();
        for idx in 0..coords.len() {
            let root = clusters.find(idx);
            *cluster_sizes.entry(root).or_insert(0) += 1;
        }

        last = &line;
        if cluster_sizes.len() <= 1 {
            break;
        }
    }
    Some(coords[(*last).idx.0].x * coords[(*last).idx.1].x)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result,Some(25272));
    }
}
