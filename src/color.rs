use Color::*;

#[derive(Clone, Copy, PartialEq)]
pub enum Color {
    Blue,
    Red,
}

impl Color {
    pub fn default() -> Self {
        Red
    }

    pub fn invert(&mut self) {
        *self = if *self == Blue { Red } else { Blue }
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        let name = match self {
            Blue => "Blue",
            Red => "Red",
        };

        write!(formatter, "{}", name)
    }
}
