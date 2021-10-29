mod model;
mod view;
mod controller;
mod global;

#[macroquad::main("UI showcase")]
async fn main() {
  controller::game::start().await
}
