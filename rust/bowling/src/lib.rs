const NUM_PINS_PER_FRAME: u16 = 10;
const LAST_FRAME: u16 = 10;

#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

enum BallThrow {
    First,
    Second,
    Fill,
}

enum FrameCase {
    OpenFrame((u16, u16)),
    Spare(u16),
    Strike,
    LastFrame((u16, u16, u16)),
}

pub struct BowlingGame {
    frames: u16,
    ball_throw: BallThrow,
    score_per_frame: Vec<FrameCase>,
    pins_left_in_frame: u16,
    prev_throw_in_frame: u16,
    prev_prev_throw_in_frame: u16,
}

impl Default for BowlingGame {
    fn default() -> Self {
        Self::new()
    }
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            frames: 1,
            ball_throw: BallThrow::First,
            score_per_frame: Vec::new(),
            pins_left_in_frame: NUM_PINS_PER_FRAME,
            prev_throw_in_frame: 0,
            prev_prev_throw_in_frame: 0,
        }
    }

    fn reset_to_new_frame(&mut self) {
        self.frames += 1;
        self.ball_throw = BallThrow::First;
        self.pins_left_in_frame = NUM_PINS_PER_FRAME;
        self.prev_throw_in_frame = 0;
        self.prev_prev_throw_in_frame = 0;
    }

    fn update_to_second_throw(&mut self, pins_left: u16) {
        self.ball_throw = BallThrow::Second;
        self.pins_left_in_frame = pins_left;
    }

    fn add_fill_throw(&mut self, pins_left: u16) {
        self.ball_throw = BallThrow::Fill;
        self.pins_left_in_frame = pins_left;
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.frames > NUM_PINS_PER_FRAME {
            return Err(Error::GameComplete);
        }
        match pins {
            0..=NUM_PINS_PER_FRAME => {
                let pins_left = self.pins_left_in_frame as i16 - pins as i16;
                if pins_left < 0 {
                    return Err(Error::NotEnoughPinsLeft);
                }
                match (pins_left, &self.ball_throw) {
                    (0, BallThrow::First) => {
                        if self.frames == LAST_FRAME {
                            self.update_to_second_throw(NUM_PINS_PER_FRAME);
                            self.prev_throw_in_frame = pins;
                        } else {
                            self.score_per_frame.push(FrameCase::Strike);
                            self.reset_to_new_frame();
                        }
                    }
                    (_, BallThrow::First) => {
                        self.prev_throw_in_frame = pins;
                        self.update_to_second_throw(pins_left as u16);
                    }
                    (0, BallThrow::Second) => {
                        if self.frames == LAST_FRAME {
                            self.add_fill_throw(NUM_PINS_PER_FRAME);
                            self.prev_prev_throw_in_frame = self.prev_throw_in_frame;
                            self.prev_throw_in_frame = pins;
                        } else {
                            self.score_per_frame
                                .push(FrameCase::Spare(self.prev_throw_in_frame));
                            self.reset_to_new_frame();
                        }
                    }
                    (_, BallThrow::Second) => {
                        if self.frames == LAST_FRAME
                            && self.prev_throw_in_frame == NUM_PINS_PER_FRAME
                        {
                            self.add_fill_throw(pins_left as u16);
                            self.prev_prev_throw_in_frame = self.prev_throw_in_frame;
                            self.prev_throw_in_frame = pins;
                        } else {
                            self.score_per_frame
                                .push(FrameCase::OpenFrame((self.prev_throw_in_frame, pins)));
                            self.reset_to_new_frame();
                        }
                    }
                    (_, BallThrow::Fill) => {
                        self.score_per_frame.push(FrameCase::LastFrame((
                            self.prev_prev_throw_in_frame,
                            self.prev_throw_in_frame,
                            pins,
                        )));
                        self.reset_to_new_frame();
                    }
                }
                Ok(())
            }
            _ => Err(Error::NotEnoughPinsLeft),
        }
    }

    pub fn score(&self) -> Option<u16> {
        if self.score_per_frame.len() < LAST_FRAME as usize {
            return None;
        }
        let sum = self
            .score_per_frame
            .iter()
            .enumerate()
            .map(|(c, v)| match v {
                FrameCase::OpenFrame((first, second)) => first + second,
                FrameCase::Spare(_) => {
                    if let Some(next_frame) = self.score_per_frame.get(c + 1) {
                        match next_frame {
                            FrameCase::OpenFrame((first, _)) => first + NUM_PINS_PER_FRAME,
                            FrameCase::Spare(first) => first + NUM_PINS_PER_FRAME,
                            FrameCase::Strike => 2 * NUM_PINS_PER_FRAME,
                            FrameCase::LastFrame((first, _, _)) => first + NUM_PINS_PER_FRAME,
                        }
                    } else {
                        NUM_PINS_PER_FRAME
                    }
                }
                FrameCase::Strike => {
                    if let Some(next_frame) = self.score_per_frame.get(c + 1) {
                        match next_frame {
                            FrameCase::OpenFrame((first, second)) => {
                                first + second + NUM_PINS_PER_FRAME
                            }
                            FrameCase::Spare(_) => 2 * NUM_PINS_PER_FRAME,
                            FrameCase::Strike => {
                                if let Some(next_next_frame) = self.score_per_frame.get(c + 2) {
                                    match next_next_frame {
                                        FrameCase::OpenFrame((first, _)) => {
                                            first + 2 * NUM_PINS_PER_FRAME
                                        }
                                        FrameCase::Spare(first) => first + 2 * NUM_PINS_PER_FRAME,
                                        FrameCase::Strike => 3 * NUM_PINS_PER_FRAME,
                                        FrameCase::LastFrame((first, _, _)) => {
                                            first + 2 * NUM_PINS_PER_FRAME
                                        }
                                    }
                                } else {
                                    2 * NUM_PINS_PER_FRAME
                                }
                            }
                            FrameCase::LastFrame((first, second, _)) => {
                                first + second + NUM_PINS_PER_FRAME
                            }
                        }
                    } else {
                        NUM_PINS_PER_FRAME
                    }
                }
                FrameCase::LastFrame((first, second, third)) => first + second + third,
            })
            .sum();
        Some(sum)
    }
}
