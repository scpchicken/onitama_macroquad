#![allow(warnings)]
use macroquad::prelude::*;
use macroquad::ui::widgets::Button;

use macroquad::ui::{
  hash, root_ui,
  widgets::{self, Group},
  Drag, Ui,
};

#[derive(Debug, PartialEq)]
struct Vecc {
  i: f32,
  j: f32,
}

const image_size: f32 = 50.;
const image_dist: f32 = 5.;

#[macroquad::main("UI showcase")]
async fn main() {
  let blue_pawn_image = load_texture("assets/blue_pawn.png").await.unwrap();
  let blue_pawn_image_select = load_texture("assets/blue_pawn_select").await.unwrap();
  let blue_king_image = load_texture("assets/blue_king.png").await.unwrap();
  let blue_king_image_select = load_texture("assets/blue_king_select").await.unwrap();
  let red_pawn_image = load_texture("assets/red_pawn.png").await.unwrap();
  let red_pawn_image_select = load_texture("assets/red_pawn_select").await.unwrap();
  let red_king_image = load_texture("assets/red_king.png").await.unwrap();
  let red_king_image_select = load_texture("assets/red_king_select").await.unwrap();
  let empty_image = load_texture("assets/empty.png").await.unwrap();

  let board_ref = ["bbBbb", ".....", ".....", ".....", "rrRrr"];

  let button_pos_vec = (0..5)
    .map(|ind| {
      (0..5)
        .map(|jnd| Vecc {
          i: 100. + ind as f32 * (image_size + image_dist),
          j: 100. + jnd as f32 * (image_size + image_dist)
        })
        .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>();
  let mut selected_pos = Vecc { i: 69., j: 69. };

  // let button_vec = (0..).zip(button_pos_vec.iter())
  //   .map(|(ind, pos)| {
  //     widgets::Button::new(
  //       if clicked && ind == 0 {
  //         blue_king_image
  //       } else {
  //         empty_image
  //       }
  //     )
  //       .size(vec2(50., 50.))
  //       .position(vec2(pos.j, pos.i))
  //   })
  //   .collect::<Vec<Button>>().clone();

  // let mut ind = 0;
  loop {
    // let button_vec = (0..5)
    //   .map(|ind| {
    //     widgets::Button::new(
    //       empty_image
    //     )
    //       .size(vec2(50., 50.))
    //       .position(vec2(100. + ind as f32 * 50., 100.))
    //   })
    //   .collect::<Vec<Button>>();

    // for button in button_vec {
    //   button.ui(&mut *root_ui());
    // }
    let button_vec = (0..)
      .zip(board_ref.iter())
      .zip(button_pos_vec.iter())
      .map(|((ind, piece_line), pos_vec)| {
        (0..)
          .zip(piece_line.chars())
          .zip(pos_vec.iter())
          .map(|((jnd, piece_char), pos)| {
            widgets::Button::new(
              match piece_char {
                'b' => blue_pawn_image,

                'B' => blue_king_image,

                'r' => red_pawn_image,

                'R' => red_king_image,

                _ => empty_image,
              }, // if selected_pos == (Vecc {i: ind as f32, j: jnd as f32}) {
                 //   blue_king_image
                 // } else {
                 //   empty_image
                 // }
            )
            .size(vec2(50., 50.))
            .position(vec2(pos.j, pos.i))
          })
          .collect::<Vec<Button>>()
      })
      .collect::<Vec<Vec<Button>>>();

    // // let bruh = &button_vec;

    // f ind % 5 == 0 {
    for (ind, button_vec) in (0..).zip(button_vec) {
      for (jnd, button) in (0..).zip(button_vec) {
        if button.ui(&mut *root_ui()) {
          selected_pos = Vecc {
            i: ind as f32,
            j: jnd as f32,
          }
        }
      }
    }

    if is_key_down(KeyCode::Escape) {
      selected_pos = Vecc { i: 69., j: 69. };
    }
    // // }

    // ind += 1;

    // let button_vec = (0..).zip(button_pos_vec.iter())
    // .map(|(ind, pos)| {
    //   widgets::Button::new(
    //     if clicked && ind == 0 {
    //       blue_king_image
    //     } else {
    //       empty_image
    //     }
    //   )
    //     .size(vec2(50., 50.))
    //     .position(vec2(pos.j, pos.i))
    // })
    // .collect::<Vec<Button>>();

    // for button in button_vec {
    //   if button.ui(&mut *root_ui()) {
    //     clicked = !clicked;
    //   }
    next_frame().await;
  }
  // for button in &button_vec {
  //   if *button.ui(&mut *root_ui()) {
  //     println!("you click")
  //   }
  // }
  // for (button_pos, button) in button_pos_vec.iter().zip(&button_vec) {
  //   // println!("{:#?}", button_pos)
  //   if button.ui(&mut *root_ui()) {
  //     button_vec[0] = widgets::Button::new(
  //     empty_image
  //   )
  //     .size(vec2(50., 50.))
  //     .position(vec2(button_pos_vec[0].j, button_pos_vec[0].i))
  //   }
  // }
  //   if button {
  //     button_vec[0] = widgets::Button::new(
  //       blue_king_image
  //     )
  //       .size(vec2(50., 50.))
  //       .position(vec2(100. + ind as f32 * 50., 100.))
  //   })
  //   }
  // }

  // for button in button_vec {
  //   // println!("{:#?}", button.size(_));
  //   button.ui(&mut *root_ui());
  // }
  //   let board: Vec<Vec<Button>> = (0..)
  //     .zip(board_ref.iter())
  //     .map(|(ind, piece_line)| {
  //       (0..).zip(piece_line.chars()).
  //         map(|(jnd, piece_char)| {
  //           widgets::Button::new(
  //             match piece_char {
  //               'b' => {
  //                 blue_pawn_image
  //               },

  //               'B' => {
  //                 blue_king_image
  //               },

  //               'r' => {
  //                 red_pawn_image
  //               },

  //               'R' => {
  //                 red_king_image
  //               }

  //               _ => {
  //                 empty_image
  //               }
  //             }
  //           )
  //             .size(vec2(50., 50.))
  //             .position(vec2(100. + (jnd as f32) * 50., 100. + (ind as f32) * 50.))
  //             // .ui(&mut *root_ui())
  //       }).collect::<Vec<Button>>()
  //     })
  //     .collect::<Vec<Vec<Button>>>();

  // // for piece in

  // for piece_line in board.iter() {
  //   for piece in piece_line.iter() {
  //     piece.ui(&mut *root_ui());
  //     // println!("{:#?}", piece);
  //   }
  // }

  // for (ind, piece_line) in (0..).zip(board_ref.iter()) {
  //   for (jnd, piece_char) in (0..).zip(piece_line.chars()) {
  //     let button = widgets::Button::new(
  //       match piece_char {
  //         'b' => {
  //           blue_pawn_image
  //         },

  //         'B' => {
  //           blue_king_image
  //         },

  //         'r' => {
  //           red_pawn_image
  //         },

  //         'R' => {
  //           red_king_image
  //         }

  //         _ => {
  //           empty_image
  //         }
  //       }
  //     )
  //       .size(vec2(50., 50.))
  //       .position(vec2(100. + (jnd as f32) * 50., 100. + (ind as f32) * 50.))
  //       .ui(&mut *root_ui());

  //     if button {
  //       println!("you click button");
  //     }
  //   }
  // }
}
