mod board;
mod bot;

use gio::prelude::*;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};

const APP_NAME: &str = "Tic-tac-toe: Evolution";
const APP_ID: &str = "com.github.dmitmel.tic-tac-toe-evolution";

fn main() {
  let application =
    Application::new(Some(APP_ID), gio::ApplicationFlags::default())
      .expect("failed to initialize GTK application");

  application.connect_activate(|app| {
    let window = ApplicationWindow::new(app);
    window.set_title(APP_NAME);
    window.set_default_size(800, 600);

    let mut board = board::Board::new(64, 64);

    let button = Button::new_with_label("Click me!");
    button.connect_clicked(|_| {
      println!("Clicked!");
    });
    window.add(&button);

    window.show_all();
  });

  let args: Vec<String> = std::env::args().collect();
  application.run(&args);
}
