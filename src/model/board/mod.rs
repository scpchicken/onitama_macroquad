use crate::constant::*;
use crate::model::{piece, piece::*};

#[derive(Clone)]
pub struct Board(pub Vec<Vec<Piece>>);

impl Board {
  pub fn at_pos(&self, coord: piece::Coord) -> piece::Piece {
    self.0[coord.i][coord.j].clone()
  }

  pub fn get_flipped(&self) -> Vec<Vec<Piece>> {
    self
      .clone()
      .0
      .into_iter()
      .map(|piece_line| piece_line.into_iter().rev().collect::<Vec<piece::Piece>>())
      .rev()
      .collect::<Vec<_>>()
  }

  pub fn get_can_die_vec(&self, curr_player_move_vec: Vec<Coord>) -> Vec<Vec<bool>> {
    self
      .clone()
      .0
      .iter()
      .map(|piece_line| {
        piece_line
          .iter()
          .map(|piece| {
            self.contains_move(piece.coord.i, piece.coord.j, curr_player_move_vec.clone())
          })
          .collect::<Vec<bool>>()
      })
      .collect::<Vec<Vec<bool>>>()
  }

  pub fn get_piece(
    &self,
    curr_player: piece::Colour,
    selected_pos: piece::Coord,
  ) -> Option<piece::Piece> {
    self
      .clone()
      .0
      .into_iter()
      .flatten()
      .find(|piece| piece.colour == curr_player && piece.coord == selected_pos)
  }

  pub fn move_piece(&mut self, start: piece::Coord, end: piece::Coord) {
    let piece = &self.0[start.i][start.j];

    self.0[end.i][end.j] = piece::Piece {
      name: piece.name,
      colour: piece.colour,
      coord: piece::Coord { i: end.i, j: end.j },
    };

    self.0[start.i][start.j] = piece::Piece {
      name: piece::Name::Empty,
      colour: piece::Colour::Empty,
      coord: piece::Coord {
        i: start.i,
        j: start.j,
      },
    };
  }

  pub fn contains_move(
    &self,
    ind: usize,
    jnd: usize,
    curr_player_move_vec: Vec<piece::Coord>,
  ) -> bool {
    for coord in curr_player_move_vec.iter() {
      if coord == &(Coord { i: ind, j: jnd }) {
        return true;
      }
    }

    return false;
  }

  pub fn contains(&self, colour: piece::Colour, name: piece::Name) -> bool {
    for piece_line in self.0.iter() {
      for piece in piece_line.iter() {
        if piece.name == name && piece.colour == colour {
          return true;
        }
      }
    }

    false
  }

  pub fn way_of_stone(&self, curr_player: piece::Colour, opponent_player: piece::Colour) -> bool {
    if !self.contains(opponent_player, piece::Name::Master) {
      println!("{:?} wins by way of stone", curr_player);
      return true;
    }

    false
  }
  pub fn way_of_stream(&self, curr_player: piece::Colour) -> bool {
    let blue_temple_arch = self.at_pos(BLUE_TEMPLE_ARCH_POS);
    let red_temple_arch = self.at_pos(RED_TEMPLE_ARCH_POS);

    if (curr_player == piece::Colour::Red
      && blue_temple_arch.name == piece::Name::Master
      && blue_temple_arch.colour == curr_player)
      || (curr_player == piece::Colour::Blue
        && red_temple_arch.name == piece::Name::Master
        && red_temple_arch.colour == curr_player)
    {
      println!("{:?} wins by way of stream", curr_player);
      return true;
    }

    false
  }
}

use std::fmt;

impl fmt::Display for Board {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{:#?}",
      self
        .0
        .iter()
        .map(|x| {
          x.iter()
            .map(|y| format!("{}", y))
            .collect::<Vec<String>>()
            .join(" | ")
        })
        .collect::<Vec<String>>()
    )
  }
}

pub fn get_board() -> Board {
  let board_ref = ["bbBbb", ".....", ".....", ".....", "rrRrr"];

  let mut board: Vec<Vec<Piece>> = vec![vec![]; 5];

  for (i, line) in (0..).zip(board_ref.iter()) {
    for (j, item) in (0..).zip(line.chars()) {
      let (name, colour) = match item {
        'b' => (piece::Name::Pawn, piece::Colour::Blue),
        'B' => (piece::Name::Master, piece::Colour::Blue),
        'r' => (piece::Name::Pawn, piece::Colour::Red),
        'R' => (piece::Name::Master, piece::Colour::Red),
        _ => (piece::Name::Empty, piece::Colour::Empty),
      };

      let piece = Piece {
        name,
        colour,
        coord: piece::Coord { i, j },
      };

      board[i].push(piece)
    }
  }

  Board(board)
}

#[macro_export]
macro_rules! input {
  () => {{
    super::input!(String)
  }};

  ($t:ty) => {{
    let input = &mut "".into();
    std::io::stdin().read_line(input).unwrap();
    if input.ends_with("\n") {
      input.pop();
    };
    input.parse::<$t>().unwrap()
  }};
}
