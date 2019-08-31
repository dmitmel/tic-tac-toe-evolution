mod board;
mod bot;

use gio::prelude::*;
use gtk::prelude::*;
use gtk::Application;

const APP_ID: &str = "com.github.dmitmel.tic-tac-toe-evolution";

fn main() {
  let application =
    Application::new(Some(APP_ID), gio::ApplicationFlags::default())
      .expect("failed to initialize GTK application");

  application.connect_activate(|app| {
    let glade_src = include_str!("main.glade");
    let builder = gtk::Builder::new_from_string(glade_src);

    let mut board = board::Board::new(10, 10);

    {
      use rand::distributions::{Bernoulli, Distribution};

      let mut rng = rand::thread_rng();
      let d = Bernoulli::new(0.5).unwrap();
      for y in 0..board.width() {
        for x in 0..board.height() {
          board.set(
            x,
            y,
            if d.sample(&mut rng) {
              Some(if d.sample(&mut rng) {
                board::PlayerMark::X
              } else {
                board::PlayerMark::O
              })
            } else {
              None
            },
          )
        }
      }
    }

    const BOARD_CELL_SIZE: i32 = 32;
    let ui_board: gtk::DrawingArea = builder.get_object("board").unwrap();
    ui_board.set_size_request(
      board.width() as i32 * BOARD_CELL_SIZE,
      board.height() as i32 * BOARD_CELL_SIZE,
    );
    ui_board.connect_draw(move |ui_board, ctx: &cairo::Context| {
      let style_ctx = ui_board.get_style_context();
      gtk::render_background(
        &style_ctx,
        ctx,
        0.0,
        0.0,
        f64::from(ui_board.get_allocated_width()),
        f64::from(ui_board.get_allocated_height()),
      );

      let line_width = 0.25;
      let padding = 0.1;
      ctx.set_line_width(line_width);
      for y in 0..board.width() {
        for x in 0..board.height() {
          if let Some(mark) = board.get(x, y) {
            use crate::board::PlayerMark;
            let size = f64::from(BOARD_CELL_SIZE);
            ctx.save();
            ctx.translate(x as f64 * size, y as f64 * size);
            ctx.scale(size, size);
            ctx.translate(padding, padding);
            ctx.scale(1.0 - padding, 1.0 - padding);
            match mark {
              PlayerMark::X => {
                let a = line_width / std::f64::consts::SQRT_2 / 2.0;
                ctx.set_source_rgb(239.0 / 255.0, 41.0 / 255.0, 41.0 / 255.0);
                ctx.move_to(a, a);
                ctx.line_to(1.0 - a, 1.0 - a);
                ctx.move_to(a, 1.0 - a);
                ctx.line_to(1.0 - a, a);
              }
              PlayerMark::O => {
                ctx.set_source_rgb(52.0 / 255.0, 101.0 / 255.0, 164.0 / 255.0);
                ctx.arc(
                  0.5,
                  0.5,
                  0.5 - line_width / 2.0,
                  0.0,
                  2.0 * std::f64::consts::PI,
                );
              }
            }
            ctx.stroke();
            ctx.restore();
          }
        }
      }

      Inhibit(false)
    });

    let window: gtk::ApplicationWindow = builder.get_object("window2").unwrap();
    window.set_application(Some(app));
    window.show_all();
  });

  let args: Vec<String> = std::env::args().collect();
  application.run(&args);
}
