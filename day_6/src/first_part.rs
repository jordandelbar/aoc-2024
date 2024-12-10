use crate::day_utils::is_within_bounds;
use std::collections::HashSet;

pub struct Guard {
    pub position: (i32, i32),
    pub direction: (i32, i32),
    pub visited_positions: HashSet<(i32, i32)>,
    pub collision_spots: HashSet<(i32, i32)>,
    pub infinite_loop: bool,
}

impl Guard {
    pub fn new(start_position: (i32, i32), direction: (i32, i32)) -> Self {
        let mut visited_positions = HashSet::new();
        let mut collision_spots = HashSet::new();
        visited_positions.insert(start_position);
        Self {
            position: start_position,
            direction,
            visited_positions,
            collision_spots,
            infinite_loop: false,
        }
    }

    pub fn move_forward(&mut self, map: &Vec<Vec<char>>, max_bounds: (i32, i32)) -> bool {
        let next_position = (
            self.position.0 + self.direction.0,
            self.position.1 + self.direction.1,
        );
        if !is_within_bounds(next_position, max_bounds) {
            self.infinite_loop = true;
            return false;
        } else if self.collision_spots.contains(&next_position) {
            return false;
        } else if map[next_position.0 as usize][next_position.1 as usize] != '#' {
            self.position = next_position;
            self.visited_positions.insert(self.position);
        } else {
            self.rotate_clockwise();
            self.collision_spots.insert(self.position);
        }

        true
    }

    fn rotate_clockwise(&mut self) {
        self.direction = (self.direction.1, -self.direction.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_forward() {
        // Arrange
        let mut guard = Guard {
            position: (0, 0),
            direction: (0, 1),
            visited_positions: HashSet::new(),
        };

        // Act
        guard.move_forward(vec![vec!['.', '.', '.']], (0, 2));
        guard.move_forward(vec![vec!['.', '.', '.']], (0, 2));

        // Assert
        assert_eq!(guard.position, (0, 2));
    }

    #[test]
    fn test_rotate_clockwise() {
        let mut guard = Guard {
            position: (0, 0),
            direction: (0, 1),
            visited_positions: HashSet::new(),
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
