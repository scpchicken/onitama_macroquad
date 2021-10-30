use std::fmt;

use crate::model::board;
use crate::model::card;
use crate::model::piece;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Piece {
  pub name: piece::Name,
  pub colour: piece::Colour,
  pub coord: piece::Coord,
}

impl fmt::Display for Piece {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{}",
      match (&self.name, &self.colour) {
        (piece::Name::Pawn, piece::Colour::Blue) => "b",
        (piece::Name::Master, piece::Colour::Blue) => "B",
        (piece::Name::Pawn, piece::Colour::Red) => "a",
        (piece::Name::Master, piece::Colour::Red) => "A",
        _ => ".",
      }
    )
  }
}

impl Piece {
  pub fn get_move_vec(
    &self,
    board: &board::Board,
    card_val: Vec<Vec<card::CardItem>>,
  ) -> Vec<piece::Coord> {
    let mut move_vec: Vec<piece::Coord> = vec![];

    for (i, line) in (0..).zip(card_val.iter()) {
      for (j, item) in (0..).zip(line.iter()) {
        match item {
          card::CardItem::Goto => {
            let piece_is_blue = self.colour == piece::Colour::Blue;

            let offset_i = if piece_is_blue { 2 - i } else { i - 2 } + self.coord.i as isize;
            let offset_j = if piece_is_blue { 2 - j } else { j - 2 } + self.coord.j as isize;

            if offset_i >= 0
              && offset_i < 5
              && offset_j >= 0
              && offset_j < 5
              && board.0[offset_i as usize][offset_j as usize].colour != self.colour
            {
              move_vec.push(piece::Coord {
                i: offset_i as usize,
                j: offset_j as usize,
              })
            }
          }
          _ => {}
        }
      }
    }

    move_vec
  }
}

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub enum Colour {
  Red,
  Blue,
  Empty,
}

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub struct Coord {
  pub i: usize,
  pub j: usize,
}

impl fmt::Display for Coord {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "({}, {})", self.i, self.j)
  }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum Name {
  Master,
  Pawn,
  Empty,
}
