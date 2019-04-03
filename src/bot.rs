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

#[derive(Debug)]
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

#[cfg(test)]
mod tests {
  use super::instructions::*;
  use super::*;

  #[test]
  fn test_constructor() {
    let program = vec![1, 2, 3, 4, 5];
    let mark = PlayerMark::X;
    let board = Board::new(1, 1);

    let bot = Bot::new(program.clone(), mark, &board);

    assert_eq!(bot.program, program);
    assert_eq!(bot.instruction_pointer, 0);
    assert_eq!(bot.mark, mark);
  }

  #[test]
  fn test_initial_head_position() {
    let board = Board::new(1, 1);

    let bot1 = Bot::new(vec![], PlayerMark::O, &board);
    let bot2 = Bot::new(vec![], PlayerMark::X, &board);

    assert_eq!(bot1.head_x, bot2.head_x);
    assert_eq!(bot1.head_y, bot2.head_y);
  }

  #[cfg(test)]
  mod instructions {
    use super::*;
    use std::iter;

    #[test]
    fn test_movement() {
      let (program, instructions_count) = {
        let program = vec![
          MOVE_UP,
          MOVE_UP_RIGHT,
          MOVE_RIGHT,
          MOVE_DOWN_RIGHT,
          MOVE_DOWN,
          MOVE_DOWN_LEFT,
          MOVE_LEFT,
          MOVE_UP_LEFT,
        ];
        let instructions_count = program.len();
        (
          program
            .iter()
            .flat_map(|instruction| vec![*instruction, PLACE_MARK])
            .collect(),
          instructions_count,
        )
      };
      let mut board = Board::new(10, 10);
      let mut bot = Bot::new(program, PlayerMark::O, &board);

      let first_head_x = bot.head_x;
      let first_head_y = bot.head_y;
      let mut prev_head_x = first_head_x;
      let mut prev_head_y = first_head_y;

      for _ in 0..instructions_count {
        bot.make_move(&mut board);
        assert!(bot.head_x != prev_head_x || bot.head_y != prev_head_y);
        prev_head_x = bot.head_x;
        prev_head_y = bot.head_y;
      }

      assert_eq!(first_head_x, bot.head_x);
      assert_eq!(first_head_y, bot.head_y);
    }

    #[test]
    fn test_movement_board_loop() {
      let board_size = 5;
      let program = {
        let mut program: Program = vec![];
        for instruction in &[MOVE_UP, MOVE_RIGHT, MOVE_DOWN, MOVE_LEFT] {
          for _ in 0..board_size {
            program.push(*instruction);
          }
          program.push(PLACE_MARK);
        }
        program
      };
      let mut board = Board::new(board_size, board_size);
      let mut bot = Bot::new(program, PlayerMark::X, &board);

      let first_head_x = bot.head_x;
      let first_head_y = bot.head_y;
      for _ in 0..4 {
        bot.make_move(&mut board);

        assert_eq!(bot.head_x, first_head_x);
        assert_eq!(bot.head_y, first_head_y);
      }
    }

    #[test]
    fn test_instruction_limit_per_move() {
      let program: Program =
        iter::repeat(MOVE_DOWN).take(INSTRUCTIONS_PER_MOVE + 1).collect();
      let mut board =
        Board::new(INSTRUCTIONS_PER_MOVE * 2, INSTRUCTIONS_PER_MOVE * 2);
      let mut bot = Bot::new(program, PlayerMark::O, &board);
      bot.head_x = 0;
      bot.head_y = 0;

      bot.make_move(&mut board);

      assert_eq!(bot.head_y, INSTRUCTIONS_PER_MOVE);
    }

    #[test]
    fn test_place_mark() {
      let program: Program = vec![
        PLACE_MARK, MOVE_RIGHT, PLACE_MARK, MOVE_RIGHT, MOVE_RIGHT, PLACE_MARK,
      ];
      let mut board = Board::new(10, 10);
      let mut bot = Bot::new(program, PlayerMark::X, &board);

      for _ in 0..3 {
        bot.make_move(&mut board);

        assert_eq!(board.get(bot.head_x, bot.head_y).unwrap(), bot.mark);
      }
    }

    #[test]
    fn test_unconditional_jump() {
      let program =
        vec![MOVE_DOWN, JUMP(2), MOVE_RIGHT, MOVE_RIGHT, PLACE_MARK];
      let mut board = Board::new(3, 3);
      let mut bot = Bot::new(program, PlayerMark::O, &board);
      bot.head_x = 0;
      bot.head_y = 0;

      bot.make_move(&mut board);

      assert_eq!(board.get(1, 1).unwrap(), bot.mark);
    }

    #[test]
    fn test_tape_loop() {
      let program = vec![
        CHECK_MARK,
        JUMP(5),
        JUMP(2),
        JUMP(3),
        MOVE_RIGHT,
        JUMP(2),
        PLACE_MARK,
      ];
      let mut board = Board::new(5, 1);
      board.set(4, 0, Some(PlayerMark::O));
      board.set(2, 0, Some(PlayerMark::O));
      let mut bot = Bot::new(program, PlayerMark::X, &board);
      bot.head_x = 0;
      bot.head_y = 0;

      for _ in 0..board.width {
        bot.make_move(&mut board);
      }

      for x in 0..board.width {
        assert_eq!(board.get(x, 0).unwrap(), PlayerMark::X);
      }
    }

    #[test]
    fn test_check_mark() {
      fn run_test(intial_mark: Option<PlayerMark>, expected_x: Coord) {
        let program =
          vec![CHECK_MARK, MOVE_RIGHT, MOVE_RIGHT, MOVE_RIGHT, PLACE_MARK];
        let mut board = Board::new(4, 1);
        let mut bot = Bot::new(program, PlayerMark::X, &board);
        bot.head_x = 0;
        bot.head_y = 0;
        board.set(bot.head_x, bot.head_y, intial_mark);

        bot.make_move(&mut board);

        assert_eq!(bot.head_x, expected_x);
      }

      run_test(None, 3);
      run_test(Some(PlayerMark::X), 2);
      run_test(Some(PlayerMark::O), 1);
    }
  }
}
