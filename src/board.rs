pub type Coord = usize;

#[derive(Debug)]
pub struct Board {
  pub width: Coord,
  pub height: Coord,
  cells: Vec<Cell>,
}

#[derive(Debug, Clone, Copy)]
pub enum Cell {
  Empty,
  Player1,
  Player2,
}

impl Board {
  pub fn new(width: Coord, height: Coord) -> Self {
    Self { width, height, cells: vec![Cell::Empty; width * height] }
  }

  fn assert_in_bounds(&self, x: Coord, y: Coord) {
    macro_rules! is_in_bounds {
      ($val:expr, $val_name:expr, $max:expr, $max_name:expr) => {
        if $val >= $max {
          panic!(
            "{} out of bounds: the {} is {} but the {} is {}",
            $val_name, $max_name, $max, $val_name, $val,
          );
        }
      };
    }

    is_in_bounds!(x, "x", self.width, "width");
    is_in_bounds!(y, "y", self.height, "height");
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
