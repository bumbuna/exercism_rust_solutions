// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    x: i32,
    y: i32,
    d: Direction
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self{x,y,d}
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        Self { 
            d: match self.d {
                Direction:: North => Direction::East,
                Direction:: South => Direction::West,
                Direction:: East => Direction::South,
                Direction:: West => Direction::North,
            },
            ..self
        }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
         Self { 
            d: match self.d {
                Direction:: North => Direction::West,
                Direction:: South => Direction::East,
                Direction:: East => Direction::North,
                Direction:: West => Direction::South,
            },
            ..self
         }
    }

    #[must_use]
    pub fn advance(self) -> Self {
        Self {
            y: match self.d {
                Direction::North => self.y+1,
                Direction::South => self.y-1,
                _ => self.y
            },
            x: match self.d {
                Direction::East => self.x+1,
                Direction::West => self.x-1,
                _ => self.x,
            },
        ..self
        }
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        // todo!("Follow the given sequence of instructions: {instructions}")
        let mut s = self;
        for c in instructions.chars() {
            s = match c {
                'A' => s.advance(),
                'L' => s.turn_left(),
                'R' => s.turn_right(),
                _ => panic!("Unknown instruction")
            }
        }
        s
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
