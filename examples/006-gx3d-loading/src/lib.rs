#![feature(duration_as_u128)]
#[macro_use]
extern crate vulkust;

mod data_gx3d;
mod game;

use game::MyGame;

vulkust_start!(MyGame);
