pub type Coord = usize;
pub type Cell = Option<PlayerMark>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerMark {
  X,
  O,
}

#[derive(Debug)]
pub struct Board {
  pub width: Coord,
  pub height: Coord,
  cells: Vec<Cell>,
}

impl Board {
  pub fn new(width: Coord, height: Coord) -> Self {
    Self { width, height, cells: vec![None; width * height] }
  }

  fn assert_in_bounds(&self, x: Coord, y: Coord) {
    let Self { width, height, .. } = self;
    if x >= self.width {
      panic!("x out of bounds: the width is {} but the x is {}", width, x);
    }
    if y >= self.height {
      panic!("y out of bounds: the height is {} but the y is {}", height, y);
    }
  }

  pub fn get(&self, x: Coord, y: Coord) -> Cell {
    self.assert_in_bounds(x, y);
    self.cells[y * self.width + x]
  }

  pub fn set(&mut self, x: Coord, y: Coord, cell: Cell) {
    self.assert_in_bounds(x, y);
    self.cells[y * self.width + x] = cell;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_constructor() {
    let width = 123;
    let height = 456;

    let board = Board::new(width, height);

    assert_eq!(board.width, width);
    assert_eq!(board.height, height);
    assert_eq!(board.cells.len(), width * height);
  }

  #[test]
  fn test_cells() {
    let x = 1;
    let y = 1;

    let mut board = Board::new(x * 2 + 1, y * 2 + 1);

    assert_eq!(board.get(x, y), None);

    board.set(x, y, Some(PlayerMark::X));
    assert_eq!(board.get(x, y), Some(PlayerMark::X));

    board.set(x, y, Some(PlayerMark::O));
    assert_eq!(board.get(x, y), Some(PlayerMark::O));
  }

  #[test]
  #[should_panic(expected = "x out of bounds")]
  fn test_x_out_of_bounds() {
    let width = 2;
    let height = 2;

    let board = Board::new(width, height);

    board.get(width, 0);
  }

  #[test]
  #[should_panic(expected = "y out of bounds")]
  fn test_y_out_of_bounds() {
    let width = 2;
    let height = 2;

    let board = Board::new(width, height);

    board.get(0, height);
  }
}
