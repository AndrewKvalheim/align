use crate::color::Color::{self, *};
use crate::walk::Walk;
use itertools::Itertools;
use Effect::*;

const ANGLES: [(isize, isize); 4] = [(0, 1), (1, 1), (1, 0), (1, -1)];
const HEIGHT: usize = 6;
const STEPS: usize = 3;
const WIDTH: usize = 7;

#[derive(Debug, PartialEq)]
pub enum Effect {
    CompleteSegment,
    FullBoard,
}

type Slot = Option<Color>;

pub struct Board {
    columns: [[Slot; HEIGHT]; WIDTH],
    remaining: usize,
}

impl Board {
    pub fn new() -> Self {
        Self {
            columns: [[None; HEIGHT]; WIDTH],
            remaining: HEIGHT * WIDTH,
        }
    }

    pub fn get(&self, (x, y): (usize, usize)) -> Option<&Slot> {
        if x < WIDTH && y < HEIGHT {
            Some(&self.columns[x][y])
        } else {
            None
        }
    }

    pub fn insert(&mut self, x: usize, color: Color) -> Result<Option<Effect>, usize> {
        let y = self.columns[x].iter().position(|s| s.is_none()).ok_or(x)?;

        self.columns[x][y].replace(color);
        self.remaining -= 1;

        let walk = |direction| Walk::new(&self, color, (x, y), direction);
        let complete_segment = ANGLES
            .iter()
            .any(|&(dx, dy)| walk((dx, dy)).chain(walk((-dx, -dy))).take(STEPS).count() == STEPS);

        Ok(if complete_segment {
            Some(CompleteSegment)
        } else if self.remaining == 0 {
            Some(FullBoard)
        } else {
            None
        })
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        let dot = |slot| match slot {
            None => '⚫',
            Some(Blue) => '🔵',
            Some(Red) => '🔴',
        };
        let display = (0..HEIGHT)
            .rev()
            .map(|y| (0..WIDTH).map(|x| dot(self.columns[x][y])).join(" "))
            .join("\n");

        write!(formatter, "{}", display)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn horizontal_segment_at_left_edge() {
        let mut board = Board::new();

        assert_eq!(board.insert(0, Red), Ok(None));
        assert_eq!(board.insert(3, Red), Ok(None));
        assert_eq!(board.insert(1, Red), Ok(None));
        assert_eq!(board.insert(2, Red), Ok(Some(CompleteSegment)));
    }

    #[test]
    fn horizontal_segment_at_right_edge() {
        let mut board = Board::new();

        assert_eq!(board.insert(5, Red), Ok(None));
        assert_eq!(board.insert(4, Red), Ok(None));
        assert_eq!(board.insert(3, Red), Ok(None));
        assert_eq!(board.insert(6, Red), Ok(Some(CompleteSegment)));
    }

    #[test]
    fn vertical_segment_at_bottom() {
        let mut board = Board::new();

        assert_eq!(board.insert(0, Red), Ok(None));
        assert_eq!(board.insert(0, Red), Ok(None));
        assert_eq!(board.insert(0, Red), Ok(None));
        assert_eq!(board.insert(0, Red), Ok(Some(CompleteSegment)));
    }

    #[test]
    fn vertical_segment_at_top() {
        let mut board = Board::new();

        assert_eq!(board.insert(0, Blue), Ok(None));
        assert_eq!(board.insert(0, Blue), Ok(None));
        assert_eq!(board.insert(0, Red), Ok(None));
        assert_eq!(board.insert(0, Red), Ok(None));
        assert_eq!(board.insert(0, Red), Ok(None));
        assert_eq!(board.insert(0, Red), Ok(Some(CompleteSegment)));
    }

    #[test]
    fn ascending_diagonal_segment() {
        let mut board = Board::new();

        assert_eq!(board.insert(3, Blue), Ok(None));
        assert_eq!(board.insert(4, Blue), Ok(None));
        assert_eq!(board.insert(4, Blue), Ok(None));
        assert_eq!(board.insert(5, Blue), Ok(None));
        assert_eq!(board.insert(5, Blue), Ok(None));
        assert_eq!(board.insert(5, Blue), Ok(None));
        assert_eq!(board.insert(2, Red), Ok(None));
        assert_eq!(board.insert(3, Red), Ok(None));
        assert_eq!(board.insert(5, Red), Ok(None));
        assert_eq!(board.insert(4, Red), Ok(Some(CompleteSegment)));
    }

    #[test]
    fn descending_diagonal_segment() {
        let mut board = Board::new();

        assert_eq!(board.insert(5, Blue), Ok(None));
        assert_eq!(board.insert(4, Blue), Ok(None));
        assert_eq!(board.insert(4, Blue), Ok(None));
        assert_eq!(board.insert(3, Blue), Ok(None));
        assert_eq!(board.insert(3, Blue), Ok(None));
        assert_eq!(board.insert(3, Blue), Ok(None));

        assert_eq!(board.insert(4, Red), Ok(None));
        assert_eq!(board.insert(5, Red), Ok(None));
        assert_eq!(board.insert(6, Red), Ok(None));
        assert_eq!(board.insert(3, Red), Ok(Some(CompleteSegment)));
    }

    #[test]
    fn unnecessarily_long_segment() {
        let mut board = Board::new();

        assert_eq!(board.insert(1, Red), Ok(None));
        assert_eq!(board.insert(2, Red), Ok(None));
        assert_eq!(board.insert(4, Red), Ok(None));
        assert_eq!(board.insert(5, Red), Ok(None));
        assert_eq!(board.insert(3, Red), Ok(Some(CompleteSegment)));
    }

    #[test]
    fn full_column() {
        let mut board = Board::new();

        assert_eq!(board.insert(0, Red), Ok(None));
        assert_eq!(board.insert(0, Blue), Ok(None));
        assert_eq!(board.insert(0, Red), Ok(None));
        assert_eq!(board.insert(0, Blue), Ok(None));
        assert_eq!(board.insert(0, Red), Ok(None));
        assert_eq!(board.insert(0, Blue), Ok(None));
        assert_eq!(board.insert(0, Red), Err(0));
    }

    #[test]
    fn full_board() {
        let mut board = Board::new();

        for &x in [0, 1, 2, 4, 5, 6].iter() {
            assert_eq!(board.insert(x, Red), Ok(None));
            assert_eq!(board.insert(x, Blue), Ok(None));
            assert_eq!(board.insert(x, Red), Ok(None));
            assert_eq!(board.insert(x, Blue), Ok(None));
            assert_eq!(board.insert(x, Red), Ok(None));
            assert_eq!(board.insert(x, Blue), Ok(None));
        }

        assert_eq!(board.insert(3, Blue), Ok(None));
        assert_eq!(board.insert(3, Red), Ok(None));
        assert_eq!(board.insert(3, Blue), Ok(None));
        assert_eq!(board.insert(3, Red), Ok(None));
        assert_eq!(board.insert(3, Blue), Ok(None));
        assert_eq!(board.insert(3, Red), Ok(Some(FullBoard)));
    }
}
