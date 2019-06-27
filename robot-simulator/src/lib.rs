// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    pos: (i32, i32),
    dir: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot { pos: (x, y), dir: d }
    }

    pub fn turn_right(self) -> Self {
        let Robot { pos: pos, dir: d } = self;
        Robot {
            pos: pos,
            dir: match d {
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
                Direction::North => Direction::East
            },
        }
    }

    pub fn turn_left(self) -> Self {
        let Robot { pos: pos, dir: d } = self;
        Robot {
            pos: pos,
            dir: match d {
                Direction::East => Direction::North,
                Direction::South => Direction::East,
                Direction::West => Direction::South,
                Direction::North => Direction::West
            },
        }
    }

    pub fn advance(self) -> Self {
        let Robot { pos: (x, y), dir: d } = self;
        match d {
            Direction::East => Robot::new(x + 1, y, d),
            Direction::South => Robot::new(x, y - 1, d),
            Direction::West => Robot::new(x - 1, y, d),
            Direction::North => Robot::new(x, y + 1, d)
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        let mut robot = self;
        for i in instructions.chars() {
            robot = match i {
                'R' => robot.turn_right(),
                'L' => robot.turn_left(),
                'A' => robot.advance(),
                _ => panic!("Illegal instruction")
            }
        }
        robot
    }

    pub fn position(&self) -> (i32, i32) {
        self.pos
    }

    pub fn direction(&self) -> &Direction {
        &self.dir
    }
}
