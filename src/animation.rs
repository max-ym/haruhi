use std::time::Duration;
use crate::math::*;
use thiserror::Error;

pub trait Animation<T> {
    fn advance(&mut self, seconds: Positive);

    fn get(&self) -> T;
}

#[derive(Debug, Clone, Copy)]
pub struct Linear {
    start: Number,
    end: Number,
    duration: Positive,
    elapsed: Unsigned,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum NewAnimationError {
    #[error("duration must be greater than zero")]
    DurationZero,
}

impl Linear {
    pub fn new(start: Number, end: Number, duration: Duration) -> Result<Self, NewAnimationError> {
        Ok(Self {
            start,
            end,
            duration: duration.as_secs_f32().try_into().map_err(|_| NewAnimationError::DurationZero)?,
            elapsed: Unsigned::zero(),
        })
    }
}

impl Animation<Number> for Linear {
    fn advance(&mut self, seconds: Positive) {
        self.elapsed += seconds;

        if self.elapsed > self.duration {
            self.elapsed = self.duration.into();
        }
    }

    fn get(&self) -> Number {
        let progress = self.elapsed / self.duration;
        let delta = self.end - self.start;
        self.start + delta * progress
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Exponential {
    start: Number,
    end: Number,
    duration: Positive,
    elapsed: Unsigned,
}

impl Exponential {
    pub fn new(start: Number, end: Number, duration: Duration) -> Result<Self, NewAnimationError> {
        Ok(Self {
            start,
            end,
            duration: duration.as_secs_f32().try_into().map_err(|_| NewAnimationError::DurationZero)?,
            elapsed: Unsigned::zero(),
        })
    }
}

impl Animation<Number> for Exponential {
    fn advance(&mut self, seconds: Positive) {
        self.elapsed += seconds;

        if self.elapsed > self.duration {
            self.elapsed = self.duration.into();
        }
    }

    fn get(&self) -> Number {
        let progress = self.elapsed / self.duration;
        let delta = self.end - self.start;
        self.start + delta * progress * progress
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Bounce {
    start: Number,
    end: Number,
    duration: Positive,
    elapsed: Unsigned,
}

impl Bounce {
    pub fn new(start: Number, end: Number, duration: Duration) -> Result<Self, NewAnimationError> {
        Ok(Self {
            start,
            end,
            duration: duration.as_secs_f32().try_into().map_err(|_| NewAnimationError::DurationZero)?,
            elapsed: Unsigned::zero(),
        })
    }
}

impl Animation<Number> for Bounce {
    fn advance(&mut self, seconds: Positive) {
        self.elapsed += seconds;

        if self.elapsed > self.duration {
            self.elapsed = self.duration.into();
        }
    }

    fn get(&self) -> Number {
        let progress = self.elapsed / self.duration;
        let delta = self.end - self.start;
        Number::new(self.start + delta * progress * progress * (3.0 - 2.0 * progress)).unwrap()
    }
}
