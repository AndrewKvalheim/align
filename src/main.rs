extern crate itertools;
extern crate read_input;

mod board;
mod color;
mod game;
mod walk;

use game::Game;

fn main() {
    Game::new().interact()
}
