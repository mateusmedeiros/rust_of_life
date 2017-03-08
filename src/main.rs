#![allow(unused_parens)]

extern crate piston_window;
extern crate pancurses;

use std::env;

mod models;
mod utils;
mod grid_displayers;

use grid_displayers::{ GridDisplayer, Pancurses };
use utils::create_dispatcher;
use utils::read_grid_from_file;

fn main() {
    let args: Vec<String> = env::args().collect(); 
    let input = args.get(1).expect("No input file passed as argument.");
    let grid = read_grid_from_file(input).unwrap();
    let receiver = create_dispatcher(grid);
    let displayer = Pancurses::new();

    displayer.draw(receiver);
}
