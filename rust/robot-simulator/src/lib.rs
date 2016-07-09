#[derive(PartialEq, Debug)]
#[derive(Clone)]
#[derive(Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone)]
#[derive(Copy)]
pub struct Robot {
    x: isize,
    y: isize,
    d: Direction,
}

use Direction::*;

impl Robot {

    pub fn new(x: isize, y: isize, d: Direction) -> Self {
        Robot {
            x: x,
            y: y,
            d: d,
        }
    }

    pub fn turn_right(self) -> Self {
        let mut clone = self.clone();
        clone.d = match clone.d {
            North => East,
            East  => South,
            South => West,
            West  => North,
        };
        clone
    }

    pub fn turn_left(self) -> Self {
        let mut clone = self.clone();
        clone.d = match clone.d {
            North => West,
            West  => South,
            South => East,
            East  => North,
        };
        clone
    }

    pub fn advance(self) -> Self {
        let mut clone = self.clone();
        match clone.d {
            North => clone.y += 1,
            East  => clone.x += 1,
            South => clone.y -= 1,
            West  => clone.x -= 1,
        }
        clone
    }

    pub fn instructions(self, instructions: &str) -> Self {
        let mut clone = self.clone();
        for instruction in instructions.chars() {
            clone = clone.apply_instruction(instruction);
        }
        clone
    }

    fn apply_instruction(self, instruction: char) -> Self {
        match instruction {
            'A' => self.advance(),
            'R' => self.turn_right(),
            'L' => self.turn_left(),
            _   => panic!("I don't understand instruction `{}`", instruction),
        }
    }

    pub fn position(&self) -> (isize, isize) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
