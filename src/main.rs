mod controller;
mod global;
mod model;
mod view;

#[macroquad::main("UI showcase")]
async fn main() {
  controller::game::start().await
}
