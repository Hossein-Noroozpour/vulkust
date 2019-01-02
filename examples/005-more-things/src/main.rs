#[macro_use]
extern crate vulkust;
extern crate rand;

mod game;
use game::MyGame;

vulkust_start!(MyGame);
