use crate::common::{Board, Coordinate, Player};
use crate::strategy::Strategy;

#[derive(Debug)]
pub struct RandomStrategy {
  player: Player,
}

impl RandomStrategy {
  pub fn new(player: Player) -> RandomStrategy {
    RandomStrategy { player }
  }
}

impl Strategy for RandomStrategy {
  fn make_move(&mut self, board: Board) -> Option<Coordinate> {
    let mut valid_moves = Vec::new();

    for x in 0..8 {
      for y in 0..8 {
        if board.is_valid_move(Coordinate { x, y }, self.player) {
          valid_moves.push(Coordinate { x, y });
        }
      }
    }

    if valid_moves.len() == 0 {
      None
    } else {
      let r: usize = rand::random();
      return Some(valid_moves[r % valid_moves.len()]);
    }
  }

  fn to_string(&self) -> String {
    format!("random strategy")
  }
}
