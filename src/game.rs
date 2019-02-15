use crate::board::{Board, Effect::*};
use crate::color::Color;
use read_input::prelude::*;

pub struct Game {
    board: Board,
    turn: Color,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            turn: Color::default(),
        }
    }

    pub fn interact(&mut self) {
        loop {
            match self.board.insert(self.prompt(), self.turn) {
                Err(i) => println!("Column {} is full.", 1 + i),
                Ok(None) => self.turn.invert(),
                Ok(Some(CompleteSegment)) => {
                    println!("{}\n{} wins.", self.board, self.turn);
                    break;
                }
                Ok(Some(FullBoard)) => {
                    println!("{}\nTie.", self.board);
                    break;
                }
            }
        }
    }

    fn prompt(&self) -> usize {
        println!("１ ２ ３ ４ ５ ６ ７\n{}", self.board);

        input()
            .inside(1..=7)
            .err("Enter a column number (1–7).")
            .repeat_msg(format!("{} into: ", self.turn))
            .get()
            - 1
    }
}
