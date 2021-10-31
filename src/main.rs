use macroquad::prelude::*;
use macroquad::ui::root_ui;
use macroquad::ui::widgets::Button;
use macroquad::ui::widgets::Texture;

use ::rand::{seq::SliceRandom, thread_rng};
use std::mem;
use strum::IntoEnumIterator;

use crate::constant::*;
use crate::model::board;
use crate::model::card;
use crate::model::piece;
use crate::view::graphics;

mod constant;
mod controller;
mod model;
mod view;

#[macroquad::main("UI showcase")]
async fn main() {
  // controller::game::start().await

  let image_hash = graphics::get_image_hash().await;

  // let pawn = load_texture(r"assets\blue_pawn.png").await.unwrap();

  loop {
    clear_background(BLUE);
    draw_rectangle(0., 0., 50., 50., GREEN);

    Button::new(image_hash["blue_pawn"]).size(vec2(50., 50.)).ui(&mut root_ui());
    // TExtur

    next_frame().await;
  }
}
