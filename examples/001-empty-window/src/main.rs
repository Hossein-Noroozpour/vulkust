#[macro_use]
extern crate vulkust;

use vulkust::core::application::Application;

fn main() {
    let mut app = Application::new();
    vx_log_i!("Initialised.");
    app.run();
    vx_log_i!("Ended.");
}
