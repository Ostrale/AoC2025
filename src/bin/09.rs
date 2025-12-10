advent_of_code::solution!(9);

#[derive(Debug)]
struct Coordinate2D {
    x: u64,
    y: u64,
}

#[derive(Debug)]
struct Rectangle<'a> {
    points: (&'a Coordinate2D, &'a Coordinate2D),
    surface: u64,
}

impl<'a> Rectangle<'a> {
    fn new(p1: &'a Coordinate2D, p2: &'a Coordinate2D) -> Self {
        let width = p1.x.abs_diff(p2.x) + 1;
        let height = p1.y.abs_diff(p2.y) + 1;

        let surface = width.saturating_mul(height);

        Self {
            points: (p1, p2),
            surface,
        }
    }

    fn contains(&self, point: &Coordinate2D) -> bool {
        let min_x = self.points.0.x.min(self.points.1.x);
        let max_x = self.points.0.x.max(self.points.1.x);
        let min_y = self.points.0.y.min(self.points.1.y);
        let max_y = self.points.0.y.max(self.points.1.y);

        point.x > min_x && point.x < max_x && point.y > min_y && point.y < max_y
    }

    fn intersects_segments(
        &self,
        vertical_segments: &[Vec<(u64, u64)>],
        horizontal_segments: &[Vec<(u64, u64)>],
    ) -> bool {
        let min_x = self.points.0.x.min(self.points.1.x);
        let max_x = self.points.0.x.max(self.points.1.x);
        let min_y = self.points.0.y.min(self.points.1.y);
        let max_y = self.points.0.y.max(self.points.1.y);

        // Vérifier les segments verticaux strictement à l'intérieur
        for x in (min_x + 1)..max_x {
            if x < vertical_segments.len() as u64 {
                for &(y_min, y_max) in &vertical_segments[x as usize] {
                    // Le segment vertical a une partie strictement à l'intérieur
                    let seg_start_inside = y_min > min_y && y_min < max_y;
                    let seg_end_inside = y_max > min_y && y_max < max_y;
                    let seg_crosses = y_min <= min_y && y_max >= max_y;

                    if seg_start_inside || seg_end_inside || seg_crosses {
                        return true;
                    }
                }
            }
        }

        // Vérifier les segments horizontaux strictement à l'intérieur
        for y in (min_y + 1)..max_y {
            if y < horizontal_segments.len() as u64 {
                for &(x_min, x_max) in &horizontal_segments[y as usize] {
                    // Le segment horizontal a une partie strictement à l'intérieur
                    let seg_start_inside = x_min > min_x && x_min < max_x;
                    let seg_end_inside = x_max > min_x && x_max < max_x;
                    let seg_crosses = x_min <= min_x && x_max >= max_x;

                    if seg_start_inside || seg_end_inside || seg_crosses {
                        return true;
                    }
                }
            }
        }

        false
    }
}

struct Polygone {
    vertical_segments: Vec<Vec<(u64, u64)>>, // index = x, values = Vec[(y_min, y_max)]
    horizontal_segments: Vec<Vec<(u64, u64)>>, // index = y, values = Vec[(x_min, x_max)]
}

impl Polygone {
    fn new(points: &Vec<Coordinate2D>) -> Self {
        let max_x = points.iter().map(|p| p.x).max().unwrap_or(0);
        let max_y = points.iter().map(|p| p.y).max().unwrap_or(0);

        let mut vertical_segments = vec![Vec::new(); (max_x + 1) as usize];
        let mut horizontal_segments = vec![Vec::new(); (max_y + 1) as usize];

        for i in 0..points.len() {
            let current = &points[i];
            let next = &points[(i + 1) % points.len()];

            if current.x == next.x {
                // Segment vertical
                let x = current.x as usize;
                let y_min = current.y.min(next.y);
                let y_max = current.y.max(next.y);
                vertical_segments[x].push((y_min, y_max));
            } else if current.y == next.y {
                // Segment horizontal
                let y = current.y as usize;
                let x_min = current.x.min(next.x);
                let x_max = current.x.max(next.x);
                horizontal_segments[y].push((x_min, x_max));
            }
        }

        Polygone {
            vertical_segments,
            horizontal_segments,
        }
    }

    fn ray_tracing(&self, point: &Coordinate2D) -> bool {
        if point.x < self.vertical_segments.len() as u64 {
            for (y_min, y_max) in &self.vertical_segments[point.x as usize] {
                if point.y >= *y_min && point.y <= *y_max {
                    return true; // Sur le contour
                }
            }
            for (x_min, x_max) in &self.horizontal_segments[point.y as usize] {
                if point.x >= *x_min && point.x <= *x_max {
                    return true; // Sur le contour
                }
            }
        }

        let mut intersections = 0;
        let mut inside_segment = false;

        for x in (0..(point.x as usize)).rev() {
            let mut current_inside = false;

            for (y_min, y_max) in &self.vertical_segments[x] {
                if point.y >= *y_min && point.y <= *y_max {
                    current_inside = true;
                    break;
                }
            }

            for (x_min, x_max) in &self.horizontal_segments[point.y as usize] {
                if x >= *x_min as usize && x <= *x_max as usize {
                    current_inside = true;
                    break;
                }
            }

            if !current_inside && inside_segment {
                intersections += 1;
            }

            inside_segment = current_inside;
        }

        intersections % 2 == 1
    }
}

fn parse(input: &str) -> Vec<Coordinate2D> {
    input
        .lines()
        .map(|line| {
            let mut it = line.split(",");
            let x = it.next().unwrap().parse::<u64>().unwrap();
            let y = it.next().unwrap().parse::<u64>().unwrap();
            Coordinate2D { x, y }
        })
        .collect()
}

fn get_all_rectangle(coords: &[Coordinate2D]) -> Vec<Rectangle<'_>> {
    let mut rects = Vec::new();
    for i in 0..coords.len() {
        for j in i + 1..coords.len() {
            rects.push(Rectangle::new(&coords[i], &coords[j]));
        }
    }
    rects
}

pub fn part_one(input: &str) -> Option<u64> {
    let coords = parse(input);
    let rectangles = get_all_rectangle(&coords);
    rectangles.iter().map(|r| r.surface).max()
}

pub fn part_two(input: &str) -> Option<u64> {
    let coords = parse(input);
    let coords_polygone = parse(input);
    let polygone = Polygone::new(&coords_polygone);

    let mut rectangles = get_all_rectangle(&coords);
    rectangles.sort_unstable_by(|a, b| b.surface.cmp(&a.surface));

    rectangles.retain(|rectangle| !coords.iter().any(|coord| rectangle.contains(coord)));

    rectangles.retain(|rectangle| {
        let intersects = rectangle
            .intersects_segments(&polygone.vertical_segments, &polygone.horizontal_segments);
        !intersects
    });

    'rectangles: for rectangle in rectangles {
        let (p1, p2) = rectangle.points;

        let center = Coordinate2D {
            x: (p1.x + p2.x) / 2,
            y: (p1.y + p2.y) / 2,
        };

        if !polygone.ray_tracing(&center) {
            continue 'rectangles;
        }

        // si on arrive ici → Tous les points valides → on renvoie le rectangle
        return Some(rectangle.surface);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
