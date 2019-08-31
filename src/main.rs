mod board;
mod bot;

use gio::prelude::*;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

const APP_NAME: &str = "Tic-tac-toe: Evolution";
const APP_ID: &str = "com.github.dmitmel.tic-tac-toe-evolution";

const PANGO_SCALE_XX_SMALL: f64 = PANGO_SCALE_X_SMALL / 1.2;
const PANGO_SCALE_X_SMALL: f64 = PANGO_SCALE_SMALL / 1.2;
const PANGO_SCALE_SMALL: f64 = PANGO_SCALE_MEDIUM / 1.2;
const PANGO_SCALE_MEDIUM: f64 = 1.0;
const PANGO_SCALE_LARGE: f64 = PANGO_SCALE_MEDIUM * 1.2;
const PANGO_SCALE_X_LARGE: f64 = PANGO_SCALE_LARGE * 1.2;
const PANGO_SCALE_XX_LARGE: f64 = PANGO_SCALE_X_LARGE * 1.2;

fn main() {
  let application =
    Application::new(Some(APP_ID), gio::ApplicationFlags::default())
      .expect("failed to initialize GTK application");

  application.connect_activate(|app| {
    let window = ApplicationWindow::new(app);
    window.set_title(APP_NAME);
    window.set_default_size(800, 600);
    window.set_property_window_position(gtk::WindowPosition::Center);

    let mut board = board::Board::new(64, 64);

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);

    let app_name_label = gtk::Label::new(Some(APP_NAME));
    let app_name_attr_list = pango::AttrList::new();

    {
      use pango::Attribute;
      let attr = Attribute::new_weight(pango::Weight::Bold).unwrap();
      app_name_attr_list.insert(attr);
      let attr = Attribute::new_scale(PANGO_SCALE_XX_LARGE).unwrap();
      app_name_attr_list.insert(attr);
    }
    app_name_label.set_attributes(Some(&app_name_attr_list));

    vbox.pack_start(&app_name_label, false, false, 0);

    window.add(&vbox);

    window.show_all();
  });

  let args: Vec<String> = std::env::args().collect();
  application.run(&args);
}
