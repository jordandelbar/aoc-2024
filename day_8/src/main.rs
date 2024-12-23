use std::collections::{HashMap, HashSet};
use utils::{read_file_to_string, string_to_grid};

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let input = read_file_to_string("../data/input_d8.txt").unwrap();
    let grid: Vec<Vec<char>> = string_to_grid(&input, false);
    let grid_size = (grid[0].len() as i32, grid.len() as i32);
    let antennas = find_all_antennas(&grid);
    let result = calculate_antinodes(&antennas, grid_size);
    println!("{:?}", result.len());
}

fn find_all_antennas(grid: &Vec<Vec<char>>) -> HashMap<char, Vec<Point>> {
    let mut antennas = HashMap::new();

    for (y, row) in grid.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c != '.' {
                antennas.entry(c).or_insert_with(Vec::new).push(Point {
                    x: x as i32,
                    y: y as i32,
                });
            }
        }
    }

    antennas
}

fn calculate_antinodes(
    antennas: &HashMap<char, Vec<Point>>,
    grid_size: (i32, i32),
) -> HashSet<Point> {
    let mut antinodes: HashSet<Point> = HashSet::new();

    for positions in antennas.values() {
        for i in 0..positions.len() {
            for j in 0..positions.len() {
                let (p1, p2) = (positions[i], positions[j]);

                let (antinode_p1, antinode_p2) = compute_antinodes_point(&p1, &p2);
                if assert_point_in_grid(&antinode_p1, grid_size) {
                    antinodes.insert(antinode_p1);
                }
                if assert_point_in_grid(&antinode_p2, grid_size) {
                    antinodes.insert(antinode_p2);
                }
            }
        }
    }

    antinodes
}

fn compute_antinodes_point(point_a: &Point, point_b: &Point) -> (Point, Point) {
    let dx = (point_b.x - point_a.x).abs();
    let dy = (point_b.y - point_a.y).abs();

    let antinode_a = Point {
        x: point_a.x - dx,
        y: point_a.y - dy,
    };
    let antinode_b = Point {
        x: point_b.x + dx,
        y: point_b.y + dy,
    };

    (antinode_a, antinode_b)
}

fn assert_point_in_grid(point: &Point, grid_length: (i32, i32)) -> bool {
    if point.x < 0 {
        return false;
    }
    if point.y < 0 {
        return false;
    }
    if point.x >= grid_length.0 {
        return false;
    }
    if point.y >= grid_length.1 {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_all_antennas() {
        let input = "....0..R...r\n....4....";
        let grid: Vec<Vec<char>> = string_to_grid(&input, false);

        let got = find_all_antennas(&grid);
        let mut want = HashMap::new();
        want.insert('0', vec![Point { x: 4, y: 0 }]);
        want.insert('R', vec![Point { x: 7, y: 0 }]);
        want.insert('r', vec![Point { x: 11, y: 0 }]);
        want.insert('4', vec![Point { x: 4, y: 1 }]);

        assert_eq!(got, want);
    }

    #[test]
    fn test_compute_antinodes_points() {
        let point_a = Point { x: 1, y: 1 };
        let point_b = Point { x: 2, y: 2 };

        let want = (Point { x: 0, y: 0 }, Point { x: 3, y: 3 });
        let got = compute_antinodes_point(&point_a, &point_b);

        assert_eq!(want, got);
    }

    #[test]
    fn test_assert_point_in_grid() {
        let grid_length = (5, 5);
        let cases = vec![
            (Point { x: -1, y: -1 }, false),
            (Point { x: 2, y: 6 }, false),
            (Point { x: 2, y: 6 }, false),
            (Point { x: 5, y: 2 }, false),
            (Point { x: 4, y: 4 }, true),
        ];

        for case in cases {
            let got = assert_point_in_grid(&case.0, grid_length);
            assert_eq!(got, case.1);
        }
    }
}
