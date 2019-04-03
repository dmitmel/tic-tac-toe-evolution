use crate::board::*;

pub const INSTRUCTIONS_PER_MOVE: usize = 25;

pub type Instruction = u8;
pub type Program = Vec<Instruction>;

pub mod instructions {
  pub use super::Instruction;

  pub const MOVE_UP: Instruction = 0;
  pub const MOVE_UP_RIGHT: Instruction = 1;
  pub const MOVE_RIGHT: Instruction = 2;
  pub const MOVE_DOWN_RIGHT: Instruction = 3;
  pub const MOVE_DOWN: Instruction = 4;
  pub const MOVE_DOWN_LEFT: Instruction = 5;
  pub const MOVE_LEFT: Instruction = 6;
  pub const MOVE_UP_LEFT: Instruction = 7;

  pub const CHECK_MARK: Instruction = 8;
  pub const PLACE_MARK: Instruction = 9;

  #[allow(non_snake_case)]
  pub fn JUMP(instructions: u8) -> Instruction {
    PLACE_MARK + instructions
  }
}

pub struct Bot {
  program: Program,
  instruction_pointer: usize,
  mark: PlayerMark,
  head_x: Coord,
  head_y: Coord,
}

impl Bot {
  pub fn new(program: Program, mark: PlayerMark, board: &Board) -> Self {
    Self {
      program,
      instruction_pointer: 0,
      mark,
      head_x: board.width / 2,
      head_y: board.height / 2,
    }
  }

  pub fn make_move(&mut self, board: &mut Board) {
    for _executed_instructions in 0..INSTRUCTIONS_PER_MOVE {
      let instruction = self.program[self.instruction_pointer];
      self.execute_instruction(instruction, board);
      if instruction == instructions::PLACE_MARK {
        break;
      }
    }
  }

  fn execute_instruction(
    &mut self,
    instruction: Instruction,
    board: &mut Board,
  ) {
    macro_rules! change_coord {
      ($coord:expr, $max_name:expr, 0) => {};
      ($coord:expr, $max:expr, +) => {
        $coord = if $coord < $max - 1 { $coord + 1 } else { 0 };
      };
      ($coord:expr, $max:expr, -) => {
        $coord = if $coord > 0 { $coord - 1 } else { $max - 1 };
      };
    }

    macro_rules! movement_instructions {
      ($($instruction:ident => ($x_change:tt, $y_change:tt),)*) => {
        {
          match instruction {
            $($instruction => {
              change_coord!(self.head_x, board.width, $x_change);
              change_coord!(self.head_y, board.height, $y_change);
            })*
            _ => unreachable!(),
          }
        }
      };
    }

    use instructions::*;
    match instruction {
      MOVE_UP..=MOVE_UP_LEFT => movement_instructions! {
        MOVE_UP         => (0, -),
        MOVE_UP_RIGHT   => (+, -),
        MOVE_RIGHT      => (+, 0),
        MOVE_DOWN_RIGHT => (+, +),
        MOVE_DOWN       => (0, +),
        MOVE_DOWN_LEFT  => (-, +),
        MOVE_LEFT       => (-, 0),
        MOVE_UP_LEFT    => (-, -),
      },

      CHECK_MARK => {
        self.skip_instructions(match board.get(self.head_x, self.head_y) {
          None => 1,
          Some(mark) if mark == self.mark => 2,
          _ => 3,
        });
        return;
      }

      PLACE_MARK => {
        board.set(self.head_x, self.head_y, Some(self.mark));
      }

      jump => {
        self.skip_instructions((jump - PLACE_MARK) as usize);
        return;
      }
    }

    self.skip_instructions(1);
  }

  fn skip_instructions(&mut self, instructions: usize) {
    self.instruction_pointer += instructions;
    self.instruction_pointer %= self.program.len();
  }
}
