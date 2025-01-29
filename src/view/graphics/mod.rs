use macroquad::prelude::*;
use macroquad::ui::{
  root_ui,
  widgets::{Button, Texture},
};

use regex::Regex;
use std::collections::HashMap;

use crate::controller::game;
use crate::model::board;
use crate::model::card;
use crate::model::piece;
use crate::view::graphics;

pub async fn get_image_hash(font: Font) -> HashMap<&'static str, Texture2D> {
  let image_name_regex = Regex::new(r"assets\\(.+?)\.png").unwrap();
  // let image_vec = fs::read_dir("assets").unwrap();
  let image_vec = vec![
    r"assets\bear.png",
    r"assets\blue_king.png",
    r"assets\blue_king_dead.png",
    r"assets\blue_king_select.png",
    r"assets\blue_pawn.png",
    r"assets\blue_pawn_dead.png",
    r"assets\blue_pawn_select.png",
    r"assets\boar.png",
    r"assets\cobra.png",
    r"assets\crab.png",
    r"assets\crane.png",
    r"assets\dog.png",
    r"assets\dragon.png",
    r"assets\eel.png",
    r"assets\elephant.png",
    r"assets\empty.png",
    r"assets\empty_dead.png",
    r"assets\fox.png",
    r"assets\frog.png",
    r"assets\giraffe.png",
    r"assets\goose.png",
    r"assets\horse.png",
    r"assets\iguana.png",
    r"assets\kirin.png",
    r"assets\mantis.png",
    r"assets\monkey.png",
    r"assets\mouse.png",
    r"assets\otter.png",
    r"assets\ox.png",
    r"assets\panda.png",
    r"assets\phoenix.png",
    r"assets\rabbit.png",
    r"assets\rat.png",
    r"assets\red_king.png",
    r"assets\red_king_dead.png",
    r"assets\red_king_select.png",
    r"assets\red_pawn.png",
    r"assets\red_pawn_dead.png",
    r"assets\red_pawn_select.png",
    r"assets\rooster.png",
    r"assets\sable.png",
    r"assets\sea_snake.png",
    r"assets\tanuki.png",
    r"assets\tiger.png",
    r"assets\turtle.png",
    r"assets\viper.png",
  ];

  let mut image_hash: HashMap<&'static str, Texture2D> = HashMap::new();

  for image_string in image_vec {
    clear_background(BLACK);
    draw_text_ex(
      &format!(
        "Loading resources {}",
        ".".repeat(((get_time() * 2.0) as usize) % 4)
      ),
      screen_width() / 2.0 - 160.0,
      screen_height() / 2.0,
      TextParams {
        font_size: 25,
        font: Some(&font),
        ..Default::default()
      },
    );
    image_hash.insert(
      image_name_regex
        .captures(image_string)
        .unwrap()
        .get(1)
        .map_or("", |m| m.as_str()),
      load_texture(image_string).await.unwrap(),
    );
    next_frame().await;
  }

  image_hash
}

pub fn draw_card_info(
  curr_player_card_vec: Vec<&card::Card>,
  opponent_player_card_vec: Vec<&card::Card>,
  middle_card: &card::Card,
  curr_select_card: usize,
  image_hash: HashMap<&'static str, Texture2D>,
  font: Font,
) {

  draw_text_ex(
    &format!("Selected: {:?}", curr_player_card_vec[curr_select_card]),
    100.,
    525.,
    TextParams {
      font_size: 25,
      font: Some(&font),
      ..Default::default()
    },
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
  }]].clone()
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
  }].clone()
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
