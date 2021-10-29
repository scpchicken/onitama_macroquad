use crate::global::*;
use crate::{piece, piece::*};

#[derive(Clone)]
pub struct Board(pub Vec<Vec<Piece>>);


impl Board {
  pub fn as_vec(&self) -> Vec<Vec<Piece>> {
    self.0.clone()
  }

  pub fn at_pos(&self, coord: piece::Coord) -> piece::Piece {
    self.as_vec()[coord.i][coord.j].clone()
  }

  pub fn move_piece(&mut self, start: piece::Coord, end: piece::Coord) {
    let piece = &self.0[start.i][start.j];

    self.as_vec()[end.i][end.j] = piece::Piece {
      name: piece.name,
      colour: piece.colour,
      coord: piece::Coord { i: end.i, j: end.j },
    };

    self.as_vec()[start.i][start.j] = piece::Piece {
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
    curr_player_move_vec: Vec<Vec<piece::Coord>>,
  ) -> bool {
    // for piece_move in curr_player_move_vec.iter() {
      for coord_vec in curr_player_move_vec.iter() {
        for i in coord_vec.iter() {
          if i == &(Coord { i: ind, j: jnd }) {
            return true;
          }
        }
      }
    // }

    return false;
  }

  pub fn contains(&self, colour: piece::Colour, name: piece::Name) -> bool {
    for piece_line in self.as_vec().iter() {
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
