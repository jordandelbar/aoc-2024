use std::collections::HashSet;

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    fn is_within_bounds(&self, max_x: i32, max_y: i32) -> bool {
        self.x >= 0 && self.x <= max_x && self.y >= 0 && self.y <= max_y
    }
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct Direction {
    pub x: i32,
    pub y: i32,
}

pub struct Guard {
    pub position: Position,
    pub direction: Direction,
    pub visited_positions: HashSet<Position>,
    pub collision_spots: HashSet<(Position, Direction)>,
    pub infinite_loop: bool,
    pub counter: u32,
}

impl Guard {
    pub fn new(start_position: (i32, i32), start_direction: (i32, i32)) -> Self {
        let mut visited_positions = HashSet::new();
        let collision_spots = HashSet::new();
        let start_position = Position {
            x: start_position.0,
            y: start_position.1,
        };
        let start_direction = Direction {
            x: start_direction.0,
            y: start_direction.1,
        };
        visited_positions.insert(start_position.clone());
        Self {
            position: start_position,
            direction: start_direction,
            visited_positions,
            collision_spots,
            infinite_loop: false,
            counter: 0,
        }
    }

    pub fn move_forward(&mut self, map: &[Vec<char>], max_bounds: (i32, i32)) -> bool {
        let next_position = Position {
            x: self.position.x + self.direction.x,
            y: self.position.y + self.direction.y,
        };
        if !next_position.is_within_bounds(max_bounds.0, max_bounds.1) {
            return false;
        } else if self
            .collision_spots
            .contains(&(next_position.clone(), self.direction.clone()))
        {
            self.infinite_loop = true;
            return false;
        } else if self.counter > 10000 {
            self.infinite_loop = true;
            return false;
        } else if map[next_position.x as usize][next_position.y as usize] != '#'
            && map[next_position.x as usize][next_position.y as usize] != '0'
        {
            self.position = next_position;
            self.visited_positions.insert(self.position.clone());
            self.counter += 1;
        } else {
            self.rotate_clockwise();
            self.collision_spots
                .insert((self.position.clone(), self.direction.clone()));
        }

        true
    }

    fn rotate_clockwise(&mut self) {
        self.direction = Direction {
            x: self.direction.y,
            y: -self.direction.x,
        };
    }
}
