use crate::math::*;

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


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub red: PercentUnsigned,
    pub green: PercentUnsigned,
    pub blue: PercentUnsigned,
    pub alpha: PercentUnsigned,
}

#[derive(Clone)]
pub struct Shader; // TODO
