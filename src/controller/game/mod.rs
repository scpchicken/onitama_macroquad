use macroquad::prelude::*;
use macroquad::ui::root_ui;
use macroquad::ui::widgets::Button;

use ::rand::{seq::SliceRandom, thread_rng};
use std::mem;
use strum::IntoEnumIterator;

use crate::constant::*;
use crate::model::board;
use crate::model::card;
use crate::model::piece;
use crate::view::graphics;

#[derive(Debug, PartialEq, Clone)]
pub struct Vecf {
  pub i: f32,
  pub j: f32,
}

pub async fn start() {
  let mut board = board::get_board();

  let mut card_vec: Vec<card::Card> = card::Card::iter().collect::<Vec<_>>();
  card_vec.shuffle(&mut thread_rng());
  let (mut curr_player_card_vec, mut opponent_player_card_vec, mut middle_card) = (
    vec![&card_vec[0], &card_vec[1]],
    vec![&card_vec[2], &card_vec[3]],
    &card_vec[4],
  );

  let (mut curr_player, mut opponent_player) = if middle_card.colour() == piece::Colour::Blue {
    (piece::Colour::Blue, piece::Colour::Red)
  } else {
    (piece::Colour::Red, piece::Colour::Blue)
  };

  let image_hash = graphics::get_image_hash().await;

  // println!("{:#?}", image_hash);

  let mut curr_select_card = 0;

  let mut game_over = false;

  let mut curr_player_move_vec: Vec<piece::Coord> = vec![];

  let font = load_ttf_font(r"fonts\MinimalPixel v2.ttf")
    .await
    .unwrap();

  let button_pos_vec = (0..5)
    .map(|ind| {
      (0..5)
        .map(|jnd| Vecf {
          i: 100. + ind as f32 * (IMAGE_SIZE + IMAGE_DIST),
          j: 100. + jnd as f32 * (IMAGE_SIZE + IMAGE_DIST),
        })
        .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>();
  let mut selected_pos = piece::Coord { i: 69, j: 69 };

  loop {
    clear_background(BLANK);

    if game_over {
      graphics::draw_rect_text(
        250.,
        250.,
        100.,
        200.,
        GRAY,
        format!("{:?} wins :O", opponent_player),
        font,
      );
    } else {
      let can_die_vec = board.get_can_die_vec(curr_player_move_vec.clone());

      let board_vec = board.clone().0;
      let board_vec_rev = board.get_flipped();

      let blue_temple_button_pos =
        button_pos_vec[BLUE_TEMPLE_ARCH_POS.i][BLUE_TEMPLE_ARCH_POS.j].clone();
      let red_temple_button_pos =
        button_pos_vec[RED_TEMPLE_ARCH_POS.i][RED_TEMPLE_ARCH_POS.j].clone();

      for (ind, piece_line) in (0..).zip(if curr_player == piece::Colour::Red {
        // button_pos_vec[BLUE_TEMPLE_ARCH_POS.i][BLUE_TEMPLE_ARCH_POS.j].i;
        // let blue_temple_arch_pos = button_pos_vec[BLUE_TEMPLE_ARCH_POS.i][BLUE_TEMPLE_ARCH_POS.j];

        draw_rectangle(
          blue_temple_button_pos.j - 3.75,
          blue_temple_button_pos.i - 3.75,
          57.5,
          57.5,
          DARKBLUE,
        );
        draw_rectangle(
          red_temple_button_pos.j - 3.75,
          red_temple_button_pos.i - 3.75,
          57.5,
          57.5,
          MAROON,
        );

        board_vec.iter()
      } else {
        draw_rectangle(
          red_temple_button_pos.j - 3.75,
          red_temple_button_pos.i - 3.75,
          57.5,
          57.5,
          DARKBLUE,
        );
        draw_rectangle(
          blue_temple_button_pos.j - 3.75,
          blue_temple_button_pos.i - 3.75,
          57.5,
          57.5,
          MAROON,
        );

        board_vec_rev.iter()
      }) {
        for (jnd, piece) in (0..).zip(piece_line.iter()) {
          if graphics::piece_button(
            selected_pos,
            piece,
            curr_player_move_vec.clone(),
            board.clone(),
            image_hash.clone(),
            button_pos_vec.clone(),
            ind,
            jnd,
          ) {
            if can_die_vec[piece.coord.i][piece.coord.j] {
              board.move_piece(
                selected_pos,
                piece::Coord {
                  i: piece.coord.i,
                  j: piece.coord.j,
                },
              );

              selected_pos = piece::Coord { i: 69, j: 69 };
              curr_player_move_vec = vec![];

              game_over = board.way_of_stream(curr_player)
                || board.way_of_stone(curr_player, opponent_player);

              mem::swap(&mut curr_player, &mut opponent_player);
              mem::swap(
                &mut curr_player_card_vec[curr_select_card],
                &mut middle_card,
              );
              mem::swap(&mut curr_player_card_vec, &mut opponent_player_card_vec);
            } else {
              selected_pos = piece::Coord {
                i: piece.coord.i,
                j: piece.coord.j,
              };

              let card = curr_player_card_vec[curr_select_card];
              let selected_piece = board.get_piece(curr_player, selected_pos);

              match selected_piece {
                Some(piece) => curr_player_move_vec = piece.get_move_vec(&board, card.value()),

                None => curr_player_move_vec = vec![],
              }
            }
          }
        }
      }

      let old_select_card = curr_select_card;

      if Button::new(graphics::get_card_image(
        curr_player_card_vec[0],
        image_hash.clone(),
      ))
      .size(vec2(100., 58.))
      .position(vec2(100., 400.))
      .ui(&mut *root_ui())
      {
        curr_select_card = 0;
      } else if Button::new(graphics::get_card_image(
        curr_player_card_vec[1],
        image_hash.clone(),
      ))
      .size(vec2(100., 58.))
      .position(vec2(225., 400.))
      .ui(&mut *root_ui())
      {
        curr_select_card = 1;
      } else if is_key_pressed(KeyCode::S) {
        curr_select_card ^= 1;
      }

      if curr_select_card != old_select_card {
        let card = curr_player_card_vec[curr_select_card];

        let selected_piece = board.get_piece(curr_player, selected_pos);

        match selected_piece {
          Some(piece) => {
            curr_player_move_vec = piece.get_move_vec(&board, card.value());
          }

          None => {}
        }
      }

      graphics::draw_card_info(
        curr_player_card_vec.clone(),
        opponent_player_card_vec.clone(),
        middle_card,
        curr_select_card,
        image_hash.clone(),
        font,
      );

      if is_key_down(KeyCode::Escape) {
        selected_pos = piece::Coord { i: 69, j: 69 };
        curr_player_move_vec = vec![];
      }
    }

    next_frame().await;
  }
}
