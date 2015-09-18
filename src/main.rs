#![feature(associated_consts)]

mod game;

use game::*;

fn main() {
    println!("Hello, world!");

    let mut game = Game::new(Configuration::clean_8());
}
