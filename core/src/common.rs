#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
pub enum Player {
  White,
  Black,
}

impl Player {
  pub fn other(&self) -> Player {
    match *self {
      Player::White => Player::Black,
      Player::Black => Player::White,
    }
  }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Coordinate {
  pub x: usize,
  pub y: usize,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct Board {
  cells: [[Option<Player>; 8]; 8],
}

impl Board {
  pub fn initial() -> Board {
    let mut cells = [[None; 8]; 8];

    cells[3][3] = Some(Player::White);
    cells[4][4] = Some(Player::White);
    cells[3][4] = Some(Player::Black);
    cells[4][3] = Some(Player::Black);

    Board { cells }
  }

  pub fn cell(&self, c: Coordinate) -> Option<Player> {
    self.cells[c.x][c.y]
  }

  pub fn count(&self, player: Player) -> i32 {
    let mut cnt = 0;
    for x in 0..8 {
      for y in 0..8 {
        if let Some(p) = self.cells[x][y] {
          if p == player {
            cnt = cnt + 1;
          }
        }
      }
    }

    cnt
  }

  // return None if invalid
  pub fn simulate(&self, c: Coordinate, player: Player) -> Option<Board> {
    if self.cells[c.x][c.y] != None {
      return None;
    }

    let mut cells_to_flip = Vec::new();

    for dx in [-1, 0, 1].iter().cloned() {
      for dy in [-1, 0, 1].iter().cloned() {
        if dx == 0 && dy == 0 {
          continue;
        }

        for i in 1..8 {
          let x = c.x as i32 + dx * (i as i32);
          let y = c.y as i32 + dy * (i as i32);

          if x < 0 || 7 < x || y < 0 || 7 < y {
            break;
          }

          let x = x as usize;
          let y = y as usize;

          if self.cells[x][y] == None {
            break;
          }

          if self.cells[x][y] == Some(player) {
            for j in 1..i {
              cells_to_flip.push(Coordinate {
                x: (c.x as i32 + dx * (j as i32)) as usize,
                y: (c.y as i32 + dy * (j as i32)) as usize,
              });
            }
            break;
          }
        }
      }
    }

    if cells_to_flip.len() == 0 {
      return None;
    }

    let mut board_updated = Board { cells: self.cells };

    board_updated.cells[c.x][c.y] = Some(player);

    for cell_to_flip in cells_to_flip.iter() {
      board_updated.cells[cell_to_flip.x][cell_to_flip.y] = Some(player);
    }

    Some(board_updated)
  }

  pub fn is_valid_move(&self, coordinate: Coordinate, player: Player) -> bool {
    match self.simulate(coordinate, player) {
      None => false,
      Some(_) => true,
    }
  }

  pub fn has_valid_move(&self, player: Player) -> bool {
    for x in 0..8 {
      for y in 0..8 {
        if self.is_valid_move(Coordinate { x, y }, player) {
          return true;
        }
      }
    }

    false
  }
}
