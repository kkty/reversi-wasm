use std::collections::HashMap;

use crate::common::{Board, Coordinate, Player};
use crate::strategy::Strategy;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);
}

pub struct MinMaxStrategy {
  player: Player,
  depth: i32,
  // for memoization
  board_scores: HashMap<Board, i32>,
}

struct Node {
  board: Board,
  edges: Vec<(Coordinate, Node)>,
}

impl MinMaxStrategy {
  pub fn new(player: Player, depth: i32) -> MinMaxStrategy {
    MinMaxStrategy {
      player,
      depth,
      board_scores: HashMap::new(),
    }
  }

  fn eval_board(&mut self, board: Board) -> i32 {
    if let Some(score) = self.board_scores.get(&board) {
      return *score;
    }

    let mut score = 0;

    // high score for having more stones in the later stage
    if board.count(self.player) + board.count(self.player.other()) >= 55 {
      score += (board.count(self.player) - board.count(self.player.other())) * 5;
    }

    for x in 0..8 {
      for y in 0..8 {
        if board.is_valid_move(Coordinate { x, y }, self.player) {
          // high score for having many options
          score = score + 1;
        }

        if board.is_valid_move(Coordinate { x, y }, self.player.other()) {
          // low score for giving many options
          score = score - 1;
        }

        if (x == 0 || x == 7) && (y == 0 || y == 7) {
          if board.cell(Coordinate { x, y }) == Some(self.player) {
            // good score for getting corners
            score = score + 10;
          } else if board.is_valid_move(Coordinate { x, y }, self.player.other()) {
            // low score for losing corners
            score = score - 10;
          }
        }
      }
    }

    self.board_scores.insert(board, score);

    score
  }

  fn construct_tree(&self, board: Board, depth: i32) -> Node {
    let mut root = Node {
      board,
      edges: Vec::new(),
    };

    if depth > 0 {
      for x in 0..8 {
        for y in 0..8 {
          if let Some(board_updated) = board.simulate(Coordinate { x, y }, self.player) {
            root.edges.push((
              Coordinate { x, y },
              self.construct_tree(board_updated, depth - 1),
            ));
          }
        }
      }
    }

    root
  }

  fn eval_tree(&mut self, node: &Node, opponent: bool) -> (i32, Option<Coordinate>) {
    if node.edges.len() == 0 {
      return (self.eval_board(node.board), None);
    }

    let mut score;
    let mut c = None;

    if opponent {
      // find the minimum
      score = std::i32::MAX;

      for edge in node.edges.iter() {
        let s = self.eval_tree(&edge.1, !opponent).0;
        if s < score {
          score = s;
          c = Some(edge.0);
        }
      }
    } else {
      // find the maximum
      score = std::i32::MIN;

      for edge in node.edges.iter() {
        let s = self.eval_tree(&edge.1, !opponent).0;
        if s > score {
          score = s;
          c = Some(edge.0);
        }
      }
    }

    (score, c)
  }

  fn tree_size(&self, tree: &Node) -> i32 {
    let mut cnt = 0;
    for edge in tree.edges.iter() {
      cnt += self.tree_size(&edge.1);
    }
    cnt + 1
  }
}

impl Strategy for MinMaxStrategy {
  fn make_move(&mut self, board: Board) -> Option<Coordinate> {
    let tree = self.construct_tree(board, self.depth);
    log(&format!("tree size: {}", self.tree_size(&tree)));
    return self.eval_tree(&tree, false).1;
  }

  fn to_string(&self) -> String {
    format!("MinMax strategy (depth: {})", self.depth)
  }
}
