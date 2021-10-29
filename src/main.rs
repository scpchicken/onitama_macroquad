mod board;
mod card;
mod game;
mod global;
mod graphics;
mod piece;

#[macroquad::main("UI showcase")]
async fn main() {
  game::start().await
}
