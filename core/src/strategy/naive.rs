use crate::common::{Board, Coordinate, Player};
use crate::strategy::Strategy;

pub struct NaiveStrategy {
  player: Player,
}

impl NaiveStrategy {
  pub fn new(player: Player) -> NaiveStrategy {
    NaiveStrategy { player }
  }
}

impl Strategy for NaiveStrategy {
  fn make_move(&mut self, board: Board) -> Option<Coordinate> {
    for x in 0..8 {
      for y in 0..8 {
        if board.is_valid_move(Coordinate { x, y }, self.player) {
          return Some(Coordinate { x, y });
        }
      }
    }

    return None;
  }

  fn to_string(&self) -> String {
    format!("naive strategy")
  }
}
