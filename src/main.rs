mod board;
mod bot;

use std::f64;
use std::rc::Rc;

use gio::prelude::*;
use gtk::prelude::*;
use gtk::Application;

use crate::board::{Board, PlayerMark};
use crate::bot::Bot;

const APP_ID: &str = "com.github.dmitmel.tic-tac-toe-evolution";

const BOARD_CELL_SIZE: i32 = 32;
const BOARD_MARK_MARGIN: f64 = 0.1;
const BOARD_MARK_LINE_WIDTH: f64 = 0.25;

fn main() {
  let application =
    Application::new(Some(APP_ID), gio::ApplicationFlags::default())
      .expect("failed to initialize GTK application");

  application.connect_activate(|app| {
    let builder = gtk::Builder::new_from_string(include_str!("main.glade"));

    let mut board = Board::new(100, 100);
    randomly_fill_board(&mut board);
    let players = Rc::new([
      Bot::new(generate_random_program(), PlayerMark::X, &board),
      Bot::new(generate_random_program(), PlayerMark::O, &board),
    ]);

    let ui_board: gtk::DrawingArea = builder.get_object("board").unwrap();
    ui_board.set_size_request(
      board.width() as i32 * BOARD_CELL_SIZE,
      board.height() as i32 * BOARD_CELL_SIZE,
    );
    ui_board.connect_draw(move |ui_board, ctx: &cairo::Context| -> Inhibit {
      let style_ctx = ui_board.get_style_context();
      gtk::render_background(
        &style_ctx,
        ctx,
        0.0,
        0.0,
        f64::from(ui_board.get_allocated_width()),
        f64::from(ui_board.get_allocated_height()),
      );
      render_board(&board, ctx);
      Inhibit(false)
    });

    let ui_player: gtk::ComboBoxText = builder.get_object("player").unwrap();
    for index in 0..players.len() {
      ui_player.append_text(&format!("Player {}", index));
    }
    ui_player.set_active(Some(0));

    {
      let players = Rc::clone(&players);
      let ui_player_info: gtk::Label =
        builder.get_object("player_info").unwrap();
      let ui_player_program_instructions: gtk::ListStore =
        builder.get_object("player_program_instructions").unwrap();
      let update_player_sidebar = move |ui_player: &gtk::ComboBoxText| {
        if let Some(selected) = ui_player.get_active() {
          let selected_player = &players[selected as usize];

          ui_player_info.set_text(&format!(
            "\
Mark: {:?}
Instruction count: {}
Current address: {:04x}
Program:",
            selected_player.mark(),
            selected_player.program().len(),
            selected_player.instruction_pointer(),
          ));

          ui_player_program_instructions.clear();
          for (address, instruction) in
            selected_player.program().iter().enumerate()
          {
            let icon = if address == 0 { Some("gtk-go-forward") } else { None };
            ui_player_program_instructions.insert_with_values(
              None,
              &[0, 1, 2],
              &[
                &icon,
                &format!("{:04x}", address),
                &format!("{:?}", instruction),
              ],
            );
          }
        }
      };
      update_player_sidebar(&ui_player);
      ui_player.connect_changed(update_player_sidebar);
    }

    {
      let players = Rc::clone(&players);
      let ui_player_program: gtk::TreeView =
        builder.get_object("player_program").unwrap();
      let ui_show_current_instruction: gtk::Button =
        builder.get_object("show_current_instruction").unwrap();
      ui_show_current_instruction.connect_clicked(move |_| {
        if let Some(selected) = ui_player.get_active() {
          let selected_player = &players[selected as usize];

          let instruction_pointer: usize =
            selected_player.instruction_pointer();
          ui_player_program.set_cursor(
            &gtk::TreePath::new_from_indicesv(&[instruction_pointer as i32]),
            None::<&gtk::TreeViewColumn>,
            false,
          );
        }
      });
    }

    let window: gtk::ApplicationWindow = builder.get_object("window2").unwrap();
    window.set_application(Some(app));
    window.maximize();
    window.show_all();
  });

  let args: Vec<String> = std::env::args().collect();
  application.run(&args);
}

fn randomly_fill_board(board: &mut Board) {
  use rand::distributions::Bernoulli;
  use rand::Rng;
  let mut rng = rand::thread_rng();
  let dstr = Bernoulli::new(0.5).unwrap();
  for y in 0..board.width() {
    for x in 0..board.height() {
      board.set(
        x,
        y,
        if rng.sample(dstr) {
          Some(if rng.sample(dstr) { PlayerMark::X } else { PlayerMark::O })
        } else {
          None
        },
      )
    }
  }
}

fn generate_random_program() -> bot::Program {
  use rand::distributions::{Standard, Uniform};
  use rand::Rng;
  let mut rng = rand::thread_rng();
  rng
    .sample_iter(Uniform::new(0, 10))
    .take(0x100)
    .map(|n: u8| {
      use crate::bot::Instruction::*;
      use crate::bot::MoveDirection::*;
      match n {
        0..=7 => Move(match n {
          0 => Up,
          1 => UpRight,
          2 => Right,
          3 => DownRight,
          4 => Down,
          5 => DownLeft,
          6 => Left,
          7 => UpLeft,
          _ => unreachable!(),
        }),

        8 => CheckMark,
        9 => PlaceMark,

        10 => Jump(rng.sample(Standard)),

        _ => unreachable!(),
      }
    })
    .collect()
}

fn render_board(board: &Board, ctx: &cairo::Context) {
  ctx.set_line_width(BOARD_MARK_LINE_WIDTH);
  for y in 0..board.width() {
    for x in 0..board.height() {
      if let Some(mark) = board.get(x, y) {
        ctx.save();
        ctx.scale(f64::from(BOARD_CELL_SIZE), f64::from(BOARD_CELL_SIZE));
        ctx.translate(
          x as f64 + BOARD_MARK_MARGIN,
          y as f64 + BOARD_MARK_MARGIN,
        );
        ctx.scale(1.0 - BOARD_MARK_MARGIN * 2.0, 1.0 - BOARD_MARK_MARGIN * 2.0);

        match mark {
          PlayerMark::X => render_player_mark_x(ctx),
          PlayerMark::O => render_player_mark_o(ctx),
        }
        ctx.stroke();

        ctx.restore();
      }
    }
  }
}

fn render_player_mark_x(ctx: &cairo::Context) {
  let a = BOARD_MARK_LINE_WIDTH / f64::consts::SQRT_2 / 2.0;
  ctx.set_source_rgb(239.0 / 255.0, 41.0 / 255.0, 41.0 / 255.0);
  ctx.move_to(a, a);
  ctx.line_to(1.0 - a, 1.0 - a);
  ctx.move_to(a, 1.0 - a);
  ctx.line_to(1.0 - a, a);
}

fn render_player_mark_o(ctx: &cairo::Context) {
  ctx.set_source_rgb(52.0 / 255.0, 101.0 / 255.0, 164.0 / 255.0);
  ctx.arc(
    0.5,
    0.5,
    0.5 - BOARD_MARK_LINE_WIDTH / 2.0,
    0.0,
    2.0 * f64::consts::PI,
  );
}
