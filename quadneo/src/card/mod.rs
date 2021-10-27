use strum_macros::EnumIter;

use crate::card::{Card::*, CardItem::*};

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum CardItem {
  Empty,
  Middle,
  Goto,
}

#[derive(Debug, EnumIter, PartialEq, Eq)]
pub enum Card {
  Dragon,
  Crab,
  Monkey,
  Ox,
  Boar,
  Locomoco,
}

impl Card {
  pub fn value(&self) -> Vec<Vec<CardItem>> {
    let card_ref = match self {
      Dragon => [".....", "o...o", "..O..", ".o.o.", "....."],

      Locomoco => ["...oo", "...oo", "..O..", ".....", "....."],

      Crab => [".....", "..o..", "o.O.o", ".....", "....."],

      Monkey => [".....", ".o.o.", "..O..", ".o.o.", "....."],
      Ox => [".....", "..o..", "..Oo.", "..o..", "....."],

      Boar => [".....", "..o..", ".oOo.", ".....", "....."],
    };

    let mut card: Vec<Vec<CardItem>> = vec![vec![]; 5];

    for (i, line) in (0..).zip(card_ref.iter()) {
      for (_, item) in (0..).zip(line.chars()) {
        card[i].push(match item {
          'o' => Goto,
          'O' => Middle,
          _ => Empty,
        })
      }
    }

    card
  }
}