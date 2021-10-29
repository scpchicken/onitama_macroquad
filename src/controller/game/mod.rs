use macroquad::prelude::*;

use macroquad::ui::{
  root_ui,
  widgets::{Button, Label},
};

use ::rand::{seq::SliceRandom, thread_rng};
use std::mem;
use strum::IntoEnumIterator;

use crate::global::*;
use crate::model::board;
use crate::model::card;
use crate::model::piece;
use crate::view::graphics;

#[derive(Debug, PartialEq)]
struct Vecf {
  i: f32,
  j: f32,
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

  let mut curr_player = piece::Colour::Red;
  let mut opponent_player = piece::Colour::Blue;

  let image_hash = graphics::get_image_hash().await;

  let mut curr_select_card = 0;

  let mut game_over = false;

  let mut curr_player_move_vec: Vec<Vec<piece::Coord>> = vec![];

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
    clear_background(BLUE);
    if game_over {
      draw_rectangle(250., 250., 100., 200., GRAY);
      Label::new(format!("{:?} wins :O", opponent_player))
        .position(vec2(250., 250.))
        .ui(&mut *root_ui());
    } else {
      // let board::Board(board) = board;

      let can_die_vec = board
        .clone()
        .0
        .iter()
        .map(|piece_line| {
          piece_line
            .iter()
            .map(|piece| {
              board.contains_move(piece.coord.i, piece.coord.j, curr_player_move_vec.clone())
            })
            .collect::<Vec<bool>>()
        })
        .collect::<Vec<Vec<bool>>>();

      let board_vec = board.clone().0;
      let board_vec_rev = board
        .clone()
        .0
        .into_iter()
        .map(|piece_line| piece_line.into_iter().rev().collect::<Vec<piece::Piece>>())
        .rev()
        .collect::<Vec<_>>();

      for (ind, piece_line) in (0..).zip(if curr_player == piece::Colour::Red {
        board_vec.iter()
      } else {
        board_vec_rev.iter()
      }) {
        for (jnd, piece) in (0..).zip(piece_line.iter()) {
          let pos = &button_pos_vec[ind][jnd];
          if Button::new(
            image_hash[&match (piece.name, piece.colour) {
              (piece::Name::Pawn, piece::Colour::Blue) => {
                ["blue_pawn", "blue_pawn_select", "blue_pawn_dead"]
              }

              (piece::Name::Master, piece::Colour::Blue) => {
                ["blue_king", "blue_king_select", "blue_king_dead"]
              }

              (piece::Name::Pawn, piece::Colour::Red) => {
                ["red_pawn", "red_pawn_select", "red_pawn_dead"]
              }

              (piece::Name::Master, piece::Colour::Red) => {
                ["red_king", "red_king_select", "red_king_dead"]
              }

              _ => ["empty", "empty", "empty_dead"],
            }[if board.contains_move(
              piece.coord.i,
              piece.coord.j,
              curr_player_move_vec.clone(),
            ) {
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
            }]],
          )
          .size(vec2(50., 50.))
          .position(vec2(pos.j, pos.i))
          .ui(&mut *root_ui())
          {
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

              curr_player_move_vec = vec![];

              for piece_line in board.0.iter() {
                for piece in piece_line.iter() {
                  if piece.colour == curr_player && piece.coord == selected_pos {
                    let card = curr_player_card_vec[curr_select_card];
                    curr_player_move_vec.push(piece.get_move_vec(&board, card.value()))
                  }
                }
              }
            }
          }
        }
      }

      let old_select_card = curr_select_card;

      if root_ui().button(vec2(100., 400.), format!("{:?}", curr_player_card_vec[0])) {
        curr_select_card = 0;
      }

      if root_ui().button(vec2(200., 400.), format!("{:?}", curr_player_card_vec[1])) {
        curr_select_card = 1;
      }

      if curr_select_card != old_select_card {
        curr_player_move_vec = vec![];

        for piece_line in board.0.iter() {
          for piece in piece_line.iter() {
            if piece.colour == curr_player && piece.coord == selected_pos {
              let card = curr_player_card_vec[curr_select_card];

              curr_player_move_vec.push(piece.get_move_vec(&board, card.value()))
            }
          }
        }
      }
      draw_rectangle(100., 450., 200., 30., GRAY);
      Label::new(format!(
        "Selected: {:?}",
        curr_player_card_vec[curr_select_card]
      ))
      .position(vec2(100., 450.))
      .ui(&mut *root_ui());

      draw_rectangle(400., 200., 200., 30., GRAY);
      Label::new(format!("Middle card: {:?}", middle_card))
        .position(vec2(400., 200.))
        .ui(&mut *root_ui());

      draw_rectangle(
        100.,
        25.,
        format!("{:?}", opponent_player_card_vec[0]).len() as f32 * 10.,
        30.,
        GRAY,
      );
      Label::new(format!("{:?}", opponent_player_card_vec[0]))
        .position(vec2(100., 25.))
        .ui(&mut *root_ui());

      draw_rectangle(
        200.,
        25.,
        format!("{:?}", opponent_player_card_vec[1]).len() as f32 * 10.,
        30.,
        GRAY,
      );
      Label::new(format!("{:?}", opponent_player_card_vec[1]))
        .position(vec2(200., 25.))
        .ui(&mut *root_ui());

      if is_key_down(KeyCode::Escape) {
        selected_pos = piece::Coord { i: 69, j: 69 };
        curr_player_move_vec = vec![];
      }
    }

    next_frame().await;
  }
}
