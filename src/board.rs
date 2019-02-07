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
