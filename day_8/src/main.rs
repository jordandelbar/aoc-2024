use std::collections::{HashMap, HashSet};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let input = std::fs::read_to_string("../data/input_d8.txt").expect("Failed to read input file");
    let grid = parse_grid(&input);
    let grid_size = (grid[0].len() as i32, grid.len() as i32);

    let antennas = find_all_antennas(&grid);
    let antinodes = calculate_antinodes(&antennas, grid_size);

    println!("{}", antinodes.len());
}

fn parse_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
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

fn is_within_grid(point: &Point, grid_size: (i32, i32)) -> bool {
    point.x >= 0 && point.y >= 0 && point.x < grid_size.0 && point.y < grid_size.1
}

fn calculate_antinodes(
    antennas: &HashMap<char, Vec<Point>>,
    grid_size: (i32, i32),
) -> HashSet<Point> {
    let mut antinodes: HashSet<Point> = HashSet::new();

    for positions in antennas.values() {
        let n = positions.len();
        for i in 0..n {
            for j in 0..n {
                if i == j {
                    continue;
                }

                let (p1, p2) = (positions[i], positions[j]);

                let dx = p2.x - p1.x;
                let dy = p2.y - p1.y;

                let antinode1 = Point {
                    x: p1.x - dx,
                    y: p1.y - dy,
                };
                let antinode2 = Point {
                    x: p2.x + dx,
                    y: p2.y + dy,
                };

                if is_within_grid(&antinode1, grid_size) {
                    antinodes.insert(antinode1);
                }
                if is_within_grid(&antinode2, grid_size) {
                    antinodes.insert(antinode2);
                }
            }
        }
    }

    antinodes
}
