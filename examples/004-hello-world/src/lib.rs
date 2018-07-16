#[macro_use]
extern crate vulkust;

mod game;
use game::MyGame;

vulkust_start!(MyGame);