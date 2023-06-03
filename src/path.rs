use crate::base::*;
use crate::math::*;

pub struct Stroke {
    pub width: Positive,
    pub shader: Shader,
    pub cap: Cap,
    pub join: Join,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cap {
    Flat,
    Round,
    Square,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Join {
    Miter(Unsigned),
    Round,
    Bevel,
}

pub struct Border<E: Element> {
    element: E,
    stroke: Stroke,
}

impl<E: Element> Border<E> {
    pub fn new(element: E, stroke: Stroke) -> Self {
        Self { element, stroke }
    }

    pub fn stroke(&self) -> &Stroke {
        &self.stroke
    }
}

impl<E: Element> Element for Border<E> {
    fn width(&self) -> f32 {
        let stroke_width: f32 = self.stroke.width.into();
        self.element.width() + stroke_width * 2.0
    }

    fn height(&self) -> f32 {
        let stroke_width: f32 = self.stroke.width.into();
        self.element.height() + stroke_width * 2.0
    }
}

impl<E: Element> Layer<E> for Border<E> {
    fn inner(&self) -> &E {
        &self.element
    }
}
