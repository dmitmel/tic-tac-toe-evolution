mod board;
mod bot;

use std::f64;

use gio::prelude::*;
use gtk::prelude::*;
use gtk::Application;

use crate::board::{Board, PlayerMark};

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
    let program1 = generate_random_program();

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

    let ui_program1: gtk::ListBox = builder.get_object("program1").unwrap();
    for instruction in program1 {
      use crate::bot::instructions::*;
      let instruction_name = match instruction {
        MOVE_UP => "MOVE_UP".to_string(),
        MOVE_UP_RIGHT => "MOVE_UP_RIGHT".to_string(),
        MOVE_RIGHT => "MOVE_RIGHT".to_string(),
        MOVE_DOWN_RIGHT => "MOVE_DOWN_RIGHT".to_string(),
        MOVE_DOWN => "MOVE_DOWN".to_string(),
        MOVE_DOWN_LEFT => "MOVE_DOWN_LEFT".to_string(),
        MOVE_LEFT => "MOVE_LEFT".to_string(),
        MOVE_UP_LEFT => "MOVE_UP_LEFT".to_string(),

        CHECK_MARK => "CHECK_MARK".to_string(),
        PLACE_MARK => "PLACE_MARK".to_string(),
        jump => format!("JUMP({})", jump),
      };

      let list_box_row = gtk::ListBoxRow::new();
      let label = gtk::Label::new(Some(&instruction_name));
      label.set_halign(gtk::Align::Start);
      let mut font_desc = pango::FontDescription::new();
      font_desc.set_family("monospace");
      WidgetExt::override_font(&label, Some(&font_desc));
      list_box_row.add(&label);
      ui_program1.add(&list_box_row);
    }

    let window: gtk::ApplicationWindow = builder.get_object("window2").unwrap();
    window.set_application(Some(app));
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
  use rand::distributions::Standard;
  use rand::Rng;
  let rng = rand::thread_rng();
  rng.sample_iter(Standard).take(10).collect()
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
