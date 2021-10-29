use macroquad::prelude::*;
use macroquad::ui::{root_ui, widgets::Label};

use regex::Regex;
use std::collections::HashMap;
use std::fs;

use crate::model::card;

pub async fn get_image_hash() -> HashMap<&'static str, Texture2D> {
  let image_name_regex = Regex::new(r"assets\\(.+?)\.png").unwrap();
  let image_vec = fs::read_dir("assets").unwrap();

  let mut image_hash: HashMap<&'static str, Texture2D> = HashMap::new();

  for image in image_vec {
    let image_string = Box::leak(format!("{}", image.unwrap().path().display()).into_boxed_str());
    image_hash.insert(
      image_name_regex
        .captures(image_string)
        .unwrap()
        .get(1)
        .map_or("", |m| m.as_str()),
      load_texture(image_string).await.unwrap(),
    );
  }

  image_hash
}

pub fn draw_rect_label(
  rect_x: f32,
  rect_y: f32,
  rect_w: f32,
  rect_h: f32,
  rect_colour: Color,
  label_string: String,
  label_pos: Vec2,
) {
  draw_rectangle(rect_x, rect_y, rect_w, rect_h, rect_colour);
  Label::new(label_string)
    .position(label_pos)
    .ui(&mut *root_ui());
}

pub fn draw_card_info(
  curr_player_card_vec: Vec<&card::Card>,
  opponent_player_card_vec: Vec<&card::Card>,
  middle_card: &card::Card,
  curr_select_card: usize,
) {
  draw_rect_label(
    100.,
    450.,
    200.,
    30.,
    GRAY,
    format!("Selected: {:?}", curr_player_card_vec[curr_select_card]),
    vec2(100., 450.),
  );
  draw_rect_label(
    400.,
    200.,
    200.,
    30.,
    GRAY,
    format!("Middle card: {:?}", middle_card),
    vec2(400., 200.),
  );

  draw_rect_label(
    100.,
    25.,
    format!("{:?}", opponent_player_card_vec[0]).len() as f32 * 10.,
    30.,
    GRAY,
    format!("{:?}", opponent_player_card_vec[0]),
    vec2(100., 25.),
  );
  draw_rect_label(
    200.,
    25.,
    format!("{:?}", opponent_player_card_vec[1]).len() as f32 * 10.,
    30.,
    GRAY,
    format!("{:?}", opponent_player_card_vec[1]),
    vec2(200., 25.),
  );
}
