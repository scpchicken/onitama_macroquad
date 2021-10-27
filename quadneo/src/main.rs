use macroquad::prelude::*;
use macroquad::ui::widgets::Button;

use macroquad::ui::{
  hash, root_ui,
  widgets::{self, Group},
  Drag, Ui,
};

use ::rand::{seq::SliceRandom, thread_rng};
use strum::IntoEnumIterator;
use crate::card::*;

mod board;
mod card;
mod global;
mod piece;

#[derive(Debug, PartialEq)]
struct Vecf {
  i: f32,
  j: f32,
}

#[derive(Debug, PartialEq, Clone)]
struct Vecu {
  i: usize,
  j: usize,
}

const image_size: f32 = 50.;
const image_dist: f32 = 5.;

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
  let blue_pawn_dead_image= load_texture("assets/blue_pawn_dead.png").await.unwrap();

  let blue_king_image = load_texture("assets/blue_king.png").await.unwrap();
  let blue_king_select_image = load_texture("assets/blue_king_select.png").await.unwrap();
  let blue_king_dead_image= load_texture("assets/blue_king_dead.png").await.unwrap();

  let red_pawn_image = load_texture("assets/red_pawn.png").await.unwrap();
  let red_pawn_select_image = load_texture("assets/red_pawn_select.png").await.unwrap();
  let red_pawn_dead_image= load_texture("assets/red_pawn_dead.png").await.unwrap();

  let red_king_image = load_texture("assets/red_king.png").await.unwrap();
  let red_king_select_image = load_texture("assets/red_king_select.png").await.unwrap();
  let red_king_dead_image= load_texture("assets/red_king_dead.png").await.unwrap();

  let empty_image = load_texture("assets/empty.png").await.unwrap();
  let empty_dead_image= load_texture("assets/empty_dead.png").await.unwrap();

  // let mut can_move_pos_vec: Vec<Vecu> = vec![];
  let mut curr_player_move_vec: Vec<(piece::Coord, Vec<(&Card, Vec<piece::Coord>)>)> = vec![];

  let button_pos_vec = (0..5)
    .map(|ind| {
      (0..5)
        .map(|jnd| Vecf {
          i: 100. + ind as f32 * (image_size + image_dist),
          j: 100. + jnd as f32 * (image_size + image_dist),
        })
        .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>();
  let mut selected_pos = Vecu { i: 69, j: 69 };

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
  //       std::mem::swap(&mut player_one_card_vec[ind], &mut middle_card);
  //       std::mem::swap(&mut player_one_card_vec, &mut player_two_card_vec);
  //     }
  //     Err(e) => {
  //       println!("error: {}", e);
  //       continue;
  //     }
  //   }
  //   if board.way_of_stone(curr_player, opponent_player) || board.way_of_stream(curr_player) {
  //     still_playing = false;
  //   }
  //   std::mem::swap(&mut curr_player, &mut opponent_player);
  //   println!("{}", board);
  // }

  loop {
    let button_vec = (0..)
      .zip(board.clone().board.iter())
      .zip(button_pos_vec.iter())
      .map(|((ind, piece_line), pos_vec)| {
        (0..)
          .zip(piece_line.iter())
          .zip(pos_vec.iter())
          .map(|((jnd, piece), pos)| {
            widgets::Button::new(
              // get_piece_image(piece_char, selected_pos.clone(), ind, jnd).await
              // if selected_pos == (Vecu {i: ind, j: jnd}) {


              match (piece.name, piece.colour) {
                (piece::Name::Pawn, piece::Colour::Blue) => [blue_pawn_image, blue_pawn_select_image, blue_pawn_dead_image],

                (piece::Name::Master, piece::Colour::Blue)  => [blue_king_image, blue_king_select_image, blue_king_dead_image],

                (piece::Name::Pawn, piece::Colour::Red)  => [red_pawn_image, red_pawn_select_image, red_pawn_select_image],

                (piece::Name::Master, piece::Colour::Red)  => [red_king_image, red_king_select_image, red_king_select_image],

                _ => [empty_image, empty_image, empty_dead_image],
              }[
              if board.contains_move(ind, jnd, curr_player_move_vec.clone()) {
                2
              }
              // if curr_player_move_vec.contains(&Vecu {i: ind, j: jnd})
              else if selected_pos == (Vecu { i: ind, j: jnd }) {
                1
              } else {
                0
              }], // get_piece_image(piece_char, selected_pos, ind, jnd)
            )
            .size(vec2(50., 50.))
            .position(vec2(pos.j, pos.i))
          })
          .collect::<Vec<Button>>()
      })
      .collect::<Vec<Vec<Button>>>();

    // // let bruh = &button_vec;

    if is_key_down(KeyCode::Escape) {
      selected_pos = Vecu { i: 69, j: 69 };
      curr_player_move_vec = vec![];
    }

    // f ind % 5 == 0 {
    for (ind, button_vec) in (0..).zip(button_vec) {
      for (jnd, button) in (0..).zip(button_vec) {
        if button.ui(&mut *root_ui()) {
          selected_pos = Vecu { i: ind, j: jnd };

          curr_player_move_vec = vec![];

          // let mut curr_player_move_vec: Vec<(piece::Coord, Vec<(&Card, Vec<piece::Coord>)>)> = vec![];
          for piece_line in board.board.iter() {
            for piece in piece_line.iter() {
              if piece.colour == curr_player && piece.coord == (piece::Coord {i: ind, j: jnd}) {
                let mut move_vec: Vec<(&Card, Vec<piece::Coord>)> = vec![];
                for card in player_one_card_vec.iter() {
                  move_vec.push((card, piece.get_move_vec(&board, card.value())))
                }
                curr_player_move_vec.push((piece.coord.clone(), move_vec))
              }
            }
          }
        }
      }
    }
    next_frame().await;
  }
}