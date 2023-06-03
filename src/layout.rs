use thiserror::Error;
use crate::base::*;

pub struct Row<E: Element> {
    elements: Vec<E>,
}

impl<E: Element> Row<E> {
    pub fn new(elements: Vec<E>) -> Self {
        Self { elements }
    }
}

impl<E: Element> Element for Row<E> {
    fn width(&self) -> f32 {
        self.elements
            .iter()
            .map(|e| e.width())
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(0.0)
    }

    fn height(&self) -> f32 {
        self.elements
            .iter()
            .map(|e| e.height())
            .sum()
    }
}

impl<E: Element> Composite<E> for Row<E> {}

pub struct Column<E: Element> {
    elements: Vec<E>,
}

impl<E: Element> Column<E> {
    pub fn new(elements: Vec<E>) -> Self {
        Self { elements }
    }
}

impl<E: Element> Element for Column<E> {
    fn width(&self) -> f32 {
        self.elements
            .iter()
            .map(|e| e.width())
            .sum()
    }

    fn height(&self) -> f32 {
        self.elements
            .iter()
            .map(|e| e.height())
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(0.0)
    }
}

impl<E: Element> Composite<E> for Column<E> {}

pub enum Direction {
    Horizontal,
    Vertical,
}

pub struct Array<E: Element> {
    elements: Vec<E>,
    direction: Direction,
}

impl<E: Element> Array<E> {
    pub fn new(elements: Vec<E>, direction: Direction) -> Self {
        Self { elements, direction }
    }
}

impl<E: Element> Element for Array<E> {
    fn width(&self) -> f32 {
        match self.direction {
            Direction::Horizontal => self.elements
                .iter()
                .map(|e| e.width())
                .sum(),
            Direction::Vertical => self.elements
                .iter()
                .map(|e| e.width())
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap_or(0.0),
        }
    }

    fn height(&self) -> f32 {
        match self.direction {
            Direction::Horizontal => self.elements
                .iter()
                .map(|e| e.height())
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap_or(0.0),
            Direction::Vertical => self.elements
                .iter()
                .map(|e| e.height())
                .sum(),
        }
    }
}

impl<E: Element> Composite<E> for Array<E> {}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Flex(f32);

impl Default for Flex {
    fn default() -> Self {
        Self(1.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Error)]
pub enum FlexError {
    #[error("flex cannot be NaN")]
    NotANumber,

    #[error("flex cannot be negative")]
    Negative,

    #[error("flex cannot be zero")]
    Zero,

    #[error("flex cannot be infinite")]
    Infinite,
}

impl Flex {
    pub fn new(flex: f32) -> Result<Self, FlexError> {
        if flex.is_nan() {
            Err(FlexError::NotANumber)
        } else if flex.is_sign_negative() {
            Err(FlexError::Negative)
        } else if flex == 0.0 {
            Err(FlexError::Zero)
        } else if flex.is_infinite() {
            Err(FlexError::Infinite)
        } else {
            Ok(Self(flex))
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum OverlapPolicy {
    /// The elements are cropped so that to avoid visual overlapping.
    /// Note as the default behavior is to center the elements so when cropped only
    /// center of each element will be visible but if alignment is applied then crop
    /// is made so that alignment part was visible.
    /// For left alignment the left part of an element will be visible.
    Crop,

    /// Stack one over another. Default behavior.
    #[default]
    Stack,
}

pub struct Align<E: Element> {
    element: E,
    x: PercentSigned,
    y: PercentSigned,
}

impl<E: Element> Align<E> {
    pub const fn new(element: E, x: PercentSigned, y: PercentSigned) -> Self {
        Self { element, x, y }
    }

    pub const fn x(&self) -> PercentSigned {
        self.x
    }

    pub const fn y(&self) -> PercentSigned {
        self.y
    }

    pub const fn left(element: E) -> Self {
        Self::new(element, PercentSigned::unit_neg(), PercentSigned::zero())
    }

    pub const fn right(element: E) -> Self {
        Self::new(element, PercentSigned::unit_pos(), PercentSigned::zero())
    }

    pub const fn top(element: E) -> Self {
        Self::new(element, PercentSigned::zero(), PercentSigned::unit_neg())
    }

    pub const fn bottom(element: E) -> Self {
        Self::new(element, PercentSigned::zero(), PercentSigned::unit_pos())
    }

    pub const fn center(element: E) -> Self {
        Self::new(element, PercentSigned::zero(), PercentSigned::zero())
    }

    pub const fn top_left(element: E) -> Self {
        Self::new(element, PercentSigned::unit_neg(), PercentSigned::unit_neg())
    }

    pub const fn top_right(element: E) -> Self {
        Self::new(element, PercentSigned::unit_pos(), PercentSigned::unit_neg())
    }

    pub const fn bottom_left(element: E) -> Self {
        Self::new(element, PercentSigned::unit_neg(), PercentSigned::unit_pos())
    }

    pub const fn bottom_right(element: E) -> Self {
        Self::new(element, PercentSigned::unit_pos(), PercentSigned::unit_pos())
    }
}

impl<E: Element> Element for Align<E> {
    fn width(&self) -> f32 {
        self.element.width()
    }

    fn height(&self) -> f32 {
        self.element.height()
    }
}

impl<E: Element> Layer<E> for Align<E> {
    fn inner(&self) -> &E {
        &self.element
    }
}

pub struct Space<E: Element> {
    top: Unsigned,
    bottom: Unsigned,
    left: Unsigned,
    right: Unsigned,
    element: E,
}

impl<E: Element> Space<E> {
    pub const fn trbl(element: E, top: Unsigned, right: Unsigned, bottom: Unsigned, left: Unsigned) -> Self {
        Self { top, bottom, left, right, element }
    }

    pub const fn top(&self) -> Unsigned {
        self.top
    }

    pub fn topf(&self) -> f32 {
        self.top.into()
    }

    pub const fn bottom(&self) -> Unsigned {
        self.bottom
    }

    pub fn bottomf(&self) -> f32 {
        self.bottom.into()
    }

    pub const fn left(&self) -> Unsigned {
        self.left
    }

    pub fn leftf(&self) -> f32 {
        self.left.into()
    }

    pub const fn right(&self) -> Unsigned {
        self.right
    }

    pub fn rightf(&self) -> f32 {
        self.right.into()
    }

    pub const fn all(element: E, space: Unsigned) -> Self {
        Self::trbl(element, space, space, space, space)
    }

    pub const fn horizontal(element: E, left: Unsigned, right: Unsigned) -> Self {
        Self::trbl(element, Unsigned::zero(), right, Unsigned::zero(), left)
    }

    pub const fn vertical(element: E, top: Unsigned, bottom: Unsigned) -> Self {
        Self::trbl(element, top, Unsigned::zero(), bottom, Unsigned::zero())
    }
}

impl<E: Element> Element for Space<E> {
    fn width(&self) -> f32 {
        self.element.width() + self.leftf() + self.rightf()
    }

    fn height(&self) -> f32 {
        self.element.height() + self.topf() + self.bottomf()
    }
}

impl<E: Element> Layer<E> for Space<E> {
    fn inner(&self) -> &E {
        &self.element
    }
}
