use std::sync::{ RwLock, Arc };
use std::sync::mpsc::{ Receiver };

use models::Grid;

mod piston;
mod simple_terminal;

pub use self::simple_terminal::SimpleTerminal;

pub trait GridDisplayer {
    fn draw(&self, receiver: Receiver<Arc<RwLock<Grid>>>);
}
