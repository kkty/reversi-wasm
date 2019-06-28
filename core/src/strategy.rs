use crate::common::{Board, Coordinate};

pub mod naive;
pub mod minmax;
pub mod random;

pub trait Strategy {
  fn make_move(&mut self, board: Board) -> Option<Coordinate>;
  fn to_string(&self) -> String;
}
