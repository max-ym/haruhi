use thiserror::Error;

pub trait Widget: Element {
}

pub trait Layer<E: Element>: Element {
    fn inner(&self) -> &E;
}

pub trait Element {
    fn width(&self) -> f32;

    fn height(&self) -> f32;
}

pub trait Builder {
    type Element: Element;

    fn build(self) -> Self::Element;
}

pub trait Composite<E: Element>: Element {
}

pub struct Blank;

impl Widget for Blank {}

impl Element for Blank {
    fn width(&self) -> f32 {
        0.0
    }

    fn height(&self) -> f32 {
        0.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PercentSigned(f32);

impl Eq for PercentSigned {}

impl PartialOrd for PercentSigned {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(unsafe { self.0.partial_cmp(&other.0).unwrap_unchecked() })
    }
}

impl Ord for PercentSigned {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { self.0.partial_cmp(&other.0).unwrap_unchecked() }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum PercentSignedError {
    #[error("value is out of bounds")]
    OutOfBounds,

    #[error("value is not a number")]
    NotANumber,

    #[error("value is infinite")]
    Infinite,
}

impl PercentSigned {
    pub fn new(value: f32) -> Result<Self, PercentSignedError> {
        if value.is_nan() {
            return Err(PercentSignedError::NotANumber);
        }

        if value.is_infinite() {
            return Err(PercentSignedError::Infinite);
        }

        if !(-1.0..=1.0).contains(&value) {
            return Err(PercentSignedError::OutOfBounds);
        }

        Ok(Self(value))
    }

    pub unsafe fn new_unchecked(value: f32) -> Self {
        debug_assert!(Self::new(value).is_ok());
        Self(value)
    }

    pub const fn unit_neg() -> Self {
        Self(-1.0)
    }

    pub const fn unit_pos() -> Self {
        Self(1.0)
    }

    pub const fn zero() -> Self {
        Self(0.0)
    }
}

impl From<PercentSigned> for f32 {
    fn from(value: PercentSigned) -> Self {
        value.0
    }
}

impl TryFrom<f32> for PercentSigned {
    type Error = PercentSignedError;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<PercentSigned> for f64 {
    fn from(value: PercentSigned) -> Self {
        value.0 as f64
    }
}

impl TryFrom<f64> for PercentSigned {
    type Error = PercentSignedError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::new(value as f32)
    }
}

impl std::fmt::Display for PercentSigned {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}%", self.0 * 100.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Unsigned(f32);

impl Eq for Unsigned {}

impl PartialOrd for Unsigned {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(unsafe { self.0.partial_cmp(&other.0).unwrap_unchecked() })
    }
}

impl Ord for Unsigned {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { self.0.partial_cmp(&other.0).unwrap_unchecked() }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum UnsignedError {
    #[error("value is not a number")]
    NotANumber,

    #[error("value is infinite")]
    Infinite,

    #[error("value is negative")]
    Negative,
}

impl Unsigned {
    pub fn new(value: f32) -> Result<Self, UnsignedError> {
        if value.is_nan() {
            return Err(UnsignedError::NotANumber);
        }

        if value.is_infinite() {
            return Err(UnsignedError::Infinite);
        }

        if value < 0.0 {
            return Err(UnsignedError::Negative);
        }

        Ok(Self(value))
    }

    pub unsafe fn new_unchecked(value: f32) -> Self {
        debug_assert!(Self::new(value).is_ok());
        Self(value)
    }

    pub const fn zero() -> Self {
        Self(0.0)
    }

    pub const fn one() -> Self {
        Self(1.0)
    }
}

impl From<Unsigned> for f32 {
    fn from(value: Unsigned) -> Self {
        value.0
    }
}

impl TryFrom<f32> for Unsigned {
    type Error = UnsignedError;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<Unsigned> for f64 {
    fn from(value: Unsigned) -> Self {
        value.0 as f64
    }
}

impl TryFrom<f64> for Unsigned {
    type Error = UnsignedError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::new(value as f32)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Positive(f32);

impl Eq for Positive {}

impl PartialOrd for Positive {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(unsafe { self.0.partial_cmp(&other.0).unwrap_unchecked() })
    }
}

impl Ord for Positive {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { self.0.partial_cmp(&other.0).unwrap_unchecked() }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum PositiveError {
    #[error("value is not a number")]
    NotANumber,

    #[error("value is infinite")]
    Infinite,

    #[error("value is negative or zero")]
    NegativeOrZero,
}

impl Positive {
    pub fn new(value: f32) -> Result<Self, PositiveError> {
        if value.is_nan() {
            return Err(PositiveError::NotANumber);
        }

        if value.is_infinite() {
            return Err(PositiveError::Infinite);
        }

        if value <= 0.0 {
            return Err(PositiveError::NegativeOrZero);
        }

        Ok(Self(value))
    }

    pub unsafe fn new_unchecked(value: f32) -> Self {
        debug_assert!(Self::new(value).is_ok());
        Self(value)
    }

    pub const fn one() -> Self {
        Self(1.0)
    }
}

impl From<Positive> for f32 {
    fn from(value: Positive) -> Self {
        value.0
    }
}

impl TryFrom<f32> for Positive {
    type Error = PositiveError;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<Positive> for f64 {
    fn from(value: Positive) -> Self {
        value.0 as f64
    }
}

impl TryFrom<f64> for Positive {
    type Error = PositiveError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::new(value as f32)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PercentUnsigned(f32);

impl Eq for PercentUnsigned {}

impl PartialOrd for PercentUnsigned {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(unsafe { self.0.partial_cmp(&other.0).unwrap_unchecked() })
    }
}

impl Ord for PercentUnsigned {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { self.0.partial_cmp(&other.0).unwrap_unchecked() }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum PercentUnsignedError {
    #[error("value is not a number")]
    NotANumber,

    #[error("value is infinite")]
    Infinite,

    #[error("value is negative")]
    Negative,

    #[error("value is out of bounds")]
    OutOfBounds,
}

impl PercentUnsigned {
    pub fn new(value: f32) -> Result<Self, PercentUnsignedError> {
        if value.is_nan() {
            return Err(PercentUnsignedError::NotANumber);
        }

        if value.is_infinite() {
            return Err(PercentUnsignedError::Infinite);
        }

        if value < 0.0 {
            return Err(PercentUnsignedError::Negative);
        }

        if value > 1.0 {
            return Err(PercentUnsignedError::OutOfBounds);
        }

        Ok(Self(value))
    }

    pub unsafe fn new_unchecked(value: f32) -> Self {
        debug_assert!(Self::new(value).is_ok());
        Self(value)
    }

    pub const fn zero() -> Self {
        Self(0.0)
    }

    pub const fn one() -> Self {
        Self(1.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub red: PercentUnsigned,
    pub green: PercentUnsigned,
    pub blue: PercentUnsigned,
    pub alpha: PercentUnsigned,
}

#[derive(Clone)]
pub struct Shader; // TODO
