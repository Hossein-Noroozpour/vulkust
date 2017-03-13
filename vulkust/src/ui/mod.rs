extern crate gtk;

mod main_window;

use self::gtk::prelude::*;

pub struct UiManager {
    win: main_window::MainWindow,
}

impl UiManager {
    pub fn new() -> Option<UiManager> {
        if gtk::init().is_err() {
            println!("Failed to initialize GTK.");
            return None;
        }
        let window = main_window::MainWindow::new();
        Some(UiManager { win: window })
    }

    pub fn run(self) {
        self.win.show();
        gtk::main();
    }
}
