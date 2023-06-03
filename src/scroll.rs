use crate::base::*;

pub struct Scroll<E: Element> {
    element: E,
}

impl<E: Element> Scroll<E> {
    pub fn new(element: E) -> Self {
        Self { element }
    }
}

impl<E: Element> Element for Scroll<E> {
    fn width(&self) -> f32 {
        self.element.width()
    }

    fn height(&self) -> f32 {
        self.element.height()
    }
}

impl<E: Element> Layer<E> for Scroll<E> {
    fn inner(&self) -> &E {
        &self.element
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Horizontal,
    Vertical,
}
