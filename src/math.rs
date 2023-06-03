use thiserror::Error;

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

impl std::ops::Add for Unsigned {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        unsafe { Self::new_unchecked(self.0 + rhs.0) }
    }
}

impl std::ops::AddAssign for Unsigned {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl std::ops::Sub for Unsigned {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        unsafe { Self::new_unchecked(self.0 - rhs.0) }
    }
}

impl std::ops::SubAssign for Unsigned {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl std::ops::Mul for Unsigned {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        unsafe { Self::new_unchecked(self.0 * rhs.0) }
    }
}

impl std::ops::MulAssign for Unsigned {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl std::ops::Div for Unsigned {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        unsafe { Self::new_unchecked(self.0 / rhs.0) }
    }
}

impl std::ops::DivAssign for Unsigned {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl std::ops::Rem for Unsigned {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        unsafe { Self::new_unchecked(self.0 % rhs.0) }
    }
}

impl std::ops::RemAssign for Unsigned {
    fn rem_assign(&mut self, rhs: Self) {
        *self = *self % rhs;
    }
}

impl std::ops::Neg for Unsigned {
    type Output = Self;

    fn neg(self) -> Self::Output {
        unsafe { Self::new_unchecked(-self.0) }
    }
}

impl std::ops::Add<Positive> for Unsigned {
    type Output = Self;

    fn add(self, rhs: Positive) -> Self::Output {
        unsafe { Self::new_unchecked(self.0 + rhs.0) }
    }
}

impl std::ops::AddAssign<Positive> for Unsigned {
    fn add_assign(&mut self, rhs: Positive) {
        *self = *self + rhs;
    }
}

impl std::ops::Sub<Positive> for Unsigned {
    type Output = Self;

    fn sub(self, rhs: Positive) -> Self::Output {
        unsafe { Self::new_unchecked(self.0 - rhs.0) }
    }
}

impl std::ops::SubAssign<Positive> for Unsigned {
    fn sub_assign(&mut self, rhs: Positive) {
        *self = *self - rhs;
    }
}

impl std::ops::Mul<Positive> for Unsigned {
    type Output = Self;

    fn mul(self, rhs: Positive) -> Self::Output {
        unsafe { Self::new_unchecked(self.0 * rhs.0) }
    }
}

impl std::ops::MulAssign<Positive> for Unsigned {
    fn mul_assign(&mut self, rhs: Positive) {
        *self = *self * rhs;
    }
}

impl std::ops::Div<Positive> for Unsigned {
    type Output = Self;

    fn div(self, rhs: Positive) -> Self::Output {
        unsafe { Self::new_unchecked(self.0 / rhs.0) }
    }
}

impl std::ops::DivAssign<Positive> for Unsigned {
    fn div_assign(&mut self, rhs: Positive) {
        *self = *self / rhs;
    }
}

impl std::ops::Rem<Positive> for Unsigned {
    type Output = Self;

    fn rem(self, rhs: Positive) -> Self::Output {
        unsafe { Self::new_unchecked(self.0 % rhs.0) }
    }
}

impl std::ops::RemAssign<Positive> for Unsigned {
    fn rem_assign(&mut self, rhs: Positive) {
        *self = *self % rhs;
    }
}

impl PartialEq<Positive> for Unsigned {
    fn eq(&self, other: &Positive) -> bool {
        self.0.eq(&other.0)
    }
}

impl PartialOrd<Positive> for Unsigned {
    fn partial_cmp(&self, other: &Positive) -> Option<std::cmp::Ordering> {
        Some(unsafe { self.0.partial_cmp(&other.0).unwrap_unchecked() })
    }
}

impl From<Positive> for Unsigned {
    fn from(value: Positive) -> Self {
        unsafe { Self::new_unchecked(value.0) }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Number(f32);

impl Eq for Number {}

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(unsafe { self.0.partial_cmp(&other.0).unwrap_unchecked() })
    }
}

impl Ord for Number {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { self.0.partial_cmp(&other.0).unwrap_unchecked() }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum NumberError {
    #[error("value is not a number")]
    NotANumber,

    #[error("value is infinite")]
    Infinite,
}

impl Number {
    pub fn new(value: f32) -> Result<Self, NumberError> {
        if value.is_nan() {
            return Err(NumberError::NotANumber);
        }

        if value.is_infinite() {
            return Err(NumberError::Infinite);
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

impl std::ops::Neg for Number {
    type Output = Self;

    fn neg(self) -> Self::Output {
        unsafe { Self::new_unchecked(-self.0) }
    }
}

impl std::ops::Add for Number {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        unsafe { Self::new_unchecked(self.0 + rhs.0) }
    }
}

impl std::ops::AddAssign for Number {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl std::ops::Sub for Number {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        unsafe { Self::new_unchecked(self.0 - rhs.0) }
    }
}

impl std::ops::SubAssign for Number {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl std::ops::Mul for Number {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        unsafe { Self::new_unchecked(self.0 * rhs.0) }
    }
}

impl std::ops::MulAssign for Number {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl std::ops::Div for Number {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        unsafe { Self::new_unchecked(self.0 / rhs.0) }
    }
}

impl std::ops::DivAssign for Number {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl std::ops::Rem for Number {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        unsafe { Self::new_unchecked(self.0 % rhs.0) }
    }
}

impl std::ops::RemAssign for Number {
    fn rem_assign(&mut self, rhs: Self) {
        *self = *self % rhs;
    }
}

impl PartialEq<Number> for f32 {
    fn eq(&self, other: &Number) -> bool {
        self.eq(&other.0)
    }
}

impl PartialOrd<Number> for f32 {
    fn partial_cmp(&self, other: &Number) -> Option<std::cmp::Ordering> {
        Some(unsafe { self.partial_cmp(&other.0).unwrap_unchecked() })
    }
}

impl From<Number> for f32 {
    fn from(value: Number) -> Self {
        value.0
    }
}

impl std::ops::Add<Number> for f32 {
    type Output = Self;

    fn add(self, rhs: Number) -> Self::Output {
        self + rhs.0
    }
}

impl std::ops::AddAssign<Number> for f32 {
    fn add_assign(&mut self, rhs: Number) {
        *self = *self + rhs;
    }
}

impl std::ops::Sub<Number> for f32 {
    type Output = Self;

    fn sub(self, rhs: Number) -> Self::Output {
        self - rhs.0
    }
}

impl std::ops::SubAssign<Number> for f32 {
    fn sub_assign(&mut self, rhs: Number) {
        *self = *self - rhs;
    }
}

impl std::ops::Mul<Number> for f32 {
    type Output = Self;

    fn mul(self, rhs: Number) -> Self::Output {
        self * rhs.0
    }
}

impl std::ops::MulAssign<Number> for f32 {
    fn mul_assign(&mut self, rhs: Number) {
        *self = *self * rhs;
    }
}

impl std::ops::Div<Number> for f32 {
    type Output = Self;

    fn div(self, rhs: Number) -> Self::Output {
        self / rhs.0
    }
}

impl std::ops::DivAssign<Number> for f32 {
    fn div_assign(&mut self, rhs: Number) {
        *self = *self / rhs;
    }
}

impl std::ops::Rem<Number> for f32 {
    type Output = Self;

    fn rem(self, rhs: Number) -> Self::Output {
        self % rhs.0
    }
}

impl std::ops::RemAssign<Number> for f32 {
    fn rem_assign(&mut self, rhs: Number) {
        *self = *self % rhs;
    }
}

impl std::ops::Add<f32> for Number {
    type Output = f32;

    fn add(self, rhs: f32) -> Self::Output {
        self.0 + rhs
    }
}

impl std::ops::Sub<f32> for Number {
    type Output = f32;

    fn sub(self, rhs: f32) -> Self::Output {
        self.0 - rhs
    }
}

impl std::ops::Mul<f32> for Number {
    type Output = f32;

    fn mul(self, rhs: f32) -> Self::Output {
        self.0 * rhs
    }
}

impl std::ops::Div<f32> for Number {
    type Output = f32;

    fn div(self, rhs: f32) -> Self::Output {
        self.0 / rhs
    }
}

impl std::ops::Rem<f32> for Number {
    type Output = f32;

    fn rem(self, rhs: f32) -> Self::Output {
        self.0 % rhs
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

impl std::ops::Add<Unsigned> for Number {
    type Output = Self;

    fn add(self, rhs: Unsigned) -> Self::Output {
        unsafe { Self::new_unchecked(self.0 + rhs.0) }
    }
}

impl std::ops::AddAssign<Unsigned> for Number {
    fn add_assign(&mut self, rhs: Unsigned) {
        *self = *self + rhs;
    }
}

impl std::ops::Sub<Unsigned> for Number {
    type Output = Self;

    fn sub(self, rhs: Unsigned) -> Self::Output {
        unsafe { Self::new_unchecked(self.0 - rhs.0) }
    }
}

impl std::ops::SubAssign<Unsigned> for Number {
    fn sub_assign(&mut self, rhs: Unsigned) {
        *self = *self - rhs;
    }
}

impl std::ops::Mul<Unsigned> for Number {
    type Output = Self;

    fn mul(self, rhs: Unsigned) -> Self::Output {
        unsafe { Self::new_unchecked(self.0 * rhs.0) }
    }
}

impl std::ops::MulAssign<Unsigned> for Number {
    fn mul_assign(&mut self, rhs: Unsigned) {
        *self = *self * rhs;
    }
}

impl std::ops::Div<Unsigned> for Number {
    type Output = Self;

    fn div(self, rhs: Unsigned) -> Self::Output {
        unsafe { Self::new_unchecked(self.0 / rhs.0) }
    }
}

impl std::ops::DivAssign<Unsigned> for Number {
    fn div_assign(&mut self, rhs: Unsigned) {
        *self = *self / rhs;
    }
}

impl std::ops::Rem<Unsigned> for Number {
    type Output = Self;

    fn rem(self, rhs: Unsigned) -> Self::Output {
        unsafe { Self::new_unchecked(self.0 % rhs.0) }
    }
}

impl std::ops::RemAssign<Unsigned> for Number {
    fn rem_assign(&mut self, rhs: Unsigned) {
        *self = *self % rhs;
    }
}

impl PartialEq<Unsigned> for f32 {
    fn eq(&self, other: &Unsigned) -> bool {
        self.eq(&other.0)
    }
}

impl PartialOrd<Unsigned> for f32 {
    fn partial_cmp(&self, other: &Unsigned) -> Option<std::cmp::Ordering> {
        Some(unsafe { self.partial_cmp(&other.0).unwrap_unchecked() })
    }
}

impl std::ops::Add<Unsigned> for f32 {
    type Output = Self;

    fn add(self, rhs: Unsigned) -> Self::Output {
        self + rhs.0
    }
}

impl std::ops::AddAssign<Unsigned> for f32 {
    fn add_assign(&mut self, rhs: Unsigned) {
        *self = *self + rhs;
    }
}

impl std::ops::Sub<Unsigned> for f32 {
    type Output = Self;

    fn sub(self, rhs: Unsigned) -> Self::Output {
        self - rhs.0
    }
}

impl std::ops::SubAssign<Unsigned> for f32 {
    fn sub_assign(&mut self, rhs: Unsigned) {
        *self = *self - rhs;
    }
}

impl std::ops::Mul<Unsigned> for f32 {
    type Output = Self;

    fn mul(self, rhs: Unsigned) -> Self::Output {
        self * rhs.0
    }
}

impl std::ops::MulAssign<Unsigned> for f32 {
    fn mul_assign(&mut self, rhs: Unsigned) {
        *self = *self * rhs;
    }
}

impl std::ops::Div<Unsigned> for f32 {
    type Output = Self;

    fn div(self, rhs: Unsigned) -> Self::Output {
        self / rhs.0
    }
}

impl std::ops::DivAssign<Unsigned> for f32 {
    fn div_assign(&mut self, rhs: Unsigned) {
        *self = *self / rhs;
    }
}

impl std::ops::Rem<Unsigned> for f32 {
    type Output = Self;

    fn rem(self, rhs: Unsigned) -> Self::Output {
        self % rhs.0
    }
}

impl std::ops::RemAssign<Unsigned> for f32 {
    fn rem_assign(&mut self, rhs: Unsigned) {
        *self = *self % rhs;
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
