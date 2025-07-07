#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    rolls: Vec<u16>,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self { rolls: Vec::new() }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }
        if self.is_complete() {
            return Err(Error::GameComplete);
        }

        let len = self.rolls.len();
        let (frame, pos) = self.current_frame_and_position();

        // Validate pin count doesn't exceed frame total
        if frame <= 10 && pos == 1 && len > 0 {
            let prev = self.rolls[len - 1];
            if frame < 10 && prev + pins > 10 {
                return Err(Error::NotEnoughPinsLeft);
            }
            if frame == 10 && prev != 10 && prev + pins > 10 {
                return Err(Error::NotEnoughPinsLeft);
            }
        }

        // Additional validation for 10th frame bonus rolls
        if frame == 10 && pos == 2 && len >= 2 {
            // This is the third roll in the 10th frame (second bonus roll)
            let mut i = 0;
            // Find start of 10th frame
            for _ in 0..9 {
                i += if self.rolls[i] == 10 { 1 } else { 2 };
            }

            let first_roll = self.rolls[i];
            let second_roll = self.rolls[i + 1];

            // If first roll was a strike, second and third rolls follow normal rules
            if first_roll == 10 {
                // If second roll wasn't a strike, second + third can't exceed 10
                if second_roll != 10 && second_roll + pins > 10 {
                    return Err(Error::NotEnoughPinsLeft);
                }
            }
        }

        self.rolls.push(pins);
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if !self.is_scoreable() {
            return None;
        }

        let mut total = 0;
        let mut i = 0;

        for _ in 0..10 {
            if self.rolls[i] == 10 {
                // Strike
                total += 10 + self.rolls[i + 1] + self.rolls[i + 2];
                i += 1;
            } else if self.rolls[i] + self.rolls[i + 1] == 10 {
                // Spare
                total += 10 + self.rolls[i + 2];
                i += 2;
            } else {
                // Open frame
                total += self.rolls[i] + self.rolls[i + 1];
                i += 2;
            }
        }

        Some(total)
    }

    fn current_frame_and_position(&self) -> (u16, u16) {
        let mut i = 0;
        let mut frame = 1;

        // Process frames 1-9
        while i < self.rolls.len() && frame < 10 {
            let frame_start = i;
            i += if self.rolls[i] == 10 { 1 } else { 2 };
            if i > self.rolls.len() {
                return (frame, (self.rolls.len() - frame_start) as u16);
            }
            frame += 1;
        }

        // Frame 10 (can have up to 3 rolls)
        if frame == 10 {
            let pos = (self.rolls.len() - i) as u16;
            (10, pos)
        } else {
            (frame, 0)
        }
    }

    fn is_complete(&self) -> bool {
        let len = self.rolls.len();
        if len < 18 {
            return false;
        }

        // Find start of 10th frame
        let mut i = 0;
        for _ in 0..9 {
            i += if self.rolls[i] == 10 { 1 } else { 2 };
        }

        let tenth_frame_rolls = len - i;
        if tenth_frame_rolls < 2 {
            return false;
        }

        // Check if 10th frame needs bonus roll
        let first_roll = self.rolls[i];
        let second_roll = self.rolls[i + 1];
        let needs_bonus = first_roll == 10 || first_roll + second_roll == 10;

        if needs_bonus {
            tenth_frame_rolls >= 3
        } else {
            tenth_frame_rolls == 2
        }
    }

    fn is_scoreable(&self) -> bool {
        if self.rolls.is_empty() {
            return false;
        }

        let mut i = 0;

        // Check we have enough rolls for frames 1-9
        for _ in 0..9 {
            if i >= self.rolls.len() {
                return false;
            }
            if self.rolls[i] == 10 {
                i += 1;
            } else {
                if i + 1 >= self.rolls.len() {
                    return false;
                }
                i += 2;
            }
        }

        // Check 10th frame
        let remaining = self.rolls.len() - i;
        if remaining < 2 {
            return false;
        }

        let first = self.rolls[i];
        let second = self.rolls[i + 1];
        let needs_bonus = first == 10 || first + second == 10;

        if needs_bonus { remaining >= 3 } else { remaining >= 2 }
    }
}
