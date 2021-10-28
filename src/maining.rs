use macroquad::prelude::*;
// use macroquad::ui::widgets::Button;

use macroquad::ui::{
  hash, root_ui,
  widgets::{self, Button, Group},
  Drag, Ui,
};

use ::rand::{seq::SliceRandom, thread_rng};
use std::mem;
use strum::IntoEnumIterator;

use crate::card::*;
use crate::global::*;

// use crate::piece;

mod board;
mod card;
mod global;
mod piece;

#[derive(Debug, PartialEq)]
struct Vecf {
  i: f32,
  j: f32,
}

// #[derive(Debug, PartialEq, Clone)]
// struct piece::Coord {
//   i: usize,
//   j: usize,
// }

#[macroquad::main("UI showcase")]
async fn main() {
  let mut board = board::get_board();
  let mut card_vec: Vec<Card> = Card::iter().collect::<Vec<_>>();
  card_vec.shuffle(&mut thread_rng());
  let (mut player_one_card_vec, mut player_two_card_vec, mut middle_card) = (
    vec![&card_vec[0], &card_vec[1]],
    vec![&card_vec[2], &card_vec[3]],
    &card_vec[4],
  );

  println!("curr: {:#?}", player_one_card_vec);

  let mut curr_player = piece::Colour::Red;
  let mut opponent_player = piece::Colour::Blue;
  let mut still_playing = true;

  // let board_ref = ["bbBbb", ".....", ".....", ".....", "rrRrr"];

  let blue_pawn_image = load_texture("assets/blue_pawn.png").await.unwrap();
  let blue_pawn_select_image = load_texture("assets/blue_pawn_select.png").await.unwrap();
  let blue_pawn_dead_image = load_texture("assets/blue_pawn_dead.png").await.unwrap();

  let blue_king_image = load_texture("assets/blue_king.png").await.unwrap();
  let blue_king_select_image = load_texture("assets/blue_king_select.png").await.unwrap();
  let blue_king_dead_image = load_texture("assets/blue_king_dead.png").await.unwrap();

  let red_pawn_image = load_texture("assets/red_pawn.png").await.unwrap();
  let red_pawn_select_image = load_texture("assets/red_pawn_select.png").await.unwrap();
  let red_pawn_dead_image = load_texture("assets/red_pawn_dead.png").await.unwrap();

  let red_king_image = load_texture("assets/red_king.png").await.unwrap();
  let red_king_select_image = load_texture("assets/red_king_select.png").await.unwrap();
  let red_king_dead_image = load_texture("assets/red_king_dead.png").await.unwrap();

  let empty_image = load_texture("assets/empty.png").await.unwrap();
  let empty_dead_image = load_texture("assets/empty_dead.png").await.unwrap();

  let mut curr_select_card = 0;

  // let mut can_move_pos_vec: Vec<piece::Coord> = vec![];
  let mut curr_player_move_vec: Vec<(piece::Coord, Vec<(&Card, Vec<piece::Coord>)>)> = vec![];

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

  // println!("{}", board);

  // while still_playing {
  //   let mut curr_player_move_vec: Vec<(piece::Coord, Vec<(&Card, Vec<piece::Coord>)>)> = vec![];
  //   for piece_line in board.board.iter() {
  //     for piece in piece_line.iter() {
  //       if piece.colour == curr_player {
  //         let mut move_vec: Vec<(&Card, Vec<piece::Coord>)> = vec![];
  //         for card in player_one_card_vec.iter() {
  //           move_vec.push((card, piece.get_move_vec(&board, card.value())))
  //         }
  //         curr_player_move_vec.push((piece.coord.clone(), move_vec))
  //       }
  //     }
  //   }
  //   for (curr_player_piece_coord, piece_move) in curr_player_move_vec.iter() {
  //     for (card, coord_vec) in piece_move.iter() {
  //       for i in coord_vec.iter() {
  //         println!("{:#?}: {} => {}", card, curr_player_piece_coord, i)
  //       }
  //     }
  //   }
  //   match board.valid_move(curr_player_move_vec) {
  //     Ok((start_coord, end_coord, card)) => {
  //       board.move_piece(start_coord, end_coord);
  //       let ind = player_one_card_vec
  //         .iter()
  //         .position(|&c| c == &card)
  //         .unwrap();
  //       mem::swap(&mut player_one_card_vec[ind], &mut middle_card);
  //       mem::swap(&mut player_one_card_vec, &mut player_two_card_vec);
  //     }
  //     Err(e) => {
  //       println!("error: {}", e);
  //       continue;
  //     }
  //   }
  //   if board.way_of_stone(curr_player, opponent_player) || board.way_of_stream(curr_player) {
  //     still_playing = false;
  //   }
  //   mem::swap(&mut curr_player, &mut opponent_player);
  //   println!("{}", board);
  // }

  while still_playing {

    clear_background(BLUE);

    let can_die_vec = board.clone().board.iter()
      .map(|piece_line| {
        piece_line.iter()
          .map(|piece| board.contains_move(piece.coord.i, piece.coord.j, curr_player_move_vec.clone()))
          .collect::<Vec<bool>>()
      })
      .collect::<Vec<Vec<bool>>>();

    for (ind, piece_line) in (0..).zip(board.clone().board.iter()) {
      for (jnd, piece) in (0..).zip(piece_line.iter()) {
        // let ind = piece.coord.i;
        // let jnd = piece.coord.j;
        // let piece = &board.board[ind][jnd];
        let pos = &button_pos_vec[ind][jnd];

        if widgets::Button::new(
          // get_piece_image(piece_char, selected_pos.clone(), ind, jnd).await
          // if selected_pos == (piece::Coord {i: ind, j: jnd}) {
          match (piece.name, piece.colour) {
            (piece::Name::Pawn, piece::Colour::Blue) => [
              blue_pawn_image,
              blue_pawn_select_image,
              blue_pawn_dead_image,
            ],

            (piece::Name::Master, piece::Colour::Blue) => [
              blue_king_image,
              blue_king_select_image,
              blue_king_dead_image,
            ],

            (piece::Name::Pawn, piece::Colour::Red) => {
              [red_pawn_image, red_pawn_select_image, red_pawn_dead_image]
            }

            (piece::Name::Master, piece::Colour::Red) => {
              [red_king_image, red_king_select_image, red_king_dead_image]
            }

            _ => [empty_image, empty_image, empty_dead_image],
          }[if board.contains_move(piece.coord.i, piece.coord.j, curr_player_move_vec.clone()) {
            2
          }
          // if curr_player_move_vec.contains(&piece::Coord {i: ind, j: jnd})
          else if selected_pos == (piece::Coord { i: piece.coord.i, j: piece.coord.j }) {
            1
          } else {
            0
          }], // get_piece_image(piece_char, selected_pos, ind, jnd)
        )
        .size(vec2(50., 50.))
        .position(vec2(pos.j, pos.i))
        .ui(&mut *root_ui()) {
          println!("pos: {:?}", pos);
          println!("piece: {:?}", piece);

          curr_player_move_vec = vec![];

          selected_pos = piece::Coord {i: piece.coord.i, j: piece.coord.j};

      // let mut curr_player_move_vec: Vec<(piece::Coord, Vec<(&Card, Vec<piece::Coord>)>)> = vec![];
          for piece_line in board.board.iter() {
            for piece in piece_line.iter() {
              if piece.colour == curr_player && piece.coord == selected_pos {
                let mut move_vec: Vec<(&Card, Vec<piece::Coord>)> = vec![];

                let card = player_one_card_vec[curr_select_card];
                move_vec.push((card, piece.get_move_vec(&board, card.value())));
                curr_player_move_vec.push((piece.coord.clone(), move_vec))
              }
            }
          }

          println!("all moves: {:?}", curr_player_move_vec);
        }
      }
    }

    let old_select_card = curr_select_card;

    if root_ui().button(vec2(100., 400.), format!("{:?}", player_one_card_vec[0])) {
      curr_select_card = 0;
    }

    if root_ui().button(vec2(200., 400.), format!("{:?}", player_one_card_vec[1])) {
      curr_select_card = 1;
    }

    if curr_select_card != old_select_card {

      println!("new card bruh");
      board.board = board.board.into_iter().map(|piece_line| piece_line.into_iter().rev().collect::<Vec<piece::Piece>>()).rev().collect::<Vec<_>>();
      // selected_pos = piece::Coord { i: ind, j: jnd };
      mem::swap(&mut curr_player, &mut opponent_player);
      curr_player_move_vec = vec![];

      // let mut curr_player_move_vec: Vec<(piece::Coord, Vec<(&Card, Vec<piece::Coord>)>)> = vec![];
      for piece_line in board.board.iter() {
        for piece in piece_line.iter() {
          if piece.colour == curr_player && piece.coord == selected_pos {
            let mut move_vec: Vec<(&Card, Vec<piece::Coord>)> = vec![];

            let card = player_one_card_vec[curr_select_card];
            move_vec.push((card, piece.get_move_vec(&board, card.value())));
            curr_player_move_vec.push((piece.coord.clone(), move_vec))
          }
        }
      }
    }


    widgets::Label::new(format!("Selected: {:?}", player_one_card_vec[curr_select_card])).position(vec2(100., 450.)).ui(&mut *root_ui());

    widgets::Label::new(format!("Middle Card: {:?}", middle_card)).position(vec2(400., 200.)).ui(&mut *root_ui());
    widgets::Label::new(format!("{:?}", player_two_card_vec[0])).position(vec2(100., 25.)).ui(&mut *root_ui());
    widgets::Label::new(format!("{:?}", player_two_card_vec[1])).position(vec2(200., 25.)).ui(&mut *root_ui());

    if is_key_down(KeyCode::Escape) {
      selected_pos = piece::Coord { i: 69, j: 69 };
      curr_player_move_vec = vec![];
    }

    // f ind % 5 == 0 {

    // if !still_playing {
    //   break;
    // }

    next_frame().await;
  }

  // println!("game done");
}
