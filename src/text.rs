use std::borrow::Cow;
use crate::base::*;
use crate::math::*;

pub struct Text {
    value: Cow<'static, str>,
    style: Style,
}

#[derive(Clone)]
pub struct Style {
    pub shader: Shader,
    pub background: Shader,
    pub font: Cow<'static, str>,
    pub family: Cow<'static, str>,
    pub size: Positive,
    pub weight: Positive,
    pub underline: Option<Positive>,
    pub strikeout: Option<Positive>,
    pub overline: Option<Positive>,
    pub italic: bool,
    pub letter_spacing: Positive,
    pub word_spacing: Positive,
}

impl Text {
    pub fn new(text: impl Into<Cow<'static, str>>, style: Style) -> Self {
        Self {
            value: text.into(),
            style,
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn style(&self) -> &Style {
        &self.style
    }
}

impl Element for Text {
    fn width(&self) -> f32 {
        todo!()
    }

    fn height(&self) -> f32 {
        todo!()
    }
}

impl Widget for Text {}
