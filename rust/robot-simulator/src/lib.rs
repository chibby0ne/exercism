// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Clone for Direction {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Robot {
    position: (i32, i32),
    orientation: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot {
            position: (x, y),
            orientation: d,
        }
    }

    pub fn turn_right(self) -> Self {
        Robot {
            position: self.position,
            orientation: match self.orientation {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
            },
        }
    }

    pub fn turn_left(self) -> Self {
        Robot {
            position: self.position,
            orientation: match self.orientation {
                Direction::North => Direction::West,
                Direction::East => Direction::North,
                Direction::South => Direction::East,
                Direction::West => Direction::South,
            },
        }
    }

    pub fn advance(self) -> Self {
        Robot {
            position: match self.orientation {
                Direction::North => (self.position.0, self.position.1 + 1),
                Direction::East => (self.position.0 + 1, self.position.1),
                Direction::South => (self.position.0, self.position.1 - 1),
                Direction::West => (self.position.0 - 1, self.position.1),
            },
            orientation: self.orientation,
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        let mut robot = self;
        for c in instructions.chars() {
            robot = match c {
                'L' => robot.turn_left(),
                'R' => robot.turn_right(),
                _ => robot.advance(),
            };
        }
        robot
    }

    pub fn position(&self) -> (i32, i32) {
        self.position
    }

    pub fn direction(&self) -> &Direction {
        &self.orientation
    }
}
