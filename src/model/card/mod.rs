use strum_macros::EnumIter;

use crate::model::card;
use crate::model::piece;

#[derive(Clone, Debug)]
pub enum CardItem {
  Empty,
  Middle,
  Goto,
}

#[derive(Debug, EnumIter, PartialEq, Eq)]
pub enum Card {
  Bear,
  Boar,
  Cobra,
  Crab,
  Crane,
  Dog,
  Dragon,
  Eel,
  Elephant,
  Fox,
  Frog,
  Giraffe,
  Goose,
  Horse,
  Iguana,
  Kirin,
  Mantis,
  Monkey,
  Mouse,
  Otter,
  Ox,
  Panda,
  Phoenix,
  Rabbit,
  Rat,
  Rooster,
  Sable,
  SeaSnake,
  Tanuki,
  Tiger,
  Turtle,
  Viper,
}

impl Card {
  pub fn value(&self) -> Vec<Vec<card::CardItem>> {
    use crate::model::card::Card::*;

    let card_ref = match self {
      Bear => [".....", ".oo..", "..O..", "...o.", "....."],
      Boar => [".....", "..o..", ".oOo.", ".....", "....."],
      Cobra => [".....", "...o.", ".oO..", "...o.", "....."],
      Crab => [".....", "..o..", "o.O.o", ".....", "....."],
      Crane => [".....", "..o..", "..O..", ".o.o.", "....."],
      Dog => [".....", ".o...", ".oO..", ".o...", "....."],
      Dragon => [".....", "o...o", "..O..", ".o.o.", "....."],
      Eel => [".....", ".o...", "..O.o", ".o...", "....."],
      Elephant => [".....", ".o.o.", ".oOo.", ".....", "....."],
      Fox => [".....", "...o.", "..Oo.", "...o.", "....."],
      Frog => [".....", ".o...", "o.O..", "...o.", "....."],
      Giraffe => [".....", "o...o", "..O..", "..o..", "....."],
      Goose => [".....", ".o...", ".oOo.", "...o.", "....."],
      Horse => [".....", "..o..", ".oO..", "..o..", "....."],
      Iguana => [".....", "o.o..", "..O..", "...o.", "....."],
      Kirin => [".o.o.", ".....", "..O..", ".....", "..o.."],
      Mantis => [".....", ".o.o.", "..O..", "..o..", "....."],
      Monkey => [".....", ".o.o.", "..O..", ".o.o.", "....."],
      Mouse => [".....", "..o..", "..Oo.", ".o...", "....."],
      Otter => [".....", ".o...", "..O.o", "....o.", "....."],
      Ox => [".....", "..o..", "..Oo.", "..o..", "....."],
      Panda => [".....", "..oo.", "..O..", ".o...", "....."],
      Phoenix => [".....", ".o.o.", "o.O.o", ".....", "....."],
      Rabbit => [".....", "...o.", "..O.o", ".o...", "....."],
      Rat => [".....", "..o..", ".oO..", "...o.", "....."],
      Rooster => [".....", "...o.", ".oOo.", ".o...", "....."],
      Sable => [".....", "...o.", "o.O..", ".o...", "....."],
      SeaSnake => [".....", "..o..", "..O.o", ".o...", "....."],
      Tanuki => [".....", "..o.o", "..O..", ".o...", "....."],
      Tiger => ["..o..", ".....", "..O..", "..o..", "....."],
      Turtle => [".....", ".....", "o.O.o", ".o.o.", "....."],
      Viper => [".....", "..o..", "o.O..", "...o.", "....."],
    };

    let mut card: Vec<Vec<card::CardItem>> = vec![vec![]; 5];

    for (i, line) in (0..).zip(card_ref.iter()) {
      for (_, item) in (0..).zip(line.chars()) {
        card[i].push(match item {
          'o' => card::CardItem::Goto,
          'O' => card::CardItem::Middle,
          _ => card::CardItem::Empty,
        })
      }
    }

    card
  }

  pub fn colour(&self) -> piece::Colour {
    use crate::model::card::Card::*;

    match self {
      Bear | Crab | Crane | Dog | Eel | Giraffe | Goose | Monkey | Mouse | Ox | Phoenix |Rabbit | Sable | SeaSnake | Tanuki | Tiger => piece::Colour::Blue,
      Boar | Cobra | Dragon | Elephant | Fox | Frog | Horse | Iguana | Kirin | Mantis | Otter | Panda | Rat | Rooster | Turtle | Viper => piece::Colour::Red,
    }
  }
}
