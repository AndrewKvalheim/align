use crate::board::Board;
use crate::color::Color;

pub struct Walk<'a> {
    board: &'a Board,
    color: Color,
    position: (usize, usize),
    step: (isize, isize),
}

impl<'a> Walk<'a> {
    pub fn new(
        board: &'a Board,
        color: Color,
        position: (usize, usize),
        step: (isize, isize),
    ) -> Self {
        Self {
            board,
            color,
            position,
            step,
        }
    }
}

impl<'a> Iterator for Walk<'a> {
    type Item = Color;

    fn next(&mut self) -> Option<Self::Item> {
        self.position = (
            (self.position.0 as isize + self.step.0) as usize,
            (self.position.1 as isize + self.step.1) as usize,
        );

        self.board
            .get(self.position)
            .and_then(|slot| slot.filter(|&c| c == self.color))
    }
}
