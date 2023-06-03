use crate::base::*;

pub trait Alive: Element {
    fn init(&mut self) {}

    fn refresh(&mut self) {}
}

pub trait Update {
    type Element: Alive;

    fn apply(self, alive: &mut Self::Element);
}
