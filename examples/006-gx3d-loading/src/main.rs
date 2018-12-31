#[macro_use]
extern crate vulkust;

mod data_gx3d;
mod game;

use game::MyGame;

vulkust_start!(MyGame);
