mod day_utils;

use std::collections::HashSet;
use utils::read_to_string;

struct Guard {
    position: (i32, i32),
    direction: (i32, i32),
    visited_positions: HashSet<(i32, i32)>,
}

impl Guard {
    fn new(start_position: (i32, i32), direction: (i32, i32)) -> Self {
       let mut visited_positions = HashSet::new();
        visited_positions.insert(start_position);
        Self {
            position: start_position,
            direction,
            visited_positions,
        }
    }

    fn move_forward(&mut self, map: Vec<Vec<char>>, max_bounds: (i32, i32)) -> bool {
        let next_position = (
            self.position.0 + self.direction.0,
            self.position.1 + self.direction.1,
        );
        if !is_within_bounds(next_position, max_bounds) {
            return false;
        } else if map[next_position.0 as usize][next_position.1 as usize] != '#' {
            self.position = next_position;
            self.visited_positions.insert(self.position);
        } else {
            self.rotate_clockwise();
        }

        true
    }

    fn rotate_clockwise(&mut self) {
        self.direction = (self.direction.1, -self.direction.0);
    }
}

fn main() {
    let input = read_to_string("../data/input_d6.txt").unwrap();
    let map = day_utils::parse_input(&input);
    let guard_index = find_char(&map, '^').unwrap();
    let mut guard = Guard::new(guard_index, (-1, 0));
    let max_bounds = max_indices(&map).unwrap();
    while guard.move_forward(map.clone(), max_bounds) {}
    println!("The answer is: {}", guard.visited_positions.len())
}

fn is_within_bounds(position: (i32, i32), max_bounds: (i32, i32)) -> bool {
    position.0 >= 0 && position.0 <= max_bounds.0 &&
        position.1 >= 0 && position.1 <= max_bounds.1
}

fn find_char(map: &Vec<Vec<char>>, target: char) -> Option<(i32, i32)> {
    for (row_index, row) in map.iter().enumerate() {
        if let Some(col_index) = row.iter().position(|&c| c == target) {
            return Some((row_index as i32, col_index as i32));
        }
    }
    None
}

fn max_indices(map: &Vec<Vec<char>>) -> Option<(i32, i32)> {
    if map.is_empty() {
        return None;
    }

    let max_row_index = map.len() - 1;
    let max_col_index = map[max_row_index].len() - 1;

    Some((max_row_index as i32, max_col_index as i32))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_forward() {
        // Arrange
        let mut guard = Guard {
            position: (0, 0),
            direction: (1, 0),
        };

        // Act
        guard.move_forward();
        guard.move_forward();

        // Assert
        assert_eq!(guard.position, (2, 0));
    }

    #[test]
    fn test_rotate_clockwise() {
        let mut guard = Guard {
            position: (0, 0),
            direction: (0, 1),
        };
        guard.rotate_clockwise();
        assert_eq!(guard.direction, (1, 0));
        guard.rotate_clockwise();
        assert_eq!(guard.direction, (0, -1));
        guard.rotate_clockwise();
        assert_eq!(guard.direction, (-1, 0));
        guard.rotate_clockwise();
        assert_eq!(guard.direction, (0, 1));
    }
}