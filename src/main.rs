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

    let mut board = board::Board::new(64, 64);

    let player1: gtk::ComboBoxText = builder.get_object("player1").unwrap();
    let player2: gtk::ComboBoxText = builder.get_object("player2").unwrap();
    let play: gtk::Button = builder.get_object("play").unwrap();
    fn update_play_button_state(
      player1: &gtk::ComboBoxText,
      player2: &gtk::ComboBoxText,
      play: &gtk::Button,
    ) {
      play.set_sensitive(
        player1.get_active_id().is_some() && player2.get_active_id().is_some(),
      );
    };
    {
      let player2 = player2.clone();
      let play = play.clone();
      player1.connect_changed(move |player1| {
        update_play_button_state(player1, &player2, &play)
      });
    }
    {
      let player1 = player1.clone();
      let play = play.clone();
      player2.connect_changed(move |player2| {
        update_play_button_state(&player1, player2, &play)
      });
    }
    update_play_button_state(&player1, &player2, &play);

    let window: gtk::ApplicationWindow = builder.get_object("window1").unwrap();
    window.set_application(Some(app));
    window.show_all();
  });

  let args: Vec<String> = std::env::args().collect();
  application.run(&args);
}
