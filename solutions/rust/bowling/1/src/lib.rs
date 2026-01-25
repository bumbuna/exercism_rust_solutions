#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    rolls: Vec<u16>,
    frame: u16,
    is_second_throw: bool,
    fill_balls: u16,
    is_complete: bool,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            rolls: vec![],
            frame: 1,
            is_second_throw: false,
            fill_balls: 0,
            is_complete: false,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        if self.frame > 10 {
            //fill balls
            if self.fill_balls == 0 {
                return Err(Error::GameComplete);
            } else {
                self.fill_balls -= 1;
                if self.fill_balls == 0 {
                    self.is_complete = true;
                }
            }
        }

        if !self.is_second_throw {
            //strike
            if pins == 10 {
                self.frame += 1;
                if self.frame == 11 {
                    self.fill_balls = 2;
                } else {
                    self.is_complete = true;
                }
            } else {
                self.is_second_throw = true;
            }
        } else {
            if pins + self.rolls.last().unwrap() > 10 {
                return Err(Error::NotEnoughPinsLeft);
            }
            self.frame += 1;
            if self.frame == 11 {
                //
                if pins + self.rolls.last().unwrap() == 10 {
                    //spare
                    self.fill_balls = 1;
                } else {
                    self.is_complete = true;
                }
            }
            self.is_second_throw = false;
        }

        self.rolls.push(pins);
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if !self.is_complete || self.fill_balls > 0 {
            None
        } else {
            let mut total_score: u16 = 0;
            let mut roll_index = 0;
            let mut frames = 0;
            while frames < 10 {
                if self.rolls[roll_index] == 10 {
                    //strike
                    total_score += 10 + self.rolls[roll_index + 1] + self.rolls[roll_index + 2];
                    roll_index += 1;
                } else if self.rolls[roll_index] + self.rolls[roll_index + 1] == 10 {
                    //spare
                    total_score += 10 + self.rolls[roll_index + 2];
                    roll_index += 2;
                } else {
                    //open
                    total_score += self.rolls[roll_index] + self.rolls[roll_index + 1];
                    roll_index += 2;
                }
                frames += 1;
            }
            Some(total_score)
        }
    }
}
