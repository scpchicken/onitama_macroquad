use macroquad::prelude::*;
use macroquad::ui::{
  root_ui,
  widgets::{Button, Label, Texture},
};

use regex::Regex;
use std::collections::HashMap;
use std::fs;

use crate::controller::game;
use crate::model::board;
use crate::model::card;
use crate::model::piece;
use crate::view::graphics;

pub async fn get_image_hash() -> HashMap<&'static str, Texture2D> {
  let image_name_regex = Regex::new(r"assets\\(.+?)\.png").unwrap();
  let image_vec = fs::read_dir("assets").unwrap();

  let mut image_hash: HashMap<&'static str, Texture2D> = HashMap::new();

  for image in image_vec {
    let image_string = Box::leak(
      format!("{}", image.unwrap().path().display())
        .to_lowercase()
        .into_boxed_str(),
    );
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

pub fn draw_rect_text(
  rect_x: f32,
  rect_y: f32,
  rect_w: f32,
  rect_h: f32,
  rect_colour: Color,
  label_string: String,
  font: Font,
) {
  draw_rectangle(rect_x, rect_y, rect_w, rect_h, rect_colour);
  draw_text_ex(
    &label_string,
    rect_x,
    rect_y,
    TextParams {
      font_size: 25,
      font,
      ..Default::default()
    },
  );
}

pub fn draw_card_info(
  curr_player_card_vec: Vec<&card::Card>,
  opponent_player_card_vec: Vec<&card::Card>,
  middle_card: &card::Card,
  curr_select_card: usize,
  image_hash: HashMap<&'static str, Texture2D>,
  font: Font,
) {
  draw_rect_text(
    100.,
    525.,
    200.,
    30.,
    GRAY,
    format!("Selected: {:?}", curr_player_card_vec[curr_select_card]),
    font,
  );

  Texture::new(get_card_image(middle_card, image_hash.clone()))
    .size(100., 58.)
    .position(vec2(400., 200.))
    .ui(&mut *root_ui());
  Texture::new(get_card_image(
    opponent_player_card_vec[0],
    image_hash.clone(),
  ))
  .size(100., 58.)
  .position(vec2(100., 25.))
  .ui(&mut *root_ui());

  Texture::new(get_card_image(
    opponent_player_card_vec[1],
    image_hash.clone(),
  ))
  .size(100., 58.)
  .position(vec2(225., 25.))
  .ui(&mut *root_ui());
}

pub fn get_image(
  selected_pos: piece::Coord,
  piece: &piece::Piece,
  curr_player_move_vec: Vec<piece::Coord>,
  board: board::Board,
  image_hash: HashMap<&'static str, Texture2D>,
) -> Texture2D {
  image_hash[&match (piece.name, piece.colour) {
    (piece::Name::Pawn, piece::Colour::Blue) => ["blue_pawn", "blue_pawn_select", "blue_pawn_dead"],

    (piece::Name::Master, piece::Colour::Blue) => {
      ["blue_king", "blue_king_select", "blue_king_dead"]
    }

    (piece::Name::Pawn, piece::Colour::Red) => ["red_pawn", "red_pawn_select", "red_pawn_dead"],

    (piece::Name::Master, piece::Colour::Red) => ["red_king", "red_king_select", "red_king_dead"],

    _ => ["empty", "empty", "empty_dead"],
  }[if board.contains_move(piece.coord.i, piece.coord.j, curr_player_move_vec.clone()) {
    2
  } else if selected_pos
    == (piece::Coord {
      i: piece.coord.i,
      j: piece.coord.j,
    })
  {
    1
  } else {
    0
  }]]
}

pub fn get_card_image(
  card: &card::Card,
  image_hash: HashMap<&'static str, Texture2D>,
) -> Texture2D {
  let card_string = format!("{:?}", card).to_lowercase();

  image_hash[if card == &card::Card::SeaSnake {
    "sea_snake"
  } else {
    card_string.as_str()
  }]
}

pub fn piece_button<'a>(
  selected_pos: piece::Coord,
  piece: &piece::Piece,
  curr_player_move_vec: Vec<piece::Coord>,
  board: board::Board,
  image_hash: HashMap<&'static str, Texture2D>,
  button_pos_vec: Vec<Vec<game::Vecf>>,
  ind: usize,
  jnd: usize,
) -> bool {
  let pos = &button_pos_vec[ind][jnd];
  Button::new(graphics::get_image(
    selected_pos,
    piece,
    curr_player_move_vec.clone(),
    board.clone(),
    image_hash.clone(),
  ))
  .size(vec2(50., 50.))
  .position(vec2(pos.j, pos.i))
  .ui(&mut *root_ui())
}
