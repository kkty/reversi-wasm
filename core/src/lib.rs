extern crate wasm_bindgen;
mod common;
mod strategy;

use crate::common::{Board, Coordinate, Player};
use crate::strategy::random::RandomStrategy;
use crate::strategy::minmax::MinMaxStrategy;
use crate::strategy::naive::NaiveStrategy;
use crate::strategy::Strategy;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);
}

#[wasm_bindgen]
pub struct Reversi {
  board: Board,
  next: Player,
  player_user: Player,
  player_cpu: Player,
  strategy_cpu: Box<Strategy>,
}

#[wasm_bindgen]
impl Reversi {
  pub fn new(player_user: &str, strategy_cpu: &str) -> Result<Reversi, JsValue> {
    let player_user = match player_user {
      "black" => Player::Black,
      "white" => Player::White,
      _ => return Err(JsValue::from("player_user should be 'white' or 'black'")),
    };

    let player_cpu = player_user.other();

    let strategy_cpu: Box<Strategy> = match strategy_cpu {
      "random" => Box::new(RandomStrategy::new(player_cpu)),
      "minmax2" => Box::new(MinMaxStrategy::new(player_cpu, 2)),
      "minmax4" => Box::new(MinMaxStrategy::new(player_cpu, 4)),
      "minmax8" => Box::new(MinMaxStrategy::new(player_cpu, 8)),
      "naive" => Box::new(NaiveStrategy::new(player_cpu)),
      _ => {
        return Err(JsValue::from(
          "strategy_cpu should be 'random', 'minmax2', 'minmax4', 'minmax8' or 'naive'",
        ))
      }
    };

    Ok(Reversi {
      board: Board::initial(),
      next: Player::Black,
      player_user,
      player_cpu,
      strategy_cpu,
    })
  }

  pub fn board(&self) -> String {
    let mut board = [[0; 8]; 8];

    for x in 0..8 {
      for y in 0..8 {
        match self.board.cell(Coordinate { x, y }) {
          Some(Player::Black) => {
            board[x][y] = 1;
          }
          Some(Player::White) => {
            board[x][y] = 2;
          }
          None => {}
        }
      }
    }

    serde_json::to_string(&board).unwrap()
  }

  pub fn has_valid_move(&self) -> bool {
    self.board.has_valid_move(self.player_user)
  }

  // return false if invalid
  pub fn pass(&mut self) -> bool {
    if self.next != self.player_user {
      return false;
    }

    if self.board.has_valid_move(self.player_user) {
      return false;
    }

    self.next = self.next.other();

    true
  }

  // return false if invalid
  pub fn make_move(&mut self, x: i32, y: i32) -> bool {
    if self.next != self.player_user {
      return false;
    }

    match self.board.simulate(
      Coordinate {
        x: x as usize,
        y: y as usize,
      },
      self.player_user,
    ) {
      Some(board_updated) => {
        self.board = board_updated;
      }
      None => return false,
    }

    self.next = self.next.other();

    true
  }

  pub fn wait_for_cpu(&mut self) {
    match self.strategy_cpu.make_move(self.board) {
      Some(c) => match self.board.simulate(c, self.player_cpu) {
        Some(board_updated) => {
          self.board = board_updated;
        }
        None => panic!("invalid move by cpu"),
      },
      // pass
      None => {
        if self.board.has_valid_move(self.player_cpu) {
          panic!("invalid pass by cpu");
        }
      }
    }

    self.next = self.next.other()
  }

  // return None if the game has not yet ended
  pub fn result(&self) -> Option<String> {
    for p in [Player::Black, Player::White].iter() {
      if self.board.has_valid_move(*p) {
        return None;
      }
    }

    let result = [
      self.board.count(Player::Black),
      self.board.count(Player::White),
    ];

    Some(serde_json::to_string(&result).unwrap())
  }
}
