mod game;
mod snake;
mod timer;

use game::{Game, Vec2};

fn main() -> crossterm::Result<()> {
    Game::new(Vec2::new(20, 20))?.init_loop()
}
